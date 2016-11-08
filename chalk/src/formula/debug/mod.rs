use std::fmt::{Debug, Formatter, Error};

use super::clause::*;
use super::quant::*;
use super::goal::*;
use super::leaf::*;

mod env;

impl<L: Debug> Debug for Clause<L> {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        write!(fmt, "{:?}", self.kind)
    }
}

impl<L: Debug> Debug for ClauseKind<L> {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        match *self {
            ClauseKind::Leaf(ref l) => l.fmt(fmt)?,
            ClauseKind::And(ref a, ref b) => write!(fmt, "and({:?}, {:?})", a, b)?,
            ClauseKind::Implication(ref g, ref l) => {
                write!(fmt, "implies(")?;
                g.fmt(fmt)?;
                write!(fmt, " => ")?;
                l.fmt(fmt)?;
                write!(fmt, ")")?;
            }
            ClauseKind::ForAll(ref q) => {
                write!(fmt, "forall")?;
                q.fmt(fmt)?;
            }
        }
        Ok(())
    }
}

impl<L: Debug> Debug for Goal<L> {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        self.kind.fmt(fmt)
    }
}

impl<L: Debug> Debug for GoalKind<L> {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        match *self {
            GoalKind::True => write!(fmt, "true")?,
            GoalKind::Leaf(ref l) => l.fmt(fmt)?,
            GoalKind::And(ref a, ref b) => write!(fmt, "and({:?}, {:?})", a, b)?,
            GoalKind::Or(ref a, ref b) => write!(fmt, "or({:?}; {:?})", a, b)?,
            GoalKind::Implication(ref g, ref l) => {
                write!(fmt, "implies(")?;
                g.fmt(fmt)?;
                write!(fmt, " => ")?;
                l.fmt(fmt)?;
                write!(fmt, ")")?;
            }
            GoalKind::ForAll(ref q) => {
                write!(fmt, "forall")?;
                q.fmt(fmt)?;
            }
            GoalKind::Exists(ref q) => {
                write!(fmt, "exists")?;
                q.fmt(fmt)?;
            }
        }
        Ok(())
    }
}

impl<F: Debug> Debug for Quantification<F> {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        write!(fmt, "(")?;
        for i in 0..self.num_binders {
            if i > 0 {
                write!(fmt, ", ")?;
            }
            env::bind_name(fmt)?;
        }
        write!(fmt, " -> ")?;
        self.formula.fmt(fmt)?;
        env::unbind_names(self.num_binders); // Nit: not exn-safe
        write!(fmt, ")")
    }
}

impl Debug for Leaf {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        self.kind.fmt(fmt)
    }
}

impl Debug for LeafKind {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        match *self {
            LeafKind::InferenceVariable(iv) => {
                write!(fmt, "{:?}", iv)?;
            }
            LeafKind::BoundVariable(bv) => {
                env::fmt_bound_variable(bv.depth, fmt)?;
            }
            LeafKind::Application(ref a) => {
                write!(fmt, "{:?}", a.constant)?;
                if a.args.len() > 0 {
                    fmt_parens(fmt, &a.args)?;
                }
            }
        }
        Ok(())
    }
}

impl Debug for Constant {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        match *self {
            Constant::Program(s) => write!(fmt, "{:?}", s),
            Constant::Skolemized(ui) => write!(fmt, "?skol{}", ui.counter),
        }
    }
}

fn fmt_parens<D: Debug>(fmt: &mut Formatter, vs: &[D]) -> Result<(), Error> {
    write!(fmt, "(")?;
    for (i, v) in vs.iter().enumerate() {
        if i > 0 {
            write!(fmt, ", ")?;
        }
        write!(fmt, "{:?}", v)?;
    }
    write!(fmt, ")")
}