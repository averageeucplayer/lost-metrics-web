use yew::*;
use yew_router::{BrowserRouter, Switch};

use crate::{components::AppStateContext, routes::*};

#[function_component(Layout)]
pub fn layout() -> Html {
    let app_state = use_context::<AppStateContext>().unwrap();
        let is_loading = app_state.is_loading;

    html! {
        <>
            if is_loading {
                <div class="loader-wrapper">
                    <div class="loader"></div>
                </div>
            } else {
                <BrowserRouter>
                    <Switch<Route> render={switch} />
                </BrowserRouter>
            }
        </>
    }
}
