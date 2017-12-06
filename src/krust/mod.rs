//! Data Structures

mod parser;
mod expr;

pub use self::expr::{Expr, Name};
pub use self::parser::{ParseError, parse_expr};