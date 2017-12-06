//! parsing using pest for grammar
use std::error::Error;
use std::fmt;
use super::expr::Expr;

use pest::Parser;

#[cfg(debug_assertions)]
const _GRAMMAR: &'static str = include_str!("kgrammar.pest"); // relative to this file

#[derive(Parser)]
#[grammar = "krust/kgrammar.pest"] // relative to src
struct KParser;


#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct ParseError {
    span: usize,
    kind: ParseErrorKind,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum ParseErrorKind {
    Ty((), ()),
    Unknown,
}

impl<R, I: ::pest::inputs::Input> From< ::pest::Error<R, I>> for ParseError {
    fn from(_input: ::pest::Error<R, I>) -> Self {
        ParseError::new(0, ParseErrorKind::Unknown)
    }
}

impl ParseError {
    fn new(span: usize, kind: ParseErrorKind) -> Self {
        ParseError {
            span,
            kind
        }
    }

    fn __description(&self) -> String {
        match self.kind {
            ParseErrorKind::Ty(a, b) => format!("invalid type: expected `{:?}`, found `{:?}`", a, b),
            _        => "unknown input character".to_string(),
        }
    }

    pub fn span(&self) -> usize {
        self.span
    } 
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.__description().fmt(f)
    }
}

impl Error for ParseError {
    fn description(&self) -> &str {
        "Parse Error"
    }
}

pub fn parse_expr(input: &str) -> Result<Expr, ParseError> {
    let pairs = KParser::parse_str(Rule::outer_block, input)?;
    for pair in pairs.flatten() {
        println!("Rule:    {:?}", pair.as_rule());
        println!("Span:    {:?}", pair.clone().into_span());
        println!("Text:    {}", pair.clone().into_span().as_str());
    }

    Ok(Expr::Nil)
}


