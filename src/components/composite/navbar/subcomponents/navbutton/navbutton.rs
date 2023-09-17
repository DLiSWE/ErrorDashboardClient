use yew::prelude::*;
use crate::components::base::Button::Button;

pub fn menu_button(navbutton_class: &str, home_onclick: Callback<_>) -> Html {
    let class = menu_button.clone();
    html! {
        <Button title={"Menu"} class={class} onclick={toggle_dropdown.clone()}>
            <Icon icon_id={IconId::LucideMenu}/>
        </Button>
    }
}

pub fn home_button(navbutton_class: &str, home_onclick: Callback<_>) -> Html {
    html! {
        <Button title={"Home"} class={navbutton_class} onclick={home_onclick}>
            <Icon icon_id={IconId::HeroiconsSolidHome}/>
        </Button>
    }
}


pub fn login_button(navbutton_class: &str, home_onclick: Callback<_>) {
    let navigator = navigator.clone();
    let onclick = Callback::from(move |_| navigator.push(&AppRoute::LoginPage));

    let class = Some(navbutton_class.clone());

    html! {
        <Button title={"Login"} class={navbutton_class.clone()} onclick={onclick}>
            <Icon icon_id={IconId::BootstrapDoorOpenFill}/>
        </Button>
    }
}

pub fn logout_button(navbutton_class: &str, home_onclick: Callback<_>) {
    let _navigator = navigator.clone();
    html! {
        <Button title={"Logout"} >
            <Icon icon_id={IconId::BootstrapDoorClosedFill} onclick={}/>
        </Button>
    }
}

pub fn registration_button(navbutton_class: &str, home_onclick: Callback<_>){
    let navigator = navigator.clone();
    let onclick = Callback::from(move |_| navigator.push(&AppRoute::RegistrationPage));

    let class = Some(navbutton_class.clone());
    
    html! {
        <Button title={"Register"} class={class} onclick={onclick}>
            <Icon icon_id={IconId::HeroiconsSolidClipboardDocumentCheck}/>
        </Button>
    }
}

pub fn dashboard_button(navbutton_class: &str, home_onclick: Callback<_>){
    let navigator = navigator.clone();
    let onclick = Callback::from(move |_| navigator.push(&AppRoute::DashboardPage));

    let class = Some(navbutton_class.clone());

    html! {
        <Button title={"Dashboard"} class={class} onclick={onclick}>
            <Icon icon_id={IconId::LucideLayoutDashboard}/>
        </Button>
    }
}
