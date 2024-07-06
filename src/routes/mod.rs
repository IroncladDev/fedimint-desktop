mod home;

use dioxus::prelude::*;
use home::*;

#[derive(Clone, Routable, Debug, PartialEq)]
pub enum Route {
    #[route("/")]
    Home {},
}
