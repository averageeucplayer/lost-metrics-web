use tauri_sys::event;
use yew::{platform::spawn_local, *};

use crate::components::AppStateContext;

#[function_component(Home)]
pub fn home() -> Html {
    let app_state = use_context::<AppStateContext>().unwrap();
    let app_name = app_state.app_name.clone();

    html! { <div class="p-4">
        <h1 class="font-roboto text-xl">{app_name}</h1>
        <UpdateChecker/>
    </div> }
}

#[function_component(UpdateChecker)]
pub fn update_checker() -> Html {
    let is_checking = use_state(|| false);

    let onclick = {
        let is_checking = is_checking.clone();
        
        Callback::from(move |_| {
            is_checking.set(true);
            spawn_local(async move {
                if let Err(err) = event::emit::<String>("check-update", &"".to_string()).await {
                    // console::log_1(&format!("Error emitting event: {:?}", err).into());
                }
            });
        })
    };

    html! {
        <button class="btn btn-sm" {onclick} disabled={*is_checking}>
            if *is_checking {
                <span class="loading loading-spinner loading-sm"></span>
                {"Checking"}
            }
            else {
                {"Check for updates"}
            }
        </button>
    }
}