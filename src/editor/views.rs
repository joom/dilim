use crate::*;
use yew::{html, Html};

impl Const {
    pub fn view(self) -> Html {
        match self {
            Const::Bool(b) => {
                if b { html! { {"True"} } } else { html! { {"False"} } }
            }
            Const::Int(i) => {
                html! { {i} }
            }
        }
    }
}

impl Stmt {
    pub fn view(t: R<Self>, holes: &Vec<Selection>, app: &App, ctx: &Context<App>) -> Html {
        match &*(t.borrow()) {
            Stmt::Hole => {
                let mut cls = vec!["hole", "stmt"];
                let is_selected = match &app.selected {
                    Some(Selection::SStmt(u)) => Rc::ptr_eq(&t, &u),
                    _ => false,
                };
                if is_selected {
                    cls.push("selected");
                }
                let is_hovered = match &app.hovered {
                    Some(Selection::SStmt(u)) => Rc::ptr_eq(&t, &u),
                    _ => false,
                };
                if is_hovered {
                    cls.push("hovered");
                }
                let t = t.clone();
                let tc = t.clone();
                let sel = Selection::SStmt(t.clone());
                let i = holes.iter().position(move |u| sel.eq(u)).unwrap();
                html! {
                    <div class={cls.join(" ")}
                         onclick={ctx.link().callback_once(move |_| Msg::Select(Selection::SStmt(tc)))}
                         onmouseover={ctx.link().callback_once(move |_| Msg::Hover(Selection::SStmt(t)))}>
                        <span>{i}</span>
                        { "  " }
                    </div>
                }
            }
            Stmt::Term { term } => {
                let mut cls = vec!["term", "stmt"];
                let is_selected = match &app.selected {
                    Some(Selection::SStmt(u)) => Rc::ptr_eq(&t, &u),
                    _ => false,
                };
                if is_selected {
                    cls.push("selected");
                }
                let is_hovered = match &app.hovered {
                    Some(Selection::SStmt(u)) => Rc::ptr_eq(&t, &u),
                    _ => false,
                };
                if is_hovered {
                    cls.push("hovered");
                }
                let t = t.clone();
                let tc = t.clone();
                let cl = ctx
                    .link()
                    .callback_once(move |_| Msg::Select(Selection::SStmt(t)));
                let cl2 = ctx
                    .link()
                    .callback_once(move |_| Msg::Hover(Selection::SStmt(tc)));
                html! {
                    <div class={cls.join(" ")}
                         onclick={cl} onmouseover={cl2}>
                        { Term::view(term.clone(), holes, app, ctx) }
                    </div>
                }
            }
            Stmt::Defn { v, body } => {
                let mut cls = vec!["defn", "stmt"];
                let is_selected = match &app.selected {
                    Some(Selection::SStmt(u)) => Rc::ptr_eq(&t, &u),
                    _ => false,
                };
                if is_selected {
                    cls.push("selected");
                }
                let is_hovered = match &app.hovered {
                    Some(Selection::SStmt(u)) => Rc::ptr_eq(&t, &u),
                    _ => false,
                };
                if is_hovered {
                    cls.push("hovered");
                }
                let body_outer = vec!["defnBodyOuter"];
                let t = t.clone();
                let tc = t.clone();
                let cl = ctx
                    .link()
                    .callback_once(move |_| Msg::Select(Selection::SStmt(t)));
                let cl2 = ctx
                    .link()
                    .callback_once(move |_| Msg::Hover(Selection::SStmt(tc)));
                html! {
                    <div class={cls.join(" ")}
                         onclick={cl.clone()} onmouseover={cl2.clone()}>
                        <div class="nameOuter" onclick={cl.clone()} onmouseover={cl2.clone()}>
                            { Name::view(v.clone(), holes, app, ctx) }
                        </div>
                        <div class={body_outer.join(" ")} onclick={cl} onmouseover={cl2}>
                            { Term::view(body.clone(), holes, app, ctx) }
                        </div>
                    </div>
                }
            }
            Stmt::While { c, b1 } => {
                let mut cls = vec!["while", "stmt"];
                let is_selected = match &app.selected {
                    Some(Selection::SStmt(u)) => Rc::ptr_eq(&t, &u),
                    _ => false,
                };
                if is_selected {
                    cls.push("selected");
                }
                let is_hovered = match &app.hovered {
                    Some(Selection::SStmt(u)) => Rc::ptr_eq(&t, &u),
                    _ => false,
                };
                if is_hovered {
                    cls.push("hovered");
                }
                let body_outer = vec!["whileBodyOuter"];
                let t = t.clone();
                let tc = t.clone();
                let cl = ctx
                    .link()
                    .callback_once(move |_| Msg::Select(Selection::SStmt(t)));
                let cl2 = ctx
                    .link()
                    .callback_once(move |_| Msg::Hover(Selection::SStmt(tc)));
                html! {
                    <div class={cls.join(" ")}
                         onclick={cl.clone()} onmouseover={cl2.clone()}>
                        <div class="condOuter" onclick={cl.clone()} onmouseover={cl2.clone()}>
                            { Term::view(c.clone(), holes, app, ctx) }
                        </div>
                        <div class={body_outer.join(" ")} onclick={cl} onmouseover={cl2}>
                            { Program::view(b1.clone(), holes, app, ctx) }
                        </div>
                    </div>
                }
            }
            Stmt::If { c, b1 } => {
                let mut cls = vec!["if", "stmt"];
                let is_selected = match &app.selected {
                    Some(Selection::SStmt(u)) => Rc::ptr_eq(&t, &u),
                    _ => false,
                };
                if is_selected {
                    cls.push("selected");
                }
                let is_hovered = match &app.hovered {
                    Some(Selection::SStmt(u)) => Rc::ptr_eq(&t, &u),
                    _ => false,
                };
                if is_hovered {
                    cls.push("hovered");
                }
                let body_outer = vec!["ifBodyOuter"];
                let t = t.clone();
                let tc = t.clone();
                let cl = ctx
                    .link()
                    .callback_once(move |_| Msg::Select(Selection::SStmt(t)));
                let cl2 = ctx
                    .link()
                    .callback_once(move |_| Msg::Hover(Selection::SStmt(tc)));
                html! {
                    <div class={cls.join(" ")}
                         onclick={cl.clone()} onmouseover={cl2.clone()}>
                        <div class="condOuter" onclick={cl.clone()} onmouseover={cl2.clone()}>
                            { Term::view(c.clone(), holes, app, ctx) }
                        </div>
                        <div class={body_outer.join(" ")} onclick={cl} onmouseover={cl2}>
                            { Program::view(b1.clone(), holes, app, ctx) }
                        </div>
                    </div>
                }
            }
            Stmt::IfElse { c, b1, b2 } => {
                let mut cls = vec!["ifelse", "stmt"];
                let is_selected = match &app.selected {
                    Some(Selection::SStmt(u)) => Rc::ptr_eq(&t, &u),
                    _ => false,
                };
                if is_selected {
                    cls.push("selected");
                }
                let is_hovered = match &app.hovered {
                    Some(Selection::SStmt(u)) => Rc::ptr_eq(&t, &u),
                    _ => false,
                };
                if is_hovered {
                    cls.push("hovered");
                }
                let body_outer = vec!["ifBodyOuter"];
                let t = t.clone();
                let tc = t.clone();
                let cl = ctx
                    .link()
                    .callback_once(move |_| Msg::Select(Selection::SStmt(t)));
                let cl2 = ctx
                    .link()
                    .callback_once(move |_| Msg::Hover(Selection::SStmt(tc)));
                html! {
                    <div class={cls.join(" ")}
                         onclick={cl.clone()} onmouseover={cl2.clone()}>
                        <div class="condOuter" onclick={cl.clone()} onmouseover={cl2.clone()}>
                            { Term::view(c.clone(), holes, app, ctx) }
                        </div>
                        <div class={body_outer.join(" ")} onclick={cl.clone()} onmouseover={cl2.clone()}>
                            { Program::view(b1.clone(), holes, app, ctx) }
                        </div>
                        <div class={body_outer.join(" ")} onclick={cl} onmouseover={cl2}>
                            { Program::view(b2.clone(), holes, app, ctx) }
                        </div>
                    </div>
                }
            }
        }
    }
}

