use yew::html;
use yew::events::KeyboardEvent;
use yew_hooks::prelude::*;
use yew::{function_component, Properties};

use crate::editor::types::*;

#[derive(Properties, Clone)]
pub struct ScopeProps { 
    pub link: yew::html::Scope<App>
}

impl PartialEq for ScopeProps {
    fn eq(&self, _: &Self) -> bool { true }
}

#[function_component(KeyListenerComponent)]
pub fn key_listener_component(props: &ScopeProps) -> Html {
    let propsc = props.clone();
    use_event_with_window("keypress", move |e: KeyboardEvent| {
        propsc.link.send_message(Msg::KeyboardMsg(e));
    });
    let propscc = props.clone();
    use_event_with_window("keydown", move |e: KeyboardEvent| {
        match e.key_code() {
            9 /* tab */ |
            8 /* backspace */ | 46 /* delete */ |
            37..=40 /* arrows */ | 27 /* escape */ => {
                e.prevent_default();
                propscc.link.send_message(Msg::KeyboardMsg(e));
            }        
            _ => {}
        }        
    });
    html! { <> </> }
}
