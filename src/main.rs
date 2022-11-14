use yew::{html, Component, Context, Html};
use gloo::console::{self};

use std::rc::Rc;

mod editor;
use editor::types::*;
use editor::help as H;
use editor::key_listener;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    pub fn save(content: &str, file_name: &str);
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let h = Stmt::Hole;
        let t = Program(vec![Rnew(h)]);
        Self { 
            page: Page::Main,
            program: Rnew(t), 
            selected: None,
            hovered: None,
            clipboard: None,
            keys: Vec::new(),
            suggestions: Vec::new(),
            error: Some(ErrorMsg::Intro)
        }
    }

    fn update(&mut self, _: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Unselect => {
                self.selected = None;
                true
            }
            Msg::Select(s) => {
                self.selected = Some(s);
                true
            }
            Msg::Unhover => {
                self.hovered = None;
                true
            }
            Msg::Hover(s) => {
                self.hovered = Some(s);
                true
            }
            Msg::KeyboardMsg(e) => { 
                let key_code = e.key_code();
                console::info!(key_code);
                console::info!(e.key());
                
                self.error = None;
                self.suggestions = Vec::new();
                let len = self.keys.len();
                if Cmd::cmd_ended(&self.keys) {
                    self.keys = Vec::new();
                }

                match self.keys.first() {
                    Some(Cmd::Insert)
                        if (97..=122).contains(&key_code) /* a-z */ || key_code ==  8 /* backspace */ => {
                        if let Some(Cmd::Text(ref mut s)) = self.keys.last_mut() {
                            if key_code == 8 {
                                if s.len() > 1 {
                                    s.pop();
                                } else {
                                    self.keys = vec![Cmd::Insert];
                                }
                            } else {
                                s.push_str(&e.key());
                            }
                        } else {
                            if key_code != 8 {
                                self.keys.push(Cmd::Text(e.key()));
                            }
                        }
                    }
                    _ => {
                        let mut keys = self.keys.clone();
                        match keys.first() {
                            Some(Cmd::Number(_)) => { keys.remove(0); }
                            _ => {}
                        }
                        match key_code {
                            27 /* escape */ => {
                                self.keys = Vec::new();
                                self.suggestions = Vec::new();
                                self.page = Page::Main;
                            }
                            9 /* tab */ => {
                                self.suggestions = App::complete_command(keys);
                            }
                            13 /* enter */ => {
                                self.keys.push(Cmd::Enter);
                            }
                            d if (48..=57).contains(&d) /* 0-9 */ => {
                                let m = (d - 48) as u8;
                                if let Some(Cmd::Number(n)) = self.keys.last_mut() {
                                    if *n < 20 {
                                        *n *= 10;
                                        *n += m;
                                    } else {
                                        self.error = Some(ErrorMsg::NumberTooBig); 
                                    }
                                } else {
                                    self.keys.push(Cmd::Number(m));
                                }
                                return true;
                            }
                            _ => {
                                let options = App::complete_command(keys);
                                for cmd in options {
                                    match cmd {
                                        Cmd::Number(_) => { continue; }
                                        Cmd::Text(_) => { continue; }
                                        _ => {}
                                    }
                                    let c = cmd.to_string().to_lowercase().chars().next().unwrap() as u32;
                                    if key_code == c {
                                        self.keys.push(cmd);
                                    }
                                }
                            }
                        }
                    }
                }
                match &self.selected {
                    Some(u) if (37..=40).contains(&key_code) => {
                        match key_code {
                            37 /* left arrow */ => {
                                let next = self.prev_hole(u.clone());
                                match next {
                                    Some(s) => { self.selected = Some(s); }
                                    _ => {}
                                }
                            }
                            38 /* up arrow */ => {}
                            39 /* right arrow */ => {
                                let next = self.next_hole(u.clone());
                                match next {
                                    Some(s) => { self.selected = Some(s); }
                                    _ => {}
                                }
                            }
                            40 /* down arrow */ => {}
                            _ => {}
                        }
                    }
                    _ => {}
                }
                if self.keys.len() != len {
                    self.run_command();
                }
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link().clone();
        let holes = self.holes();
        html! { 
            <>
                <key_listener::KeyListenerComponent link={link} />
                <div id="wrapper" 
                    onclick={ctx.link().callback(|_| Msg::Unselect)}  
                    onmouseover={ctx.link().callback(|_| Msg::Unhover)}>
                    { Program::view(self.program.clone(), &holes, self, ctx) }
                </div>
                if self.page == Page::Help {
                    { H::help() }
                } 
                <div id="header">
                    <span class="kbdsequence left">
                        { for self.keys.iter().map(|k| k.clone().view()) }
                            <span class="suggestions kbdsequence">
                            { for self.suggestions.iter().map(|k| k.clone().view()) }
                            </span>
                    </span>
                    <span class="right">
                        { self.status() }
                    </span>
                </div>
                <div id="error">
                    if let Some(e) = &self.error { { e.view() } }
                </div>
            </>
        }
    }
}

fn main() {
    yew::set_event_bubbling(false);
    yew::start_app::<App>();
}
