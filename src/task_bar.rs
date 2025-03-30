use yew::*;
use log::*;
use crate::components::AppStateContext;

#[function_component(Taskbar)]
pub fn task_bar() -> Html {
    let app_state = use_context::<AppStateContext>().unwrap();

    info!("{}", app_state.process_state);
    let process_state = match app_state.process_state.as_str() {
        "ProcessNotRunning" => html! { <NotRunningLabel /> },
        _ => html! { <CheckingLabel /> },
    };

    html! {
        <footer class="bg-gray-800 p-2">
            {process_state}
        </footer>
    }
}

#[function_component(NotRunningLabel)]
pub fn not_running() -> Html {
    html! {
        <div class="flex items-center">
            <div aria-label="error" class="status status-lg status-error"></div>
            <span class="ml-2">{"The game is not running"}</span>
        </div>
    }
}

#[function_component(CheckingLabel)]
pub fn checking() -> Html {
    html! {
        <div class="flex items-center">
            <div class="inline-grid *:[grid-area:1/1]">
                <div class="status status-lg status-info animate-ping"></div>
                <div class="status status-lg status-info"></div>
            </div>
            <span class="ml-2">{"Checking if game is running"}</span>
        </div>
    }
}