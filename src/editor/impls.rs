use crate::editor::deep_clone::DeepClone;
use crate::*;

impl Name {
    fn new(x: String) -> Self {
        Name::Named { name: x }
    }
    fn new_rc(x: String) -> R<Self> {
        Rnew(Name::Named { name: x })
    }
    fn names(t: R<Name>) -> Vec<String> {
        let mut v = Vec::new();
        Name::names_acc(t, &mut v);
        v
    }
    fn names_acc(t: R<Name>, vec: &mut Vec<String>) -> () {
        match &*t.borrow_mut() {
            Name::Hole => {}
            Name::Named { name } => {
                vec.push(name.clone());
            }
        }
    }
    fn holes_acc(t: R<Name>, vec: &mut Vec<Selection>) -> () {
        match &*t.borrow_mut() {
            Name::Hole => {
                vec.push(Selection::SName(t.clone()));
            }
            Name::Named { .. } => {}
        }
    }
}

impl Term {
    fn names(t: R<Self>) -> Vec<String> {
        let mut v = Vec::new();
        Term::names_acc(t, &mut v);
        v
    }

    fn names_acc(t: R<Self>, vec: &mut Vec<String>) -> () {
        match &*t.borrow_mut() {
            Term::Hole => {}
            Term::Const { .. } => {}
            Term::Var { v } => match &*v.borrow() {
                Name::Named { name } => {
                    vec.push(name.clone());
                }
                _ => {}
            },
            Term::App { t1, t2 } => {
                Term::names_acc(t1.clone(), vec);
                Term::names_acc(t2.clone(), vec);
            }
            Term::Lam { v, body } => {
                match &*v.borrow() {
                    Name::Named { name } => {
                        vec.push(name.clone());
                    }
                    _ => {}
                }
                Term::names_acc(body.clone(), vec);
            }
        }
    }

    pub fn holes_acc(t: R<Self>, vec: &mut Vec<Selection>) -> () {
        match &*t.borrow_mut() {
            Term::Hole => {
                vec.push(Selection::STerm(t.clone()));
            }
            Term::Const { .. } => {}
            Term::Var { v } => match &*v.borrow_mut() {
                Name::Named { .. } => {}
                Name::Hole => {
                    vec.push(Selection::SName(v.clone()));
                }
            },
            Term::App { t1, t2 } => {
                Term::holes_acc(t1.clone(), vec);
                Term::holes_acc(t2.clone(), vec);
            }
            Term::Lam { v, body } => {
                match &*v.borrow_mut() {
                    Name::Named { .. } => {}
                    Name::Hole => {
                        vec.push(Selection::SName(v.clone()));
                    }
                }
                Term::holes_acc(body.clone(), vec);
            }
        }
    }
}

impl Stmt {
    fn names(t: R<Self>) -> Vec<String> {
        let mut v = Vec::new();
        Stmt::names_acc(t, &mut v);
        v
    }

    fn names_acc(t: R<Self>, vec: &mut Vec<String>) -> () {
        match &*t.borrow_mut() {
            Stmt::Hole => {}
            Stmt::Term { term } => {
                Term::names_acc(term.clone(), vec);
            }
            Stmt::Defn { v, body } => {
                Name::names_acc(v.clone(), vec);
                Term::names_acc(body.clone(), vec);
            }
            Stmt::While { c, b1 } => {
                Term::names_acc(c.clone(), vec);
                Program::names_acc(b1.clone(), vec);
            }
            Stmt::If { c, b1 } => {
                Term::names_acc(c.clone(), vec);
                Program::names_acc(b1.clone(), vec);
            }
            Stmt::IfElse { c, b1, b2 } => {
                Term::names_acc(c.clone(), vec);
                Program::names_acc(b1.clone(), vec);
                Program::names_acc(b2.clone(), vec);
            }
        }
    }

