use crate::components::base::Button::Button;
use yew::prelude::*;
// use crate::components::composite::navbar::subcomponents::navbutton::style::{
//     navbutton,
//     dropdown_navbutton,
//     menu_button as menu_button_style,
// };
use yew_icons::{Icon, IconId};

pub fn menu_button(menu_button_style: &str, toggle_dropdown: Callback<MouseEvent>) -> Html {
    html! {
        <Button title={"Menu"} class={menu_button_style.to_string()} onclick={toggle_dropdown.clone()}>
            <Icon icon_id={IconId::LucideMenu}/>
        </Button>
    }
}

pub fn home_button(navbutton: &str, home_onclick_redirect: Callback<MouseEvent>) -> Html {
    html! {
        <Button title={"Home"} class={navbutton.to_string()} onclick={home_onclick_redirect.clone()}>
            <Icon icon_id={IconId::HeroiconsSolidHome}/>
        </Button>
    }
}

pub fn login_button(navbutton: &str, login_onclick: Callback<MouseEvent>) -> Html {
    html! {
        <Button title={"Login"} class={navbutton.to_string()} onclick={login_onclick.clone()}>
            <Icon icon_id={IconId::BootstrapDoorOpenFill}/>
        </Button>
    }
}

pub fn logout_button(navbutton: &str, home_onclick: Callback<MouseEvent>) -> Html {
    html! {
        <Button title={"Logout"} class={navbutton.to_string()} onclick={home_onclick.clone()} >
            <Icon icon_id={IconId::BootstrapDoorClosedFill}/>
        </Button>
    }
}

pub fn registration_button(navbutton: &str, registration_onclick: Callback<MouseEvent>) -> Html {
    html! {
        <Button title={"Register"} class={navbutton.to_string()} onclick={registration_onclick.clone()}>
            <Icon icon_id={IconId::HeroiconsSolidClipboardDocumentCheck}/>
        </Button>
    }
}

pub fn dashboard_button(navbutton: &str, dashboard_onclick: Callback<MouseEvent>) -> Html {
    html! {
        <Button title={"Dashboard"} class={navbutton.to_string()} onclick={dashboard_onclick.clone()}>
            <Icon icon_id={IconId::LucideLayoutDashboard}/>
        </Button>
    }
}
