pub mod toast;

pub use toast::*;

#[derive(PartialEq, Clone, Debug)]
pub enum Tab {
    Admin,
    Lightning,
    Mint,
    Onchain,
}

pub struct Point {
    pub x: f64,
    pub y: f64,
}
