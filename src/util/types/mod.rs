pub mod toast;

pub use toast::*;

#[derive(PartialEq, Clone, Debug)]
pub enum Tab {
    Admin,
    Lightning,
    Mint,
    Onchain,
}
