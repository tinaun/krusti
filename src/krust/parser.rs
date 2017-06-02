///parsing
use std::error::Error;
use std::fmt;
use std::f64;

use nom::{double_s, digit, space};
use super::{Item, Ty, Primitive, Integer, Float};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum ParseError {
    Ty(Ty, Ty),
    UnknownChar,
}

type ParseResult = Result<Item, ParseError>;

impl ParseError {
    fn __description(&self) -> String {
        match *self {
            ParseError::Ty(a, b) => format!("invalid type: expected `{}`, found `{}`", a, b),
            _        => "unknown input character".to_string(),
        }
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
    match parse_vector(input).to_result() {
        Ok(e) => e,
        Err(_) => Err(ParseError::UnknownChar)
    }
}

fn is_name_start(chr: char) -> bool {
    chr.is_alphabetic() || chr == '.'
}

fn is_name_continue(chr: char) -> bool {
    chr.is_alphanumeric() || chr == '.'
}


named!(parse_symbol<&str, String>, do_parse!(
    tag_s!("`") >> 
    name: opt!(complete!(recognize!(do_parse!(
        take_while1_s!(is_name_start) >>
        take_while_s!(is_name_continue) >>
        ()
    )))) >> (name.unwrap_or("").to_string())
));

named!(parse_int<&str, Integer>, alt_complete!(
    map!(tag_s!("0N"), |_| Integer(None)) |
    map_res!(recognize!(tuple!(
        opt!(alt!(tag_s!("+") | tag_s!("-"))),
        digit
    )), |s: &str| s.parse().map(|i| Integer(Some(i)) ))
));

named!(parse_float<&str, Float>, alt_complete!(
    map!(tag_s!("0n"), |_| f64::NAN) |
    map!(tag_s!("0w"), |_| f64::INFINITY) |
    double_s
));

named!(parse_primitive<&str, Primitive>, alt_complete!(
        map!(parse_symbol, Primitive::from) |
        map!(parse_float, Primitive::from) |
        map!(parse_int, Primitive::from)  
    )
);

named!(parse_vector<&str, ParseResult>, map!(many1!(do_parse!(
    opt!(space) >>
    prim: parse_primitive >>
    (prim)
)), |v: Vec<Primitive>| {
    if v.len() == 1 {
        Ok(Item::Unit(v[0].clone()))
    } else {
        let ty = v[0].ty();

        if let Some(bad_ty) = v.iter().find(|&p| p.ty() != ty) {
            Err(ParseError::Ty(ty, bad_ty.ty()))
        } else {
            Ok(Item::Vector(ty, v.clone()))
        }
    }
}
));
