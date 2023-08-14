use yew::prelude::*;
use yew_router::prelude::*;

use crate::views::home::HomePage;

#[derive(Routable, Debug, Clone, PartialEq)]
pub enum AppRoute {
    #[at("/")]
    HomePage,
}

pub fn switch(routes: AppRoute) -> Html {
    match routes {
        AppRoute::HomePage => html! {<HomePage/>}
    }
}
