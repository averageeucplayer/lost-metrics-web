use std::rc::Rc;

use futures::StreamExt;
use gloo::timers::future::TimeoutFuture;
use log::*;
use tauri_sys::event;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::spawn_local;
use yew::{html::ImplicitClone, prelude::*};

use crate::models::Encounter;

pub enum Event {
    Update(Encounter),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct EncounterMonitor {
    
}

impl EncounterMonitor {
    pub fn new() -> Self {
        Self { 
            
        }
    }
}

impl Reducible for EncounterMonitor {
    type Action = Event;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            Event::Update(new) => {
                Self {
                  
                }.into()
            }
            
        }
    }
}

pub type EncounterMonitorContext = UseReducerHandle<EncounterMonitor>;

#[derive(Properties, Debug, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub children: Html,
}

#[function_component(EncounterMonitorProvider)]
pub fn encounter_monitor_provider(props: &Props) -> Html {
    let reducer = use_reducer(|| EncounterMonitor::new());

    {
        let reducer = reducer.clone();
        use_effect_with((), move |_| {
            spawn_local(async move {
                
                
            });    
        });
    }

    html! {
        <ContextProvider<EncounterMonitorContext> context={reducer}>
            {props.children.clone()}
        </ContextProvider<EncounterMonitorContext>>
    }
}