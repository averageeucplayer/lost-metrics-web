use std::fmt;
use yew::*;

#[derive(Debug, Clone, PartialEq)]
pub enum Tab {
    Sniffer,
    Meter,
    Updates,
    Storage,
}

impl fmt::Display for Tab {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Tab::Sniffer => "Sniffer",
                Tab::Meter => "Meter",
                Tab::Updates => "Updates",
                Tab::Storage => "Storage",
            }
        )
    }
}

#[derive(Properties, PartialEq)]
pub struct TabProps {
    pub active: bool,
    pub tab: Tab,
    pub on_click: Callback<Tab>,
}

#[function_component(TabButton)]
pub fn tab_button(props: &TabProps) -> Html {
    let class = if props.active { "tab tab-active" } else { "tab" };

    let onclick = {
        let on_click = props.on_click.clone();
        let tab = props.tab.clone();
        Callback::from(move |_| on_click.emit(tab.clone()))
    };

    html! {
        <a role="tab" {class} {onclick}>{ props.tab.to_string() }</a>
    }
}