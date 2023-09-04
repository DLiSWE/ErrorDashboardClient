mod app;
mod components;
mod views;
mod routes;
mod hooks;
mod validations;
mod dtos;
mod globals;

use crate::app::App;

pub fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();   
}