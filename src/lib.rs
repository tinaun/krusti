//! an interpreter for a k5-k6 ish language
//! heavily inspired by JohnEarnest's oK
#![recursion_limit = "256"]

#[macro_use]
extern crate pest_derive;
extern crate pest;

pub mod krust;

use std::collections::BTreeMap;
use krust::{Name, Expr};

#[derive(Debug, PartialEq)]
pub struct Interpreter {
    names: BTreeMap<Name, Expr>
}

/// k interpreter
impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            names: BTreeMap::new(),
        }
    }

    pub fn evaluate(&mut self, input: &str) -> EvalResult {
        let _ = krust::parse_expr(input);

        Ok(())
    }

    pub fn names(&self) -> Vec<&str> {
        return self.names.keys().map(|s| s.as_str()).collect()
    } 
}

pub type EvalResult = Result<(), EvalError>;

pub enum EvalError {
    Parse(krust::ParseError),
    Name
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
