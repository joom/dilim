#![allow(dead_code)]
use crate::editor::deep_clone::DeepClone;
use crate::*;

impl App {
    pub fn go_outwards(&mut self) -> Result<Selection, ErrorMsg> {
        match &self.selected {
            Some(s) => {
                let n = self.find_parent(s.clone());
                match n {
                    Some(s) => {
                        self.selected = Some(s.clone());
                        Ok(s)
                    }
                    None => Err(ErrorMsg::NowhereToGo),
                }
            }
            None => Err(ErrorMsg::NothingSelected),
        }
    }

    pub fn go_inwards(&mut self) -> Result<Selection, ErrorMsg> {
        match &self.selected {
            Some(s) => {
                let n = App::find_child(s.clone());
                match n {
                    Some(s) => {
                        self.selected = Some(s.clone());
                        Ok(s)
                    }
                    None => Err(ErrorMsg::NowhereToGo),
                }
            }
            None => Err(ErrorMsg::NothingSelected),
        }
    }

    pub fn go_next_sibling(&mut self) -> Result<Selection, ErrorMsg> {
        match &self.selected {
            Some(s) => {
                let n = self.find_sibling(s.clone(), true);
                match n {
                    Some(s) => {
                        self.selected = Some(s.clone());
                        Ok(s)
                    }
                    None => Err(ErrorMsg::NowhereToGo),
                }
            }
            None => Err(ErrorMsg::NothingSelected),
        }
    }

    pub fn go_prev_sibling(&mut self) -> Result<Selection, ErrorMsg> {
        match &self.selected {
            Some(s) => {
                let n = self.find_sibling(s.clone(), false);
                match n {
                    Some(s) => {
                        self.selected = Some(s.clone());
                        Ok(s)
                    }
                    None => Err(ErrorMsg::NowhereToGo),
                }
            }
            None => Err(ErrorMsg::NothingSelected),
        }
    }

    pub fn go_prev_hole(&mut self) -> Result<Selection, ErrorMsg> {
        match &self.selected {
            Some(u) => {
                let next = self.find_adjacent_hole(u.clone(), false);
                match next {
                    Some(s) => {
                        self.selected = Some(s.clone());
                        Ok(s)
                    }
                    _ => Err(ErrorMsg::NowhereToGo),
                }
            }
            None => Err(ErrorMsg::NothingSelected),
        }
    }

    pub fn go_next_hole(&mut self) -> Result<Selection, ErrorMsg> {
        match &self.selected {
            Some(u) => {
                let next = self.find_adjacent_hole(u.clone(), true);
                match next {
                    Some(s) => {
                        self.selected = Some(s.clone());
                        Ok(s)
                    }
                    _ => Err(ErrorMsg::NowhereToGo),
                }
            }
            None => Err(ErrorMsg::NothingSelected),
        }
    }

    pub fn go_specific_hole(&mut self, i: u8) -> Result<Selection, ErrorMsg> {
        let holes = self.holes();
        match holes.get(i as usize) {
            None => Err(ErrorMsg::NowhereToGo),
            Some(h) => {
                self.selected = Some(h.clone());
                Ok(h.clone())
            }
        }
    }

    pub fn delete(&mut self) -> Result<Selection, ErrorMsg> {
        self.clipboard = self.selected.clone().deep_clone();
        match &self.selected {
            Some(Selection::SName(r)) => {
                *(r.borrow_mut()) = Name::Hole;
                Ok(Selection::SName(r.clone()))
            }
            Some(Selection::STerm(r)) => {
                *(r.borrow_mut()) = Term::Hole;
                Ok(Selection::STerm(r.clone()))
            }
            Some(Selection::SStmt(r1)) => {
                let r1c = r1.borrow().clone();
                match r1c {
                    Stmt::Hole => match self.find_parent(Selection::SStmt(r1.clone())) {
                        Some(Selection::SProgram(r2)) => {
                            let sel = Selection::SStmt(r1.clone());
                            match *(r2.borrow_mut()) {
                                Program(ref mut ps) => {
                                    let i = ps
                                        .iter()
                                        .position(move |p| {
                                            Selection::SStmt(p.clone()).eq(&sel.clone())
                                        })
                                        .unwrap();
                                    ps.remove(i);
                                    match ps.get(i) {
                                        Some(s) => {
                                            self.selected = Some(Selection::SStmt(s.clone()));
                                            Ok(Selection::SStmt(s.clone()))
                                        }
                                        None => {
                                            self.selected = None;
                                            Ok(Selection::SProgram(r2.clone()))
                                        }
                                    }
                                }
                            }
                        }
                        _ => Err(ErrorMsg::Impossible), // parent is not a program
                    },
                    _ => {
                        *(r1.borrow_mut()) = Stmt::Hole;
                        Ok(Selection::SStmt(r1.clone()))
                    }
                }
            }
            Some(Selection::SProgram(r)) => {
                let t = Stmt::Hole;
                *(r.borrow_mut()) = Program(vec![Rnew(t)]);
                Ok(Selection::SProgram(r.clone()))
            }
            None => Err(ErrorMsg::NothingSelected),
        }
    }

