#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_web::launch;

mod app;
mod components;
mod pages;
mod hooks;
mod context;
mod api;
mod services;
mod types;
mod utils;
mod constants;
mod config;

use app::App;

fn main() {
    launch(App);
}
