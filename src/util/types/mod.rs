pub mod toast;

use multimint::types::InfoResponse;
pub use toast::*;

#[derive(PartialEq, Clone, Debug)]
pub enum Tab {
    Admin,
    Lightning,
    Mint,
    Onchain,
}

#[derive(PartialEq, Clone, Debug)]
pub enum Elevation {
    Root,
    Default,
    Higher,
    Highest,
}