    pub fn holes_acc(t: R<Self>, vec: &mut Vec<Selection>) -> () {
        match &*t.borrow_mut() {
            Stmt::Hole => vec.push(Selection::SStmt(t.clone())),
            Stmt::Term { term } => {
                Term::holes_acc(term.clone(), vec);
            }
            Stmt::Defn { v, body } => {
                Name::holes_acc(v.clone(), vec);
                Term::holes_acc(body.clone(), vec);
            }
            Stmt::While { c, b1 } => {
                Term::holes_acc(c.clone(), vec);
                Program::holes_acc(b1.clone(), vec);
            }
            Stmt::If { c, b1 } => {
                Term::holes_acc(c.clone(), vec);
                Program::holes_acc(b1.clone(), vec);
            }
            Stmt::IfElse { c, b1, b2 } => {
                Term::holes_acc(c.clone(), vec);
                Program::holes_acc(b1.clone(), vec);
                Program::holes_acc(b2.clone(), vec);
            }
        }
    }
}

impl Program {
    fn names(t: R<Self>) -> Vec<String> {
        let mut v = Vec::new();
        Program::names_acc(t, &mut v);
        v
    }

    fn names_acc(t: R<Self>, vec: &mut Vec<String>) -> () {
        match &*t.borrow_mut() {
            Program(ps) => {
                ps.iter().map(|p| Stmt::names_acc(p.clone(), vec)).count();
            }
        }
        ()
    }

    pub fn holes_acc(t: R<Self>, vec: &mut Vec<Selection>) -> () {
        match &*t.borrow_mut() {
            Program(ps) => {
                ps.iter().map(|p| Stmt::holes_acc(p.clone(), vec)).count();
            }
        }
        ()
    }
}

impl Cmd {
    pub fn cmd_ended(v: &Vec<Cmd>) -> bool {
        match v.last() {
            Some(Cmd::Done) | Some(Cmd::Failed) => true,
            _ => false,
        }
    }
}

