use yew::prelude::*;
use yew_router::prelude::*;
use crate::components::navbar::style::{navbar_container, navbar_logo_container, navlinks, navbutton};
use crate::routes::AppRoute;

#[function_component(Navbar)]
pub fn navbar() -> Html {
    // Hooks
    let navigator = use_navigator().unwrap();

    // Styles
    let navbar_container = navbar_container();
    let navlinks: String = navlinks();
    let navbutton: String = navbutton();
    let navbar_logo_container = navbar_logo_container();


    let home_onclick_redirect = {
    
    let navigator = navigator.clone();
        Callback::from(move |_| navigator.push(&AppRoute::HomePage))
    };
    
    let home_button = {
        let onclick = home_onclick_redirect.clone();
        html! {
            <button class={navbutton.clone()} {onclick}>{"Home"}</button>
        }
    };

    let login_button = {
        let navigator = navigator.clone();
        let onclick = Callback::from(move |_| navigator.push(&AppRoute::LoginPage));
        html! {
            <button class={navbutton.clone()} {onclick}>{"Login"}</button>
        }
    };

    let registration_button = {
        let navigator = navigator.clone();
        let onclick = Callback::from(move |_| navigator.push(&AppRoute::RegistrationPage));
        html! {
            <button class={navbutton.clone()} {onclick}>{"Register"}</button>
        }
    };

    let dashboard_button = {
        let navigator = navigator.clone();
        let onclick = Callback::from(move |_| navigator.push(&AppRoute::DashboardPage));
        html! {
            <button class={navbutton.clone()} {onclick}>{"Dashboard"}</button>
        }
    };

    html! {
        <div class={navbar_container}>
            <div class={navbar_logo_container} onclick={home_onclick_redirect.clone()}>
            <img src={"./images/micro64.png"} />
            <h1>{"Error Dashboard"}</h1>
            </div>
            <ul class={navlinks}>
                {home_button}
                {login_button}
                {registration_button}
                {dashboard_button}
            </ul>
        </div>
    }
}
