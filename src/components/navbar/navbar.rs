use yew::prelude::*;
use yew_router::prelude::*;
use crate::components::navbar::style::{navbar_container, navlinks, navbutton};
use crate::routes::AppRoute;

#[function_component(Navbar)]
pub fn navbar() -> Html {
    // Hooks
    let navigator = use_navigator().unwrap();

    // Styles
    let navbar_container = navbar_container();
    let navlinks: String = navlinks();
    let navbutton: String = navbutton();

    let home_button = {
        let navigator = navigator.clone();
        let onclick = Callback::from(move |_| navigator.push(&AppRoute::HomePage));
        html! {
            <button class={navbutton} {onclick}>{"Home"}</button>
        }

    };

    html! {
        <div class={navbar_container}>
            <img src={"./images/micro64.png"} />
            <h1>{"Error Dashboard"}</h1>
            <ul class={navlinks}>
                {home_button}
                <li>{"Login"}</li>
                <li>{"Register"}</li>
                <li>{"Dashboard"}</li>
            </ul>
        </div>
    }
}