    pub fn yank(&mut self) -> Result<Selection, ErrorMsg> {
        match &self.selected {
            Some(s) => {
                let t = s.clone().deep_clone();
                self.clipboard = Some(t.clone());
                Ok(t)
            }
            None => Err(ErrorMsg::NothingSelected),
        }
    }

    pub fn paste(&mut self) -> Result<Selection, ErrorMsg> {
        match &self.selected {
            Some(Selection::SName(r1)) => match &self.clipboard {
                Some(Selection::SName(r2)) => {
                    *(r1.borrow_mut()) = r2.borrow().clone().deep_clone();
                    Ok(Selection::SName((*r1).clone()))
                }
                Some(_) => Err(ErrorMsg::NotNameInClipboard),
                None => Err(ErrorMsg::NothingInClipboard),
            },
            Some(Selection::STerm(r1)) => match &self.clipboard {
                Some(Selection::STerm(r2)) => {
                    *(r1.borrow_mut()) = r2.borrow().clone().deep_clone();
                    Ok(Selection::STerm((*r1).clone()))
                }
                Some(_) => Err(ErrorMsg::NotTermInClipboard),
                None => Err(ErrorMsg::NothingInClipboard),
            },
            Some(Selection::SStmt(r1)) => match &self.clipboard {
                Some(Selection::SStmt(r2)) => {
                    *(r1.borrow_mut()) = r2.borrow().clone().deep_clone();
                    Ok(Selection::SStmt((*r1).clone()))
                }
                Some(_) => Err(ErrorMsg::NotStmtInClipboard),
                None => Err(ErrorMsg::NothingInClipboard),
            },
            Some(Selection::SProgram(r1)) => match &self.clipboard {
                Some(Selection::SProgram(r2)) => {
                    *(r1.borrow_mut()) = r2.borrow().clone().deep_clone();
                    Ok(Selection::SProgram((*r1).clone()))
                }
                Some(_) => Err(ErrorMsg::NotProgramInClipboard),
                None => Err(ErrorMsg::NothingInClipboard),
            },
            None => Err(ErrorMsg::NothingSelected),
        }
    }

    pub fn add_fresh(&mut self) -> Result<Selection, ErrorMsg> {
        let name = self.gensym();
        match &self.selected {
            Some(Selection::SName(r)) => {
                let t = Name::Named { name };
                *(r.borrow_mut()) = t.clone();
                Ok(Selection::SName(r.clone()))
            }
            Some(_) => Err(ErrorMsg::NotANameHole),
            None => Err(ErrorMsg::NothingSelected),
        }
    }

    pub fn add_stmt_to_program(&mut self) -> Result<Selection, ErrorMsg> {
        let h = Rnew(Stmt::Hole);
        match &self.selected {
            Some(Selection::SProgram(r)) => match *(r.borrow_mut()) {
                Program(ref mut ps) => {
                    ps.push(h.clone());
                    Ok(Selection::SStmt(h))
                }
            },
            Some(_) => Err(ErrorMsg::NotAProgram),
            None => Err(ErrorMsg::NothingSelected),
        }
    }

    pub fn add_stmt_after_stmt(&mut self) -> Result<Selection, ErrorMsg> {
        let h = Rnew(Stmt::Hole);
        match &self.selected {
            Some(Selection::SStmt(r1)) => match self.find_parent(Selection::SStmt(r1.clone())) {
                Some(Selection::SProgram(r2)) => {
                    let sel = Selection::SStmt(r1.clone());
                    match *(r2.borrow_mut()) {
                        Program(ref mut ps) => {
                            let i = ps
                                .iter()
                                .position(move |p| Selection::SStmt(p.clone()).eq(&sel.clone()))
                                .unwrap();
                            ps.insert(i + 1, h.clone());
                            self.selected = Some(Selection::SStmt(h.clone()));
                            Ok(Selection::SStmt(h))
                        }
                    }
                }
                _ => Err(ErrorMsg::NotAStmtHole),
            },
            Some(_) => Err(ErrorMsg::NotAStmt),
            None => Err(ErrorMsg::NothingSelected),
        }
    }

