use yew::{html, Html, function_component};

#[function_component(DashboardPage)]
pub fn dashboard_page()-> Html {
    html! {
        <div>
            <p>{"Dashboard Page"}</p>
        </div>
    }
}
