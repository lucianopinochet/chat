#![allow(non_snake_case)]

mod components;
mod server;
use dioxus_router::prelude::*;
use dioxus::prelude::*;
use components::*;

#[tokio::main]
async fn main() {
    
  dioxus_desktop::launch(App);
    
}

fn App(cx: Scope) -> Element {
  render!{
    style {include_str!("./style.css")},
    Router::<Route> {},

  }
}

#[derive(Clone, Routable, Debug, PartialEq)]
 pub enum Route {
  #[route("/")]
  // Login{},
  // #[route("/chat")]
  Chat{}
}