    pub fn add_term_stmt_in_stmt_hole(&mut self) -> Result<Selection, ErrorMsg> {
        match self.selected.clone() {
            Some(Selection::SStmt(r)) => match *r.borrow_mut() {
                ref mut s @ Stmt::Hole => {
                    let h = Rnew(Term::Hole);
                    *s = Stmt::Term { term: h.clone() };
                    self.selected = Some(Selection::STerm(h.clone()));
                    Ok(Selection::STerm(h))
                }
                _ => Err(ErrorMsg::NotAStmtHole),
            },
            Some(_) => Err(ErrorMsg::NotAStmtHole),
            None => Err(ErrorMsg::NothingSelected),
        }
    }

    pub fn add_defn_stmt_in_stmt_hole(&mut self) -> Result<Selection, ErrorMsg> {
        match self.selected.clone() {
            Some(Selection::SStmt(r)) => match *r.borrow_mut() {
                ref mut s @ Stmt::Hole => {
                    let h1 = Rnew(Name::Hole);
                    let h2 = Rnew(Term::Hole);
                    *s = Stmt::Defn {
                        v: h1.clone(),
                        body: h2.clone(),
                    };
                    self.selected = Some(Selection::STerm(h2.clone()));
                    Ok(Selection::STerm(h2))
                }
                _ => Err(ErrorMsg::NotAStmtHole),
            },
            Some(_) => Err(ErrorMsg::NotAStmtHole),
            None => Err(ErrorMsg::NothingSelected),
        }
    }

    pub fn add_while_stmt_in_stmt_hole(&mut self) -> Result<Selection, ErrorMsg> {
        match self.selected.clone() {
            Some(Selection::SStmt(r)) => match *r.borrow_mut() {
                ref mut s @ Stmt::Hole => {
                    let h1 = Rnew(Term::Hole);
                    let h2 = Rnew(Stmt::Hole);
                    let h3 = Rnew(Program(vec![h2]));
                    *s = Stmt::While {
                        c: h1.clone(),
                        b1: h3.clone(),
                    };
                    self.selected = Some(Selection::SStmt(r.clone()));
                    Ok(Selection::SStmt(r.clone()))
                }
                _ => Err(ErrorMsg::NotAStmtHole),
            },
            Some(_) => Err(ErrorMsg::NotAStmtHole),
            None => Err(ErrorMsg::NothingSelected),
        }
    }

    pub fn add_if_stmt_in_stmt_hole(&mut self) -> Result<Selection, ErrorMsg> {
        match self.selected.clone() {
            Some(Selection::SStmt(r)) => match *r.borrow_mut() {
                ref mut s @ Stmt::Hole => {
                    let h1 = Rnew(Term::Hole);
                    let h2 = Rnew(Stmt::Hole);
                    let h3 = Rnew(Program(vec![h2]));
                    *s = Stmt::If {
                        c: h1.clone(),
                        b1: h3.clone(),
                    };
                    self.selected = Some(Selection::SStmt(r.clone()));
                    Ok(Selection::SStmt(r.clone()))
                }
                _ => Err(ErrorMsg::NotAStmtHole),
            },
            Some(_) => Err(ErrorMsg::NotAStmtHole),
            None => Err(ErrorMsg::NothingSelected),
        }
    }

    pub fn add_else_to_if_stmt(&mut self) -> Result<Selection, ErrorMsg> {
        match self.selected.clone() {
            Some(Selection::SStmt(r)) => match &*r.deep_clone().borrow() {
                Stmt::If {c, b1} => {
                    let h1 = Rnew(Stmt::Hole);
                    let h2 = Rnew(Program(vec![h1.clone()]));
                    *r.borrow_mut() = Stmt::IfElse {
                        c: c.clone(),
                        b1: b1.clone(),
                        b2: h2.clone()
                    };
                    self.selected = Some(Selection::SStmt(h1.clone()));
                    Ok(Selection::SStmt(h1.clone()))
                }
                _ => Err(ErrorMsg::NotAStmtHole),
            },
            Some(_) => Err(ErrorMsg::NotAStmtHole),
            None => Err(ErrorMsg::NothingSelected),
        }
    }

    pub fn add_var_in_term_hole(&mut self) -> Result<Selection, ErrorMsg> {
        match self.selected.clone() {
            Some(Selection::STerm(r)) => match *r.borrow_mut() {
                ref mut s @ Term::Hole => {
                    let v = Rnew(Name::Hole);
                    *s = Term::Var { v: v.clone() };
                    self.selected = Some(Selection::SName(v.clone()));
                    Ok(Selection::SName(v))
                }
                _ => Err(ErrorMsg::NotATermHole),
            },
            Some(_) => Err(ErrorMsg::NotATermHole),
            None => Err(ErrorMsg::NothingSelected),
        }
    }

