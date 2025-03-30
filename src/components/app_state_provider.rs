use std::rc::Rc;

use futures::StreamExt;
use gloo::timers::future::TimeoutFuture;
use log::*;
use tauri_sys::event;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::spawn_local;
use yew::{html::ImplicitClone, prelude::*};

use crate::models::{LoadResult, Settings};

pub enum AppEvent {
    Load(LoadResult),
    ProcessState(String),
    UpdateSettings(Settings)
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct AppState {
    pub is_loading: bool,
    pub process_state: String,
    pub app_name: String,
    pub version: String,
    pub settings: Settings
}

impl AppState {
    pub fn new() -> Self {
        Self { 
            is_loading: true,
            process_state: "".into(),
            app_name: "".into(),
            version: "0.0.0".into(),
            settings: Settings::default()
        }
    }
}

impl Reducible for AppState {
    type Action = AppEvent;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            AppEvent::Load(result) => {
                Self {
                    is_loading: false,
                    process_state: self.process_state.clone(),
                    app_name: result.app_name,
                    version: result.version,
                    settings: result.settings
                }.into()
            },
            AppEvent::UpdateSettings(settings) => {
                let mut state = self.as_ref().clone();
                state.settings = settings;

                state.into()
            }
            AppEvent::ProcessState(process_state) => {
                let mut state = self.as_ref().clone();
                state.process_state = process_state;

                state.into()
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
        use_effect_with((), move |_| {
            spawn_local(async move {
                
                let load_result = tauri_sys::core::invoke::<LoadResult>("load", ()).await;
                info!("{:?}", load_result);
                
                reducer.dispatch(AppEvent::Load(load_result));
                
                let mut events = event::listen::<String>("app-state").await.unwrap();
                while let Some(event) = events.next().await {
                    info!("{:?}", event.payload);
                    reducer.dispatch(AppEvent::ProcessState(event.payload));
                }  
            });    
        });
    }

    html! {
        <ContextProvider<AppStateContext> context={reducer}>
            {props.children.clone()}
        </ContextProvider<AppStateContext>>
    }
}