mod app;
mod components;

use crate::app::App;

pub fn main() {
    yew::Renderer::<App>::new().render();   
}