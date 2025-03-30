use web_sys::HtmlInputElement;
use yew::*;
use log::*;

use crate::models::{Settings, SnifferSettings};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub settings: SnifferSettings,
    pub on_settings_change: Callback<SnifferSettings>,
}

#[function_component(SnifferTab)]
pub fn sniffer_tab(props: &Props) -> Html {

    let update_process_name = {
        let on_settings_change = props.on_settings_change.clone();
        let new_settings = props.settings.clone();

        Callback::from(move |event: InputEvent| {
            let input: HtmlInputElement = event.target_unchecked_into();
            let mut new_settings = new_settings.clone();
            new_settings.process_name = input.value();
            on_settings_change.emit(new_settings);
        })
    };

    let update_port = {
        let on_settings_change = props.on_settings_change.clone();
        let new_settings = props.settings.clone();

        Callback::from(move |event: InputEvent| {
            let input: HtmlInputElement = event.target_unchecked_into();
            let mut new_settings = new_settings.clone();
            new_settings.port = input.value().parse().unwrap_or(0);
            on_settings_change.emit(new_settings);
        })
    };

    html! {
        <div>
            <label class="input input-sm">
                <span class="label">{"Process"}</span>
                <input type="search"
                    placeholder="Find process"
                    value={props.settings.process_name.clone()}
                    oninput={update_process_name} />
            </label>
            <label class="input input-sm mt-2">
                <span class="label">{"Port"}</span>
                <input type="number"
                    placeholder="Enter port to listen"
                    value={props.settings.port.to_string()}
                    oninput={update_port}
                    />
            </label>
        </div>
    }
}