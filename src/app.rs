use yew::prelude::*;

use crate::components::navbar::navbar::Navbar;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <div>
        <Navbar/>
        <h1>{"Hello World"}</h1>
        </div>
    }
}

pub fn render() {
    yew::Renderer::<App>::new().render();
}
