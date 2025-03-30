use log::*;
use meter::MeterTab;
use sniffer::SnifferTab;
use storage::StorageTab;
use updates::UpdatesTab;
use yew::{platform::spawn_local, *};

use crate::{components::{AppEvent, AppStateContext, CrossIcon}, models::{SaveSettings, Settings, SnifferSettings}};

mod tab;
mod sniffer;
mod storage;
mod meter;
mod updates;

pub use tab::*;

#[derive(Clone, PartialEq)]
enum SaveState {
    Idle,
    Changed,
    Saving,
    Saved,
    Error
}

#[function_component(SettingsPage)]
pub fn settings() -> Html {
    let app_state = use_context::<AppStateContext>().unwrap();
    let active_tab = use_state(|| Tab::Sniffer);
    let save_state = use_state(|| SaveState::Idle);
    let switch_tab = {
        let active_tab = active_tab.clone();  
        Callback::from(move |tab: Tab| active_tab.set(tab))
    };
    let original_settings = use_state(|| app_state.settings.clone());
    let settings = use_state(|| app_state.settings.clone());

    let is_save_disabled = match *save_state {
        SaveState::Idle => true,
        SaveState::Changed => false,
        SaveState::Saving => true,
        SaveState::Saved => false,
        SaveState::Error => false
    };

    let on_sniffer_settings_change = {
        let settings = settings.clone();
        let save_state = save_state.clone();

        Callback::from(move |sniffer_settings: SnifferSettings| {
            
            let mut new_settings = (&*settings).clone();
            new_settings.sniffer = sniffer_settings;

            if new_settings == *original_settings {
                save_state.set(SaveState::Idle);
            }
            else {
                save_state.set(SaveState::Changed);
            }

            settings.set(new_settings); 
        })
    };

    let onclick = {
        let save_state = save_state.clone();
        let settings = settings.clone();

        Callback::from(move |_| {
            save_state.set(SaveState::Saving);
            let app_state = app_state.clone();
            let save_state = save_state.clone();
            let settings = settings.clone();

            spawn_local(async move {
                let settings = &*settings;
                let args = SaveSettings {
                    settings: settings.clone()
                };
                match tauri_sys::core::invoke_result::<Settings, String>("save_settings", args).await {
                    Ok(_) => {
                        app_state.dispatch(AppEvent::UpdateSettings(settings.clone()));
                        save_state.set(SaveState::Saved);
                    },
                    Err(_) => {
                        save_state.set(SaveState::Error);
                    },
                }
            });
        })
    };

    html! {
        <div class="p-4">
            <div role="tablist" class="tabs tabs-border">
                <TabButton active={*active_tab == Tab::Sniffer} tab={Tab::Sniffer} on_click={switch_tab.clone()} />
                <TabButton active={*active_tab == Tab::Meter} tab={Tab::Meter} on_click={switch_tab.clone()} />
                <TabButton active={*active_tab == Tab::Updates} tab={Tab::Updates} on_click={switch_tab.clone()} />
                <TabButton active={*active_tab == Tab::Storage} tab={Tab::Storage} on_click={switch_tab.clone()} />
            </div>

            <div class="tab-content block border-base-300 bg-base-100 p-4">
                {
                    match *active_tab {
                        Tab::Sniffer => html! { <SnifferTab settings={settings.sniffer.clone()} on_settings_change={on_sniffer_settings_change} /> },
                        Tab::Meter => html! { <MeterTab /> },
                        Tab::Updates => html! { <UpdatesTab /> },
                        Tab::Storage => html! { <StorageTab /> },
                    }
                }
            </div>
            <div class="mt-4">
                <button class="btn" disabled={is_save_disabled} {onclick}>
                    if *save_state == SaveState::Saving {
                        <span class="loading loading-spinner"></span>
                        <span>{"Saving"}</span>
                    }
                    else {
                        <span>{"Save"}</span>
                    }
                </button>
            </div>
            if *save_state == SaveState::Saved {
                <div role="alert" class="alert alert-success alert-soft">
                    <span>{"Saved settings"}</span>
                </div>
            }
            if *save_state == SaveState::Error {
                <div class="mt-4">
                    <div role="alert" class="alert alert-error alert-soft">
                        <CrossIcon/>
                        <span>{"An error occurred whilst saving settings. Try later"}</span>
                    </div>
                </div>
            }
        </div>
    }
}