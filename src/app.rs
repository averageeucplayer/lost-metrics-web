use futures::StreamExt;
use serde::Deserialize;
use web_sys::console;
use yew::*;
use yew_router::{switch, BrowserRouter, HashRouter, Switch};
use tauri_sys::event;
use wasm_bindgen_futures::spawn_local;

use crate::{components::{AppStateContext, AppStateProvider}, layout::Layout, routes::*};

#[function_component(App)]
pub fn app() -> Html {

    html! {
        <AppStateProvider>
            <Layout/>
        </AppStateProvider>
    }
}