///parsing
use std::error::Error;
use std::fmt;
use std::f64;

use super::{Item, Ty, Primitive, Integer, Float};
use pest::prelude::*;

impl_rdp! {
    grammar! {
        block   = _{ (block_expr ~ [";"] | void )* ~ block_expr ~ [";"]? }
        list    = { ["("] ~ block? ~ [")"] | int ~ int+ }
            
        noun = { list | atom | ident }
        dyadic = { noun ~ verb ~ expression } 
        
        expression = _{ dyadic | noun }
        assign  = { ident ~ [":"] ~ expression }

        block_expr = _{ assign | expression }

        //atoms
        symbol  = @{ ["`"] ~ (sym_start ~ sym_continue* )* }
        char    = @{ ["\""] ~ (!["\""] ~ any) ~ ["\""] } 
        int     = @{ null | (["-"] | ["+"])? ~ intseq }
        float   = @{ nan | inf | (["-"] | ["+"])? ~ intseq? ~ ["."] ~ ['0'..'9']+ }
        boollist    = @{ ['0'..'1']+ ~ ["b"] } 
        hexlist     = @{ ["0x"] ~ (hex ~ hex)+ }
        string      = @{ ["\""] ~ (!["\""] ~ any)* ~ ["\""] }

        atom    = { symbol | char | int | float | boollist | hexlist | string }
        
        ident   = @{ alpha ~ alphanum* }

        verb    = @{ ["+"] | ["-"] | ["*"] | ["%"] }
       
        intseq  = _{ ["0"] | ['1'..'9'] ~ ['0'..'9']* }
        hex     = _{ ['0'..'9'] | ['a'..'f'] | ['A'..'F'] }
        null    =  { ["0N"] }
        nan     =  { ["0n"] }
        inf     =  { ["0f"] }
        void    =  { [";"] }
        
        sym_start = _{ alpha | ["."] }
        sym_continue = _{ alphanum | ["."] }
        alpha = _{ ['a'..'z'] | ['A'..'Z'] }
        alphanum = _{ ['a'..'z'] | ['A'..'Z'] | ['0'..'9'] }


        whitespace = _{ [" "] | ["\t"] }
    }
}


#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct ParseError {
    span: usize,
    kind: ParseErrorKind,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum ParseErrorKind {
    Ty(Ty, Ty),
    UnknownChar,
}

type ParseResult = Result<Item, ParseError>;

impl ParseError {
    fn new(span: usize, kind: ParseErrorKind) -> Self {
        ParseError {
            span,
            kind
        }
    }

    fn __description(&self) -> String {
        match self.kind {
            ParseErrorKind::Ty(a, b) => format!("invalid type: expected `{}`, found `{}`", a, b),
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

pub fn parse_item(input: &str) -> Result<Item, ParseError> {
    let mut parser = Rdp::new(StringInput::new(input));

    parser.block();

    for tok in parser.queue() {
        let dist = tok.end - tok.start;

        if dist > 1 {
        println!("   {: >start$}{:^>end$} - {:?}", "", "", 
                 tok.rule, start = tok.start, end = dist);
        } else {
        println!("   {: >start$} - {:?}", '^', 
                 tok.rule, start = tok.end);
        }
    }

    Ok(Item::Nil)
}


