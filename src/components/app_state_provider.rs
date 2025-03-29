use std::rc::Rc;

use futures::StreamExt;
use tauri_sys::event;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use crate::models::{LoadResult, Settings};

pub enum Event {
    Load(LoadResult)
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct AppState {
    pub is_loading: bool,
    pub version: String,
    pub settings: Settings
}

impl AppState {
    pub fn new() -> Self {
        Self { 
            is_loading: true,
            version: "0.0.0".into(),
            settings: Settings::default()
        }
    }
}

impl Reducible for AppState {
    type Action = Event;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            Event::Load(result) => {
                Self {
                    is_loading: true,
                    version: result.version,
                    settings: result.settings
                }.into()
            }
        }
    }
}

pub type AppStateContext = UseReducerHandle<AppState>;

#[derive(Properties, Debug, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub children: Html,
}

#[function_component(AppStateProvider)]
pub fn app_state_provider(props: &Props) -> Html {
    let reducer = use_reducer(|| AppState::new());

    {
        let reducer = reducer.clone();
        spawn_local(async move {
    
            let load_result = tauri_sys::core::invoke::<LoadResult>("load", ()).await;
            
            reducer.dispatch(Event::Load(load_result));
            
            let mut events = event::listen::<String>("app-state").await.unwrap();
            while let Some(event) = events.next().await {
                // reducer.dispatch(Event::Load(load_result));
            }  
        });
    }

    html! {
        <ContextProvider<AppStateContext> context={reducer}>
            {props.children.clone()}
        </ContextProvider<AppStateContext>>
    }
}