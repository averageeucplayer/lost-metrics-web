use yew::*;
use yew_router::Switch;

use crate::{components::AppStateContext, routes::*, side_bar::SideBar, task_bar::Taskbar};

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
                <>
                    <div class="min-h-screen flex flex-col">
                        <div class="flex flex-1">
                            <SideBar/>
                            <main class="flex flex-1">
                                <Switch<Route> render={switch} />
                            </main>
                        </div>
                        <Taskbar/>
                    </div>
                </>
            }
        </>
    }
}
