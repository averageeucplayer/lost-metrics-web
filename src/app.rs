use yew::*;
use yew_router::BrowserRouter;
use crate::{components::{AppStateProvider, EncounterMonitorProvider}, layout::Layout};

#[function_component(App)]
pub fn app() -> Html {

    html! {
        <AppStateProvider>
            <EncounterMonitorProvider>
                <BrowserRouter>
                    <Layout/>                    
                </BrowserRouter>
            </EncounterMonitorProvider>
        </AppStateProvider>
    }
}