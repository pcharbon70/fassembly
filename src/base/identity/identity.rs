use crate::term::Term::*;
use crate::term::{abs, app, Term};

/// Identity function: λx.x
///
/// This is the identity function which returns the argument to which it is
/// applied.
///
///
pub fn identity() -> Term {
    abs!(2, Var(2))
}