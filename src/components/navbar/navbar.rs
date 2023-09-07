use web_sys::MouseEvent;
use yew::prelude::*;
use yew_router::prelude::*;
use yew_icons::{Icon, IconId};

use crate::routes::AppRoute;
use crate::components::navbar::style::{
    navbar_container,
    navbar_logo_container,
    navlinks,
    menu_container,
    menu_button,
    navbutton,
    dropdown_navbutton,
};


#[function_component(Navbar)]
pub fn navbar() -> Html {
    // Hooks
    let navigator = use_navigator().unwrap();
    let dropdown_handler= use_state(|| false);

    // Styles
    let navbar_container = navbar_container();
    let navlinks: String = navlinks();
    let navbutton: String = navbutton();
    let navbar_logo_container = navbar_logo_container();
    let menu_container = menu_container();
    let menu_button = menu_button();
    let dropdown_navbutton = dropdown_navbutton();

    // Utils
    let toggle_dropdown = {
        let dropdown_visible_setter = dropdown_handler.setter().clone();
        let dropdown_handler_for_toggle = dropdown_handler.clone();
        
        Callback::from(move |_: MouseEvent| {
            let current_state = *dropdown_handler_for_toggle;
            dropdown_visible_setter.set(!current_state);
        })
    };

    let navbutton_class = if *dropdown_handler {
        dropdown_navbutton.clone()
    } else { navbutton.clone() };

    let home_onclick_redirect = {
        let navigator = navigator.clone();
        Callback::from(move |_| navigator.push(&AppRoute::HomePage))
    };
    
    // Components
    let menu_button = {
        html! {
            <button title={"Menu"} class={menu_button.clone()} onclick={toggle_dropdown.clone()}>
                <Icon icon_id={IconId::LucideMenu}/>
            </button>
        }
    };

    let home_button = {
        let onclick = home_onclick_redirect.clone();
        html! {
            <button title={"Home"} class={navbutton_class.clone()} {onclick}>
                <Icon icon_id={IconId::HeroiconsSolidHome}/>
            </button>
        }
    };

    let login_button = {
        let navigator = navigator.clone();
        let onclick = Callback::from(move |_| navigator.push(&AppRoute::LoginPage));
        html! {
            <button title={"Login"} class={navbutton_class.clone()} {onclick}>
                <Icon icon_id={IconId::BootstrapDoorOpenFill}/>
            </button>
        }
    };

    let logout_button = {
        let navigator = navigator.clone();
        html! {
            <button title={"Logout"} class={navbutton_class.clone()}>
                <Icon icon_id={IconId::BootstrapDoorClosedFill}/>
            </button>
        }
    };

    let registration_button = {
        let navigator = navigator.clone();
        let onclick = Callback::from(move |_| navigator.push(&AppRoute::RegistrationPage));
        html! {
            <button title={"Logout"} class={navbutton_class.clone()} {onclick}>
                <Icon icon_id={IconId::HeroiconsSolidClipboardDocumentCheck}/>
            </button>
        }
    };

    let dashboard_button = {
        let navigator = navigator.clone();
        let onclick = Callback::from(move |_| navigator.push(&AppRoute::DashboardPage));
        html! {
            <button class={navbutton_class.clone()} {onclick}>
                <Icon icon_id={IconId::LucideLayoutDashboard}/>
            </button>
        }
    };


    html! {
        <div class={navbar_container}>
            <div class={navbar_logo_container} onclick={home_onclick_redirect.clone()}>
                <img src={"./images/micro64.png"} />
                <h1>{"Error Dashboard"}</h1>
            </div>
            <div class={menu_container}>
                {menu_button}
            </div>
            <ul class={navlinks}>
                {home_button}
                {login_button}
                {logout_button}
                {registration_button}
                {dashboard_button}
            </ul>

        </div>
    }
}