impl Program {
    pub fn view(t: R<Self>, holes: &Vec<Selection>, app: &App, ctx: &Context<App>) -> Html {
        match &*(t.borrow()) {
            Program(ps) => {
                let mut cls = vec!["program"];
                let is_selected = match &app.selected {
                    Some(Selection::SProgram(u)) => Rc::ptr_eq(&t, &u),
                    _ => false,
                };
                if is_selected {
                    cls.push("selected");
                }
                let is_hovered = match &app.hovered {
                    Some(Selection::SProgram(u)) => Rc::ptr_eq(&t, &u),
                    _ => false,
                };
                if is_hovered {
                    cls.push("hovered");
                }
                let t = t.clone();
                let tc = t.clone();
                html! {
                    <div class={cls.join(" ")}
                         onclick={ctx.link().callback_once(move |_| Msg::Select(Selection::SProgram(t)))}
                         onmouseover={ctx.link().callback_once(move |_| Msg::Hover(Selection::SProgram(tc)))}>
                        { for ps.iter().map(|k| Stmt::view(k.clone(), holes, app, ctx)) }
                    </div>
                }
            }
        }
    }
}

impl Name {
    pub fn view(t: R<Self>, holes: &Vec<Selection>, app: &App, ctx: &Context<App>) -> Html {
        match &*t.borrow_mut() {
            Name::Hole => {
                let mut cls = vec!["namehole"];
                let is_selected = match &app.selected {
                    Some(Selection::SName(u)) => Rc::ptr_eq(&t, &u),
                    _ => false,
                };
                if is_selected {
                    cls.push("selected");
                }
                let is_hovered = match &app.hovered {
                    Some(Selection::SName(u)) => Rc::ptr_eq(&t, &u),
                    _ => false,
                };
                if is_hovered {
                    cls.push("hovered");
                }
                let t = t.clone();
                let tc = t.clone();
                let sel = Selection::SName(t.clone());
                let i = holes.iter().position(move |u| sel.eq(u)).unwrap();
                html! {
                    <div class={cls.join(" ")}
                         onclick={ctx.link().callback_once(move |_| Msg::Select(Selection::SName(t)))}
                         onmouseover={ctx.link().callback_once(move |_| Msg::Hover(Selection::SName(tc)))}>
                        <span>{i}</span>
                        { "  " }
                    </div>
                }
            }
            Name::Named { name: x } => {
                let mut cls = vec!["name"];
                let is_selected = match &app.selected {
                    Some(Selection::SName(u)) => Rc::ptr_eq(&t, &u),
                    _ => false,
                };
                if is_selected {
                    cls.push("selected");
                }
                let is_hovered = match &app.hovered {
                    Some(Selection::SName(u)) => Rc::ptr_eq(&t, &u),
                    _ => false,
                };
                if is_hovered {
                    cls.push("hovered");
                }
                let t = t.clone();
                let tc = t.clone();
                html! {
                    <div class={cls.join(" ")}
                         onclick={ctx.link().callback_once(move |_| Msg::Select(Selection::SName(t)))}
                         onmouseover={ctx.link().callback_once(move |_| Msg::Hover(Selection::SName(tc)))}>
                        { x }
                    </div>
                }
            }
        }
    }
}

