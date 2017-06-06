//! an interpreter for a k5-k6 ish language
//! heavily inspired by JohnEarnest's oK
#![recursion_limit = "256"]

#[macro_use]
extern crate pest;

pub mod krust;

use std::collections::BTreeMap;

use krust::Item;
use krust::expr::{Expr, Name};

#[derive(Debug, PartialEq)]
pub struct KrustI {
    names: BTreeMap<Name, Item>
}

/// k interpreter
impl KrustI {
    pub fn new() -> Self {
        KrustI {
            names: BTreeMap::new(),
        }
    }

    pub fn evaluate(&mut self, input: &str) -> EvalResult {
        unimplemented!()
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
