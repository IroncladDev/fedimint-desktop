use anyhow::bail;
use anyhow::Context;
use multimint::{fedimint_core::Amount, fedimint_ln_common::lightning_invoice::Bolt11Invoice};
use serde::Deserialize;
use std::str::FromStr;
use tracing::debug;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LnPayRequest {
    pub payment_info: String,
    pub amount_msat: Option<Amount>,
    pub lnurl_comment: Option<String>,
}

pub async fn get_invoice(req: &LnPayRequest) -> anyhow::Result<Bolt11Invoice> {
    let info = req.payment_info.trim();
    match Bolt11Invoice::from_str(info) {
        Ok(invoice) => {
            match (invoice.amount_milli_satoshis(), req.amount_msat) {
                (Some(_), Some(_)) => {
                    bail!("Amount specified in both invoice and command line")
                }
                (None, _) => {
                    bail!("We don't support invoices without an amount")
                }
                _ => {}
            };
            Ok(invoice)
        }
        Err(e) => {
            let lnurl = if info.to_lowercase().starts_with("lnurl") {
                lnurl::lnurl::LnUrl::from_str(info)?
            } else if info.contains('@') {
                lnurl::lightning_address::LightningAddress::from_str(info)?.lnurl()
            } else {
                bail!("Invalid invoice or lnurl: {e:?}");
            };
            debug!("Parsed parameter as lnurl: {lnurl:?}");
            let amount = req
                .amount_msat
                .context("When using a lnurl, an amount must be specified")?;
            let async_client = lnurl::AsyncClient::from_client(reqwest::Client::new());
            let response = async_client.make_request(&lnurl.url).await?;
            match response {
                lnurl::LnUrlResponse::LnUrlPayResponse(response) => {
                    let invoice = async_client
                        .get_invoice(&response, amount.msats, None, req.lnurl_comment.as_deref())
                        .await?;
                    let invoice = Bolt11Invoice::from_str(invoice.invoice())?;
                    assert_eq!(invoice.amount_milli_satoshis(), Some(amount.msats));
                    Ok(invoice)
                }
                other => {
                    bail!("Unexpected response from lnurl: {other:?}");
                }
            }
        }
    }
}
