use yew::{function_component, html, Html};

#[function_component(Navbar)]
pub fn navbar() -> Html {
    html! {
        <div>
            <h1>{"Navbar"}</h1>
        </div>
    }
}