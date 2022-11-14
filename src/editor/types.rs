use std::cell::RefCell;
use std::rc::Rc;
use strum_macros::Display;
use yew::events::KeyboardEvent;

pub type R<T> = Rc<RefCell<T>>;

#[inline(always)]
pub fn Rnew<T>(x: T) -> R<T> {
    Rc::new(RefCell::new(x))
}

#[derive(Clone)]
pub enum Name {
    Hole,
    Named { name: String },
}

#[derive(Clone)]
pub enum Const {
    Bool(bool),
    Int(u32),
}

#[derive(Clone)]
pub enum Term {
    Hole,
    Var { v: R<Name> },
    App { t1: R<Term>, t2: R<Term> },
    Lam { v: R<Name>, body: R<Term> },
    Const { c: Const },
}

#[derive(Clone)]
pub enum Stmt {
    Hole,
    Term { term: R<Term> },
    Defn { v: R<Name>, body: R<Term> },
    While { c: R<Term>, b1: R<Program> },
    If { c: R<Term>, b1: R<Program> },
    IfElse { c: R<Term>, b1: R<Program>, b2: R<Program> }
}

#[derive(Clone)]
pub struct Program(pub Vec<R<Stmt>>);

#[derive(Clone)]
pub enum Selection {
    STerm(R<Term>),
    SName(R<Name>),
    SStmt(R<Stmt>),
    SProgram(R<Program>),
}

pub enum Msg {
    KeyboardMsg(KeyboardEvent),
    Unselect,
    Select(Selection),
    Unhover,
    Hover(Selection),
}

#[derive(Debug, Display, Clone)]
pub enum Cmd {
    // Main command
    Help,   // h
    Add,    // a
    Delete, // d
    Wrap,   // w
    Go,     // g
    Insert, // i
    Yank,   // y
    Paste,  // p
    // Input
    Term,      // t
    Defn,      // d
    If,        // i
    Else,      // e
    While,     // w
    Var,       // v
    Constant,  // c
    Lam,       // l
    App,       // a
    Hole,      // h
    Fresh,     // f
    Sibling,   // s
    Statement, // s
    Text(String),
    Number(u8),
    True,  // t
    False, // f
    // Location or direction
    Prev,     // p
    Next,     // n
    Inwards,  // i
    Outwards, // o
    // Final: special command added once the command is run
    Enter,
    Done,
    Failed,
}

#[derive(Debug, Clone)]
pub struct Cmds(pub Vec<Cmd>);

#[derive(Debug, Clone)]
pub enum ErrorMsg {
    Intro,
    Impossible,
    NumberTooBig,
    NotANameHole,
    NotATermHole,
    NotAStmtHole,
    NotAHole,
    NotAProgramOrStmtHole,
    NotAStmt,
    NotAProgram,
    NotATermStmt,
    NothingSelected,
    NothingInClipboard,
    NotNameInClipboard,
    NotTermInClipboard,
    NotStmtInClipboard,
    NotProgramInClipboard,
    NothingToInsert,
    NowhereToGo,
    NoSuchCommand,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Page {
    Main,
    Help,
}

pub struct App {
    pub page: Page,
    pub program: R<Program>,
    pub selected: Option<Selection>,
    pub hovered: Option<Selection>,
    pub clipboard: Option<Selection>,
    pub keys: Vec<Cmd>,
    pub suggestions: Vec<Cmd>,
    pub error: Option<ErrorMsg>,
}

// Minor trait implementations

impl PartialEq for Selection {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Selection::SName(t) => match other {
                Selection::SName(u) => {
                    return Rc::ptr_eq(&t, &u);
                }
                _ => {
                    return false;
                }
            },
            Selection::STerm(t) => match other {
                Selection::STerm(u) => {
                    return Rc::ptr_eq(&t, &u);
                }
                _ => {
                    return false;
                }
            },
            Selection::SStmt(t) => match other {
                Selection::SStmt(u) => {
                    return Rc::ptr_eq(&t, &u);
                }
                _ => {
                    return false;
                }
            },
            Selection::SProgram(t) => match other {
                Selection::SProgram(u) => {
                    return Rc::ptr_eq(&t, &u);
                }
                _ => {
                    return false;
                }
            },
        }
    }
}