    pub fn add_const_in_term_hole(&mut self, c: Const) -> Result<Selection, ErrorMsg> {
        match self.selected.clone() {
            Some(Selection::STerm(r)) => match *r.borrow_mut() {
                ref mut s @ Term::Hole => {
                    *s = Term::Const { c };
                    self.selected = Some(Selection::STerm(r.clone()));
                    Ok(Selection::STerm(r.clone()))
                }
                _ => Err(ErrorMsg::NotATermHole),
            },
            Some(_) => Err(ErrorMsg::NotATermHole),
            None => Err(ErrorMsg::NothingSelected),
        }
    }

    pub fn add_app_in_term_hole(&mut self) -> Result<Selection, ErrorMsg> {
        match self.selected.clone() {
            Some(Selection::STerm(r)) => match *r.borrow_mut() {
                ref mut s @ Term::Hole => {
                    let h1 = Rnew(Term::Hole);
                    let h2 = Rnew(Term::Hole);
                    *s = Term::App {
                        t1: h1.clone(),
                        t2: h2,
                    };
                    self.selected = Some(Selection::STerm(h1.clone()));
                    Ok(Selection::STerm(h1))
                }
                _ => Err(ErrorMsg::NotATermHole),
            },
            Some(_) => Err(ErrorMsg::NotATermHole),
            None => Err(ErrorMsg::NothingSelected),
        }
    }

    pub fn add_lam_in_term_hole(&mut self) -> Result<Selection, ErrorMsg> {
        match self.selected.clone() {
            Some(Selection::STerm(r)) => match *r.borrow_mut() {
                ref mut s @ Term::Hole => {
                    let h1 = Rnew(Name::Hole);
                    let h2 = Rnew(Term::Hole);
                    *s = Term::Lam {
                        v: h1.clone(),
                        body: h2.clone(),
                    };
                    self.selected = Some(Selection::STerm(h2.clone()));
                    Ok(Selection::STerm(h2))
                }
                _ => Err(ErrorMsg::NotATermHole),
            },
            Some(_) => Err(ErrorMsg::NotATermHole),
            None => Err(ErrorMsg::NothingSelected),
        }
    }

    pub fn fill_name_hole(&mut self, name: String) -> Result<Selection, ErrorMsg> {
        match self.selected.clone() {
            Some(Selection::SName(r)) => match *r.borrow_mut() {
                ref mut s @ Name::Hole => {
                    *s = Name::Named { name };
                    self.selected = Some(Selection::SName(r.clone()));
                    Ok(Selection::SName(r.clone()))
                }
                _ => Err(ErrorMsg::NotANameHole),
            },
            _ => Err(ErrorMsg::NotANameHole),
        }
    }

    pub fn wrap_in_app(&mut self) -> Result<Selection, ErrorMsg> {
        match &self.selected.clone() {
            Some(Selection::STerm(r)) => match *r.borrow_mut() {
                ref mut term => {
                    let h1 = Rnew(term.clone());
                    let h2 = Rnew(Term::Hole);
                    let t = Term::App {
                        t1: h1.clone(),
                        t2: h2.clone(),
                    };
                    *term = t;
                    self.selected = Some(Selection::STerm(h1.clone()));
                    Ok(Selection::STerm(h1))
                }
            },
            Some(_) => Err(ErrorMsg::NotATermHole),
            None => Err(ErrorMsg::NothingSelected),
        }
    }

    pub fn wrap_in_lam(&mut self) -> Result<Selection, ErrorMsg> {
        match &self.selected.clone() {
            Some(Selection::STerm(r)) => match *r.borrow_mut() {
                ref mut term => {
                    let h1 = Rnew(Name::Hole);
                    let h2 = Rnew(term.clone());
                    let t = Term::Lam {
                        v: h1.clone(),
                        body: h2.clone(),
                    };
                    *term = t;
                    self.selected = Some(Selection::STerm(h2.clone()));
                    Ok(Selection::STerm(h2))
                }
            },
            Some(_) => Err(ErrorMsg::NotATermHole),
            None => Err(ErrorMsg::NothingSelected),
        }
    }

    pub fn wrap_in_defn(&mut self) -> Result<Selection, ErrorMsg> {
        match &self.selected.clone() {
            Some(Selection::SStmt(r)) => match *r.borrow_mut() {
                ref mut stmt => match stmt.clone() {
                    Stmt::Term { term } => {
                        let h1 = Rnew(Name::Hole);
                        let t = Stmt::Defn {
                            v: h1.clone(),
                            body: term.clone(),
                        };
                        *stmt = t;
                        self.selected = Some(Selection::STerm(term.clone()));
                        Ok(Selection::STerm(term.clone()))
                    }
                    _ => Err(ErrorMsg::NotATermStmt),
                },
            },
            Some(_) => Err(ErrorMsg::NotATermStmt),
            None => Err(ErrorMsg::NothingSelected),
        }
    }
}
