use yew::{html, Html};
use yew_router::Routable;
use crate::pages::*;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/meter")]
    Meter,
    #[at("/simulator")]
    Simulator,
    #[at("/settings")]
    Settings,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Home/> },
        Route::Settings => html! { <SettingsPage/> },
        Route::Meter => html! { <Meter/> },
        Route::Simulator => html! { <Simulator/> },
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}