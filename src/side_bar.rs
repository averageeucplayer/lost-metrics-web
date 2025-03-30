use yew::*;
use yew_router::{hooks::use_route, prelude::Link};

use crate::{components::*, routes::Route};

#[function_component(SideBar)]
pub fn side_bar() -> Html {
    let route: Route = use_route::<Route>().unwrap_or_default();

    html! {
        <aside class="bg-gray-900 p-1">
            <ul class="mt-2 space-y-4">
                <SideBarLink route={Route::Home} current_route={route.clone()}>
                    <HomeIcon/>
                </SideBarLink>
                <SideBarLink route={Route::Simulator} current_route={route.clone()}>
                    <GamepadIcon/>
                </SideBarLink>
                <SideBarLink route={Route::Meter} current_route={route.clone()}>
                    <GaugeIcon/>
                </SideBarLink>
                <SideBarLink route={Route::Settings} current_route={route.clone()}>
                    <SettingsIcon/>
                </SideBarLink>
            </ul>
        </aside>
    }
}


#[derive(Properties, PartialEq)]
struct SideBarLinkProps {
    route: Route,
    current_route: Route,
    #[prop_or_default]
    pub children: Html,
}

#[function_component(SideBarLink)]
fn side_bar_link(props: &SideBarLinkProps) -> Html {
    let is_active = props.route == props.current_route;
    let classes = if is_active {
        "block p-2 text-xl bg-gray-700 rounded-lg"
    } else {
        "block p-2 text-xl hover:bg-gray-700 rounded-lg"
    };

    html! {
        <li>
            <Link<Route> to={props.route.clone()} classes={classes}>
                {props.children.clone()}
            </Link<Route>>
        </li>
    }
}