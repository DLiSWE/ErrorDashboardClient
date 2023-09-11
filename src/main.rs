mod app;
mod components;
mod views;
mod routes;
mod hooks;
mod validations;
mod globals;
mod configs;

use crate::app::App;

pub fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();   
}