impl Term {
    pub fn view(t: R<Self>, holes: &Vec<Selection>, app: &App, ctx: &Context<App>) -> Html {
        match &*t.borrow_mut() {
            Term::Hole => {
                let mut cls = vec!["hole"];
                let is_selected = match &app.selected {
                    Some(Selection::STerm(u)) => Rc::ptr_eq(&t, &u),
                    _ => false,
                };
                if is_selected {
                    cls.push("selected");
                }
                let is_hovered = match &app.hovered {
                    Some(Selection::STerm(u)) => Rc::ptr_eq(&t, &u),
                    _ => false,
                };
                if is_hovered {
                    cls.push("hovered");
                }
                let t = t.clone();
                let tc = t.clone();
                let sel = Selection::STerm(t.clone());
                let i = holes.iter().position(move |u| sel.eq(u)).unwrap();
                html! {
                    <div class={cls.join(" ")}
                         onclick={ctx.link().callback_once(move |_| Msg::Select(Selection::STerm(t)))}
                         onmouseover={ctx.link().callback_once(move |_| Msg::Hover(Selection::STerm(tc)))}>
                        <span>{i}</span>
                        { "  " }
                    </div>
                }
            }

            Term::Const { c } => {
                let mut cls = vec!["const"];
                let is_selected = match &app.selected {
                    Some(Selection::STerm(u)) => Rc::ptr_eq(&t, &u),
                    _ => false,
                };
                if is_selected {
                    cls.push("selected");
                }
                let is_hovered = match &app.hovered {
                    Some(Selection::STerm(u)) => Rc::ptr_eq(&t, &u),
                    _ => false,
                };
                if is_hovered {
                    cls.push("hovered");
                }
                let t = t.clone();
                let tc = t.clone();
                html! {
                    <div class={cls.join(" ")}
                         onclick={ctx.link().callback_once(move |_| Msg::Select(Selection::STerm(t)))}
                         onmouseover={ctx.link().callback_once(move |_| Msg::Hover(Selection::STerm(tc)))}>
                        { c.clone().view() }
                    </div>
                }
            }

            Term::Var { v } => {
                let mut cls = vec!["var"];
                let is_selected = match &app.selected {
                    Some(Selection::STerm(u)) => Rc::ptr_eq(&t, &u),
                    _ => false,
                };
                if is_selected {
                    cls.push("selected");
                }
                let is_hovered = match &app.hovered {
                    Some(Selection::STerm(u)) => Rc::ptr_eq(&t, &u),
                    _ => false,
                };
                if is_hovered {
                    cls.push("hovered");
                }
                let t = t.clone();
                let tc = t.clone();
                html! {
                    <div class={cls.join(" ")}
                         onclick={ctx.link().callback_once(move |_| Msg::Select(Selection::STerm(t)))}
                         onmouseover={ctx.link().callback_once(move |_| Msg::Hover(Selection::STerm(tc)))}>
                        { Name::view(v.clone(), holes, app, ctx) }
                    </div>
                }
            }

            Term::App { t1, t2 } => {
                let mut cls = vec!["app"];
                let is_selected = match &app.selected {
                    Some(Selection::STerm(u)) => Rc::ptr_eq(&t, &u),
                    _ => false,
                };
                if is_selected {
                    cls.push("selected");
                }
                let is_hovered = match &app.hovered {
                    Some(Selection::STerm(u)) => Rc::ptr_eq(&t, &u),
                    _ => false,
                };
                if is_hovered {
                    cls.push("hovered");
                }
                let t = t.clone();
                let tc = t.clone();
                html! {
                    <div class={cls.join(" ")}
                         onclick={ctx.link().callback_once(move |_| Msg::Select(Selection::STerm(t)))}
                         onmouseover={ctx.link().callback_once(move |_| Msg::Hover(Selection::STerm(tc)))}>
                        { Term::view(t1.clone(), holes, app, ctx) }
                        { Term::view(t2.clone(), holes, app, ctx) }
                    </div>
                }
            }

            Term::Lam { v, body } => {
                let mut cls = vec!["lam"];
                let is_selected = match &app.selected {
                    Some(Selection::STerm(u)) => Rc::ptr_eq(&t, &u),
                    _ => false,
                };
                if is_selected {
                    cls.push("selected");
                }
                let is_hovered = match &app.hovered {
                    Some(Selection::STerm(u)) => Rc::ptr_eq(&t, &u),
                    _ => false,
                };
                if is_hovered {
                    cls.push("hovered");
                }
                let mut body_outer = vec!["bodyOuter"];
                let has_lam_inside = match *(body.borrow()) {
                    Term::Lam { v: _, body: _ } => true,
                    _ => false,
                };
                if !has_lam_inside {
                    body_outer.push("bodyOuterLast");
                }
                let t = t.clone();
                let tc = t.clone();
                let cl = ctx
                    .link()
                    .callback_once(move |_| Msg::Select(Selection::STerm(t)));
                let cl2 = ctx
                    .link()
                    .callback_once(move |_| Msg::Hover(Selection::STerm(tc)));
                html! {
                    <div class={cls.join(" ")}
                         onclick={cl.clone()} onmouseover={cl2.clone()}>
                        <div class="nameOuter" onclick={cl.clone()} onmouseover={cl2.clone()}>
                            { Name::view(v.clone(), holes, app, ctx) }
                        </div>
                        <div class={body_outer.join(" ")} onclick={cl} onmouseover={cl2}>
                            { Term::view(body.clone(), holes, app, ctx) }
                        </div>
                    </div>
                }
            }
        }
    }
}

