use crate::*;

pub trait DeepClone {
    fn deep_clone(&self) -> Self;
}

impl<T: DeepClone> DeepClone for R<T> {
    fn deep_clone(&self) -> R<T> {
        Rnew(self.borrow().deep_clone())
    }
}

impl<T: DeepClone> DeepClone for Option<T> {
    fn deep_clone(&self) -> Option<T> {
        match self {
            Some(s) => Some(s.deep_clone()),
            None => None,
        }
    }
}

impl<T: DeepClone> DeepClone for Vec<T> {
    fn deep_clone(&self) -> Vec<T> {
        self.iter().map(|x| x.deep_clone()).collect()
    }
}

impl DeepClone for Name {
    fn deep_clone(&self) -> Self {
        self.clone()
    }
}

impl DeepClone for Term {
    fn deep_clone(&self) -> Self {
        match self {
            Term::Hole => Term::Hole,
            Term::Const { c } => Term::Const { c: c.clone() },
            Term::Var { v } => Term::Var { v: v.deep_clone() },
            Term::App { t1, t2 } => Term::App {
                t1: t1.deep_clone(),
                t2: t2.deep_clone(),
            },
            Term::Lam { v, body } => Term::Lam {
                v: v.deep_clone(),
                body: body.deep_clone(),
            },
        }
    }
}

impl DeepClone for Stmt {
    fn deep_clone(&self) -> Self {
        match self {
            Stmt::Hole => Stmt::Hole,
            Stmt::Term { term } => Stmt::Term {
                term: term.deep_clone(),
            },
            Stmt::Defn { v, body } => Stmt::Defn {
                v: v.deep_clone(),
                body: body.deep_clone(),
            },
            Stmt::While { c, b1 } => Stmt::While {
                c: c.deep_clone(),
                b1: b1.deep_clone(),
            },
            Stmt::If { c, b1 } => Stmt::If {
                c: c.deep_clone(),
                b1: b1.deep_clone(),
            },
            Stmt::IfElse { c, b1, b2 } => Stmt::IfElse {
                c: c.deep_clone(),
                b1: b1.deep_clone(),
                b2: b2.deep_clone(),
            },
        }
    }
}

impl DeepClone for Program {
    fn deep_clone(&self) -> Self {
        match self {
            Program(ps) => Program(ps.deep_clone()),
        }
    }
}

impl DeepClone for Selection {
    fn deep_clone(&self) -> Self {
        match self {
            Selection::SName(r) => Selection::SName(r.deep_clone()),
            Selection::STerm(r) => Selection::STerm(r.deep_clone()),
            Selection::SStmt(r) => Selection::SStmt(r.deep_clone()),
            Selection::SProgram(r) => Selection::SProgram(r.deep_clone()),
        }
    }
}
