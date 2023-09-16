use yew::prelude::*;
use yew_router::prelude::*;
use web_sys::MouseEvent;

#[derive(Properties, Clone, PartialEq)]
pub struct ButtonProps {
    pub label: String,
    pub on_click: Callback<MouseEvent>,
    pub style: String,
    pub children: Html,
}

#[function_component(Button)]
pub fn button(props: &ButtonProps) -> Html {
    // Styles

    // Utils


    html! {

    }
}
