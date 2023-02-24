use yew_router::prelude::*;
use yew::prelude::*;
use crate::pages::{HomePage,LoginPage};

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    // #[at("/Login")]
    // Login,
}

pub fn switch(route: Route) -> Html {
    match route {
        Route::Home => html!(<HomePage />),
        // Route::Login => html!(<LoginPage/>),
    }
}