impl Cmd {
    pub fn view(self) -> Html {
        match self {
            Cmd::Done => {
                html! { <kbd class="done"></kbd> }
            }
            Cmd::Failed => {
                html! { <kbd class="failed"></kbd> }
            }
            Cmd::Text(c) => {
                html! { <kbd class="text">{c}</kbd> }
            }
            Cmd::Number(c) => {
                html! { <kbd class="number">{c}</kbd> }
            }
            _ => {
                html! { <kbd class="usual">{ format!("{:?}", self) }</kbd> }
            }
        }
    }
}

impl Cmds {
    pub fn view(self) -> Html {
        html! {
            <span class="kbdsequence">
                { for self.iter().map(|k| k.clone().view()) }
            </span>
        }
    }
}

impl ErrorMsg {
    pub fn view(&self) -> Html {
        match self {
            ErrorMsg::Intro => {
                html! { 
                    <span class="normal">
                        {"This is "} 
                        <span class="dilim">
                            {"dilim"} 
                        </span>
                        {", a structural editor. Press "}
                        <kbd class="text">{"h"}</kbd>
                        {" for help. Press "}
                        <kbd class="text">{"Esc"}</kbd>
                        {" to dismiss this message."}
                    </span>
                }
            }
            ErrorMsg::Impossible => {
                html! { {"Impossible."} }
            }
            ErrorMsg::NumberTooBig => {
                html! { {"The number is too big."} }
            }
            ErrorMsg::NotANameHole => {
                html! { {"Not a name hole!"} }
            }
            ErrorMsg::NotATermHole => {
                html! { {"Not a term hole!"} }
            }
            ErrorMsg::NotAStmtHole => {
                html! { {"Not a statement hole!"} }
            }
            ErrorMsg::NotAHole => {
                html! { {"Not a hole!"} }
            }
            ErrorMsg::NotAProgramOrStmtHole => {
                html! { {"Not a program or statement hole!"} }
            }
            ErrorMsg::NotAStmt => {
                html! { {"Not a statement!"} }
            }
            ErrorMsg::NotAProgram => {
                html! { {"Not a program!"} }
            }
            ErrorMsg::NotATermStmt => {
                html! { {"Not a term statement!"} }
            }
            ErrorMsg::NothingSelected => {
                html! { {"Nothing is selected!"} }
            }
            ErrorMsg::NothingInClipboard => {
                html! { {"There is nothing in the clipboard!"} }
            }
            ErrorMsg::NotNameInClipboard => {
                html! { {"Not a name in the clipboard!"} }
            }
            ErrorMsg::NotTermInClipboard => {
                html! { {"Not a term in the clipboard!"} }
            }
            ErrorMsg::NotStmtInClipboard => {
                html! { {"Not a statement in the clipboard!"} }
            }
            ErrorMsg::NotProgramInClipboard => {
                html! { {"Not a program in the clipboard!"} }
            }
            ErrorMsg::NothingToInsert => {
                html! { {"There is nothing to insert!"} }
            }
            ErrorMsg::NowhereToGo => {
                html! { {"Nowhere to go!"} }
            }
            ErrorMsg::NoSuchCommand => {
                html! { {"No such command!"} }
            }
        }
    }
}

impl App {
    pub fn status(&self) -> Html {
        let len = self.holes().len();
        html! {
            format!("{} hole{}", len, if len == 1 {""} else {"s"})
        }
    }
}
