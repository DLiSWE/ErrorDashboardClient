use yew::prelude::*;
use yew_router::prelude::*;
use yew_router::router::BrowserRouter;
use crate::components::navbar::navbar::Navbar;
use crate::routes::{AppRoute, switch};

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <div>
            <BrowserRouter>
                <Navbar/>
                <Switch<AppRoute> render={switch} />
            </BrowserRouter>
        </div>
    }
}
