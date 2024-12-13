#![allow(non_snake_case)]

use dioxus::prelude::*;
use tracing::info;

mod app;
mod data;
mod routes;
mod components;
mod i18n;
mod utils;
mod pages;
mod models;
mod theme;

fn main() {
    info!("starting app");
    launch(app::App);
}
