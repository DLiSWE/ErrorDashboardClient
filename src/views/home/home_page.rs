use yew::{html, Html, function_component};

#[function_component(HomePage)]
pub fn home_page()-> Html {
    html! {
        <div>
            <p>{"Error Dashboard built with Web Assembly in Rust"}</p>
        </div>
    }
}