impl std::ops::Deref for Cmds {
    type Target = Vec<Cmd>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl App {
    pub fn complete_command(keys: Vec<Cmd>) -> Vec<Cmd> {
        match &keys[..] {
            [] => vec![
                Cmd::Help,
                Cmd::Add,
                Cmd::Delete,
                Cmd::Go,
                Cmd::Insert,
                Cmd::Wrap,
                Cmd::Yank,
                Cmd::Paste,
            ],
            [Cmd::Wrap] => vec![Cmd::App, Cmd::Lam, Cmd::Defn],
            [Cmd::Add] => vec![
                Cmd::App,
                Cmd::Lam,
                Cmd::Var,
                Cmd::Constant,
                Cmd::Defn,
                Cmd::Term,
                Cmd::Statement,
                Cmd::Fresh,
                // Cmd::While,
                // Cmd::If,
                // Cmd::Else,
            ],
            [Cmd::Add, Cmd::Constant] => vec![Cmd::True, Cmd::False, Cmd::Number(0)],
            [Cmd::Delete] => vec![],
            [Cmd::Go] => vec![
                Cmd::Number(0),
                Cmd::Prev,
                Cmd::Next,
                Cmd::Outwards,
                Cmd::Inwards,
            ],
            [Cmd::Go, Cmd::Number(_)] => vec![Cmd::Hole],
            [Cmd::Go, Cmd::Number(_), Cmd::Hole] => vec![],
            [Cmd::Go, Cmd::Outwards | Cmd::Inwards] => vec![],
            [Cmd::Go, Cmd::Prev | Cmd::Next] => vec![Cmd::Hole, Cmd::Sibling],
            [Cmd::Go, Cmd::Prev | Cmd::Next, Cmd::Hole | Cmd::Sibling] => vec![],
            [Cmd::Insert] => vec![Cmd::Text(String::from("name"))],
            [Cmd::Insert, Cmd::Text(_)] => vec![Cmd::Text(String::from("more")), Cmd::Enter],
            [Cmd::Insert, Cmd::Text(_), Cmd::Enter] => vec![],
            _ => vec![],
        }
    }

    pub fn run_command(&mut self) -> () {
        let keys = self.keys.clone();
        match self.run_command_res(keys) {
            Ok(true) => {
                self.keys.push(Cmd::Done);
            }
            Ok(false) => {}
            Err(e) => {
                self.error = Some(e);
                self.keys.push(Cmd::Failed);
            }
        }
    }
    pub fn run_command_res(&mut self, keys: Vec<Cmd>) -> Result<bool, ErrorMsg> {
        match &keys.clone()[..] {
            [Cmd::Number(_)] => Ok(false),
            [Cmd::Number(n), ..] => {
                let mut rest = keys.clone();
                rest.remove(0);
                let mut temp = Ok(false);
                for _i in 0..*n {
                    temp = self.run_command_res(rest.clone());
                    match temp {
                        Ok(false) => return Ok(false),
                        Ok(true) => {}
                        Err(e) => return Err(e),
                    }
                }
                temp
            }
            [Cmd::Help] => {
                self.page = Page::Help;
                Ok(true)
            }
            [Cmd::Wrap] => Ok(false),
            [Cmd::Wrap, Cmd::Lam] => self.wrap_in_lam().map(|_| true),
            [Cmd::Wrap, Cmd::App] => self.wrap_in_app().map(|_| true),
            [Cmd::Wrap, Cmd::Defn] => self.wrap_in_defn().map(|_| true),
            // .or_else(|_| self.go_outwards().and_then(|_| self.wrap_in_defn().map(|_| true))),
            [Cmd::Add] => Ok(false),
            [Cmd::Add, Cmd::Statement] => self.add_stmt_to_program().map(|_| true).or_else(|_| {
                self.add_stmt_after_stmt()
                    .map(|_| true)
                    .or_else(|_| Err(ErrorMsg::NotAProgramOrStmtHole))
            }),
            [Cmd::Add, Cmd::Term] => self.add_term_stmt_in_stmt_hole().map(|_| true),
            [Cmd::Add, Cmd::Defn] => self.add_defn_stmt_in_stmt_hole().map(|_| true),
            [Cmd::Add, Cmd::While] => self.add_while_stmt_in_stmt_hole().map(|_| true),
            [Cmd::Add, Cmd::If] => self.add_if_stmt_in_stmt_hole().map(|_| true),
            [Cmd::Add, Cmd::Else] => self.add_else_to_if_stmt().map(|_| true),
            [Cmd::Add, Cmd::Var] => (self
                .add_term_stmt_in_stmt_hole()
                .and_then(|_| self.add_var_in_term_hole().map(|_| true)))
            .or_else(|_| self.add_var_in_term_hole().map(|_| true)),
            [Cmd::Add, Cmd::App] => (self
                .add_term_stmt_in_stmt_hole()
                .and_then(|_| self.add_app_in_term_hole().map(|_| true)))
            .or_else(|_| self.add_app_in_term_hole().map(|_| true)),
            [Cmd::Add, Cmd::Lam] => (self
                .add_term_stmt_in_stmt_hole()
                .and_then(|_| self.add_lam_in_term_hole().map(|_| true)))
            .or_else(|_| self.add_lam_in_term_hole().map(|_| true)),
            [Cmd::Add, Cmd::Fresh] => self.add_fresh().map(|_| true),
            [Cmd::Add, Cmd::Constant] => Ok(false),
            [Cmd::Add, Cmd::Constant, Cmd::True] => {
                self.add_const_in_term_hole(Const::Bool(true)).map(|_| true)
            }
            [Cmd::Add, Cmd::Constant, Cmd::False] => self
                .add_const_in_term_hole(Const::Bool(false))
                .map(|_| true),
            [Cmd::Delete] => self.delete().map(|_| true),
            [Cmd::Yank] => self.yank().map(|_| true),
            [Cmd::Paste] => self.paste().map(|_| true),
            [Cmd::Insert, Cmd::Enter] => Err(ErrorMsg::NothingToInsert),
            [Cmd::Insert, Cmd::Text(s), Cmd::Enter] => {
                let name = s.to_string();
                (self.add_term_stmt_in_stmt_hole().and_then(|_| {
                    self.add_var_in_term_hole()
                        .and_then(|_| self.fill_name_hole(name.clone()).map(|_| true))
                }))
                .or_else(|_| {
                    self.add_var_in_term_hole()
                        .and_then(|_| self.fill_name_hole(name.clone()).map(|_| true))
                })
                .or_else(|_| self.fill_name_hole(name).map(|_| true))
            }
            [Cmd::Insert] | [Cmd::Insert, Cmd::Text(_)] => Ok(false),
            [Cmd::Go, Cmd::Outwards] => self.go_outwards().map(|_| true),
            [Cmd::Go, Cmd::Inwards] => self.go_inwards().map(|_| true),
            [Cmd::Go, Cmd::Prev, Cmd::Sibling] => self.go_prev_sibling().map(|_| true),
            [Cmd::Go, Cmd::Next, Cmd::Sibling] => self.go_next_sibling().map(|_| true),
            [Cmd::Go, Cmd::Prev, Cmd::Hole] => self.go_prev_hole().map(|_| true),
            [Cmd::Go, Cmd::Next, Cmd::Hole] => self.go_next_hole().map(|_| true),
            [Cmd::Go, Cmd::Number(i), Cmd::Hole] => self.go_specific_hole(*i).map(|_| true),
            [Cmd::Go] => Ok(false),
            [Cmd::Add, _] | [_] => Err(ErrorMsg::NoSuchCommand),
            _ => Ok(false),
        }
    }

    fn gensym_aux(chars: &Vec<String>, start: &Vec<String>) -> Vec<String> {
        start
            .iter()
            .map(|c| chars.iter().map(move |d| format!("{}{}", c, d)))
            .flatten()
            .collect()
    }

    // generates a,b,c,...y,z,aa,ab,ac,ad...,aaa,aab,aac...
    pub fn gensym(&self) -> String {
        let names = Program::names(self.program.clone());
        let chars: Vec<String> = vec![
            "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q",
            "r", "s", "t", "u", "v", "w", "x", "y", "z",
        ]
        .iter()
        .map(|x| x.to_string())
        .collect();
        let mut v: Vec<String> = vec![String::new()];
        loop {
            v = App::gensym_aux(&chars, &v);
            for n in v.iter() {
                if !names.contains(&n) {
                    return n.to_string();
                }
            }
        }
    }

    pub fn holes(&self) -> Vec<Selection> {
        let mut v = Vec::new();
        Program::holes_acc(self.program.clone(), &mut v);
        v
    }

    pub fn prev_hole(&self, t: Selection) -> Option<Selection> {
        let v = self.holes();
        let oi = v.iter().position(move |u| t.eq(u));
        match oi {
            None => None,
            Some(i) => {
                if !(0 < i) {
                    None
                } else {
                    v.get(i - 1).map(|x| (*x).clone())
                }
            }
        }
    }

    pub fn next_hole(&self, t: Selection) -> Option<Selection> {
        let v = self.holes();
        let oi = v.iter().position(move |u| t.eq(u));
        match oi {
            None => None,
            Some(i) => {
                let len = v.len();
                if !(i + 1 < len) {
                    None
                } else {
                    v.get(i + 1).map(|x| (*x).clone())
                }
            }
        }
    }

    pub fn find_adjacent_hole(&self, t: Selection, is_next: bool) -> Option<Selection> {
        let v = self.holes();
        let oi = v.iter().position(move |u| t.eq(u));
        match oi {
            None => None,
            Some(i) => {
                let j = if i == 0 {
                    if is_next {
                        1
                    } else {
                        v.len() - 1
                    }
                } else {
                    (if is_next { i + 1 } else { i - 1 }) % v.len()
                };
                v.get(j).map(|x| (*x).clone())
            }
        }
    }

    pub fn find_parent_aux(
        &self,
        goal: Selection,
        curr: Selection,
        prev: Option<Selection>,
    ) -> Option<Selection> {
        if goal == curr {
            prev
        } else {
            let currc = curr.clone();
            match curr {
                Selection::SName(_) => None,
                Selection::STerm(r) => match &*(r.borrow()) {
                    Term::Hole => None,
                    Term::Const { .. } => None,
                    Term::Var { v } => {
                        self.find_parent_aux(goal, Selection::SName((*v).clone()), Some(currc))
                    }
                    Term::App { t1, t2 } => self
                        .find_parent_aux(
                            goal.clone(),
                            Selection::STerm((*t1).clone()),
                            Some(currc.clone()),
                        )
                        .or(self.find_parent_aux(
                            goal,
                            Selection::STerm((*t2).clone()),
                            Some(currc),
                        )),
                    Term::Lam { v, body } => self
                        .find_parent_aux(
                            goal.clone(),
                            Selection::SName((*v).clone()),
                            Some(currc.clone()),
                        )
                        .or(self.find_parent_aux(
                            goal,
                            Selection::STerm((*body).clone()),
                            Some(currc),
                        )),
                },
                Selection::SStmt(r) => match &*(r.borrow()) {
                    Stmt::Hole => None,
                    Stmt::Term { term } => {
                        self.find_parent_aux(goal, Selection::STerm((*term).clone()), Some(currc))
                    }
                    Stmt::Defn { v, body } => self
                        .find_parent_aux(
                            goal.clone(),
                            Selection::SName((*v).clone()),
                            Some(currc.clone()),
                        )
                        .or(self.find_parent_aux(
                            goal,
                            Selection::STerm((*body).clone()),
                            Some(currc),
                        )),
                    Stmt::While { c, b1 } => self
                        .find_parent_aux(
                            goal.clone(),
                            Selection::STerm((*c).clone()),
                            Some(currc.clone()),
                        )
                        .or(self.find_parent_aux(
                            goal,
                            Selection::SProgram((*b1).clone()),
                            Some(currc),
                        )),
                    Stmt::If { c, b1 } => self
                        .find_parent_aux(
                            goal.clone(),
                            Selection::STerm((*c).clone()),
                            Some(currc.clone()),
                        )
                        .or(self.find_parent_aux(
                            goal,
                            Selection::SProgram((*b1).clone()),
                            Some(currc),
                        )),
                    Stmt::IfElse { c, b1, b2 } => self
                        .find_parent_aux(
                            goal.clone(),
                            Selection::STerm((*c).clone()),
                            Some(currc.clone()),
                        )
                        .or(self.find_parent_aux(
                            goal.clone(),
                            Selection::SProgram((*b1).clone()),
                            Some(currc.clone()),
                        ))
                        .or(self.find_parent_aux(
                            goal,
                            Selection::SProgram((*b2).clone()),
                            Some(currc),
                        )),
                },
                Selection::SProgram(r) => match &*(r.borrow()) {
                    Program(vec) => {
                        for s in vec {
                            match self.find_parent_aux(
                                goal.clone(),
                                Selection::SStmt(s.clone()),
                                Some(currc.clone()),
                            ) {
                                None => {}
                                Some(s) => {
                                    return Some(s);
                                }
                            }
                        }
                        return None;
                    }
                },
            }
        }
    }

    pub fn find_parent(&self, goal: Selection) -> Option<Selection> {
        let t = Selection::SProgram(self.program.clone());
        self.find_parent_aux(goal, t, None)
    }
    pub fn find_child(goal: Selection) -> Option<Selection> {
        match goal {
            Selection::SName(_) => None,
            Selection::STerm(r) => match &*(r.borrow()) {
                Term::Hole => None,
                Term::Const { .. } => None,
                Term::Var { v } => Some(Selection::SName(v.clone())),
                Term::App { t1, t2: _ } => Some(Selection::STerm(t1.clone())),
                Term::Lam { v, body: _ } => Some(Selection::SName(v.clone())),
            },
            Selection::SStmt(r) => match &*(r.borrow()) {
                Stmt::Hole => None,
                Stmt::Term { term } => Some(Selection::STerm(term.clone())),
                Stmt::Defn { v, body: _ } => Some(Selection::SName(v.clone())),
                Stmt::While{ c, b1: _ } => Some(Selection::STerm(c.clone())),
                Stmt::If { c, b1: _ } => Some(Selection::STerm(c.clone())),
                Stmt::IfElse { c, b1: _, b2: _ } => Some(Selection::STerm(c.clone())),
            },
            Selection::SProgram(r) => match &*(r.borrow()) {
                Program(ps) => ps.get(0).map(|s| Selection::SStmt(s.clone())),
            },
        }
    }

    pub fn find_sibling(&self, goal: Selection, is_next: bool) -> Option<Selection> {
        let goalc = goal.clone();
        let p = self.find_parent(goal);
        match p {
            Some(Selection::STerm(r)) => match &*(r.borrow()) {
                Term::App { t1, t2 } => {
                    let v = vec![Selection::STerm(t1.clone()), Selection::STerm(t2.clone())];
                    let oi = v.iter().position(move |u| goalc.eq(u));
                    match oi {
                        None => None,
                        Some(i) => {
                            let j = if i == 0 {
                                if is_next {
                                    1
                                } else {
                                    v.len() - 1
                                }
                            } else {
                                (if is_next { i + 1 } else { i - 1 }) % v.len()
                            };
                            v.get(j).map(|x| (*x).clone())
                        }
                    }
                }
                Term::Lam { v, body } => {
                    let v = vec![Selection::SName(v.clone()), Selection::STerm(body.clone())];
                    let oi = v.iter().position(move |u| goalc.eq(u));
                    match oi {
                        None => None,
                        Some(i) => {
                            let j = if i == 0 {
                                if is_next {
                                    1
                                } else {
                                    v.len() - 1
                                }
                            } else {
                                (if is_next { i + 1 } else { i - 1 }) % v.len()
                            };
                            v.get(j).map(|x| (*x).clone())
                        }
                    }
                }
                _ => None,
            },
            Some(Selection::SStmt(r)) => match &*(r.borrow()) {
                Stmt::Defn { v, body } => {
                    let v = vec![Selection::SName(v.clone()), Selection::STerm(body.clone())];
                    let oi = v.iter().position(move |u| goalc.eq(u));
                    match oi {
                        None => None,
                        Some(i) => {
                            let j = if i == 0 {
                                if is_next {
                                    1
                                } else {
                                    v.len() - 1
                                }
                            } else {
                                (if is_next { i + 1 } else { i - 1 }) % v.len()
                            };
                            v.get(j).map(|x| (*x).clone())
                        }
                    }
                }
                _ => None,
            },
            Some(Selection::SProgram(r)) => match &*(r.borrow()) {
                Program(ps) => {
                    let oi = ps
                        .iter()
                        .map(|u| Selection::SStmt(u.clone()))
                        .position(move |u| goalc.eq(&u));
                    match oi {
                        None => None,
                        Some(i) => {
                            let j = if i == 0 {
                                if is_next {
                                    1
                                } else {
                                    ps.len() - 1
                                }
                            } else {
                                (if is_next { i + 1 } else { i - 1 }) % ps.len()
                            };
                            ps.get(j).map(|x| Selection::SStmt((*x).clone()))
                        }
                    }
                }
            },
            _ => None,
        }
    }
}
