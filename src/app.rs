use futures::StreamExt;
use serde::Deserialize;
use web_sys::console;
use yew::*;
use yew_router::{HashRouter, Switch};
use tauri_sys::event;
use wasm_bindgen_futures::spawn_local;

// TO-DO lost-metrics-core-web?
#[derive(Debug, Default, Clone, Deserialize)]
pub struct Encounter {
    pub id: String,
    pub updated_on: String,
    pub total_damage: i64
}

#[function_component(App)]
pub fn app() -> Html {
    let app_state = use_state(|| Vec::<String>::new());
    let encounter_state = use_state(|| None::<Encounter>);

    {
        let app_state = app_state.clone();
        spawn_local(async move {
            let mut events = event::listen::<String>("app-state").await.unwrap();
    
            while let Some(event) = events.next().await {
                app_state.set({
                    let mut new_state = (*app_state).clone();
                    new_state.push(event.payload.clone());
                    new_state
                });
            }  
        });

        let encounter_state = encounter_state.clone();
        spawn_local(async move {
            let mut events = event::listen::<Encounter>("encounter-update").await.unwrap();
    
            while let Some(event) = events.next().await {
                encounter_state.set(Some(event.payload));
            }  
        });
    }

    let emit_event = {
        Callback::from(move |_| {
            spawn_local(async move {
                if let Err(err) = event::emit::<String>("check-update", &"".to_string()).await {
                    console::log_1(&format!("Error emitting event: {:?}", err).into());
                }
            });
        })
    };

    html! {
        <div>
            <button class="px-6 py-2 bg-blue-500 cursor-pointer text-white rounded-lg hover:bg-blue-600 focus:outline-none focus:ring-2 focus:ring-blue-400 transition duration-200" onclick={emit_event}>{"Check for update"}</button>
            <h3>{"App State Messages:"}</h3>
            <ul>
                { for app_state.iter().map(|msg| html! { <li>{msg}</li> }) }
            </ul>

            <h3>{"Encounter:"}</h3>
            {
                if let Some(encounter) = (*encounter_state).clone() {
                    html! {
                        <div>
                            <p><strong>{"ID: "}</strong>{encounter.id}</p>
                            <p><strong>{"Updated On: "}</strong>{encounter.updated_on}</p>
                            <p><strong>{"Total Damage: "}</strong>{encounter.total_damage}</p>
                        </div>
                    }
                } else {
                    html! { <p>{"No encounter data"}</p> }
                }
            }
        </div>
    }
}
