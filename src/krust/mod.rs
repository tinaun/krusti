//! Data Structures
use std::fmt;
use std::ops::Deref;

mod parser;

pub use self::parser::{ParseError, parse_item};

/// k-internal data types
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Ty {
    Symbol,
    Int,
    Float,
    Bool,
    Char,
    List,
    Unknown,
}

impl fmt::Display for Ty {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            e => write!(f, "{:?}", e)
        }
    }
}

/// an `Item` is everything that can be the result of a valid computation
#[derive(Debug, Clone, PartialEq)]
pub enum Item {
    Unit(Primitive),
    Vector(Ty, Vec<Primitive>),
    List(Vec<Item>),
    Nil,
}

impl Item {
    /// the length of the item
    pub fn len(&self) -> usize {
        match *self {
            Item::Unit(_) => 1,
            Item::Vector(_, ref v) => v.len(),
            Item::List(ref v) => v.len(),
            _ => 0,
        }
    }

    /// the dimension of the item (how many nested sublists)
    pub fn depth(&self) -> usize {
        match *self {
            Item::Vector(_, ref v) if v.len() > 0 => 1,
            Item::List(ref v) if v.len() > 0 => {
                v.iter().map(|i| i.depth()).max().unwrap_or(0) + 1   
            },
            _ => 0,
        }
    }
}

impl IntoIterator for Item {
    type Item = Item;
    type IntoIter = IntoIter;
    fn into_iter(self) -> Self::IntoIter {
        match self {
            e @ Item::Unit(_) => {
                IntoIter {
                    items: vec![e],
                }
            },
            Item::Vector(_, plist) => {
                IntoIter {
                    items: plist.into_iter().map(|p| Item::Unit(p)).collect()
                }
            },
            Item::List(list) => {
                IntoIter {
                    items: list,
                }
            },
            _ => {
                IntoIter {
                    items: vec![],
                } 
            },
        }
    }
} 

impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Item::Unit(ref p) => write!(f, "{}", p)?,
            Item::Vector(Ty::Char, ref s) => {
                write!(f, "\"{}\"", s.iter().map(|p| {
                    match *p {
                        Primitive::Char(c) => c,
                        ref n => panic!(r#"expected `char`, found `{}`.
                        this shouldn't happen in the pretty-printer!"#, n.ty()),
                    } 
                }).collect::<String>())?;
            },
            Item::Vector(ty, ref v) if v.len() > 0 => {
                match ty {
                    Ty::Symbol => {
                        write!(f,"{}", v[0])?;
                        for val in &v[1..] {
                           write!(f, "{}", val)?;
                        }                         
                    },
                    Ty::Bool => {
                        for val in v {
                           write!(f, "{}", val)?;
                        }
                        write!(f, "b")?; 
                    },
                    _ => {
                        write!(f,"{}", v[0])?;
                        for val in &v[1..] {
                            write!(f, " {}", val)?;
                        } 
                    }
                }
            },
            Item::List(ref l) => {
                write!(f, "(")?;
                if l.len() > 0 {
                    write!(f, "{}", l[0])?;
                    for val in &l[1..] {
                        write!(f, "\n{}", val)?;
                    }
                }

                write!(f, ")")?;
            },
            _ => (),
        } 

        Ok(())
    }
}

/// primitive types
#[derive(Debug, Clone, PartialEq)]
pub enum Primitive {
    Symbol(String),
    Int(Integer),
    Float(Float),
    Bool(bool),
    Char(char),
}

impl Primitive {
    pub fn ty(&self) -> Ty {
        match *self {
            Primitive::Symbol(_) => Ty::Symbol,
            Primitive::Int(_) => Ty::Int,
            Primitive::Float(_) => Ty::Float,
            Primitive::Bool(_) => Ty::Char,
            Primitive::Char(_) => Ty::Bool,
        }
    }
}

macro_rules! from_impl {
    ($from:ty, $to:tt :: $var:tt) => {
        impl From<$from> for $to {
            fn from(f: $from) -> $to {
                $to::$var(f)
            }
        }
    }
}

from_impl!(String, Primitive::Symbol);
from_impl!(Float, Primitive::Float);
from_impl!(Integer, Primitive::Int);
from_impl!(bool, Primitive::Bool);
from_impl!(char, Primitive::Char);

impl fmt::Display for Primitive {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Primitive::Symbol(ref s) => {
                write!(f, "`{}", s)
            },
            Primitive::Int(ref i) => {
                write!(f, "{}", i)   
            },
            Primitive::Float(fl) => {
                if fl.is_nan() {
                    write!(f, "0n")
                } else if fl.is_infinite() {
                    write!(f, "0w")
                } else {
                    write!(f, "{}", fl)
                }
            },
            Primitive::Bool(b) => {
                if b {
                    write!(f, "1")
                } else {
                    write!(f, "0")
                }
            },
            Primitive::Char(c) => {
                write!(f, "\"{}\"", c)   
            },
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Integer(Option<i64>);

impl Deref for Integer {
    type Target = Option<i64>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl fmt::Display for Integer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.0 {
            Some(int) => {
                write!(f, "{}", int)
            },
            None => {
                write!(f, "0N")
            }
        }
    }
}

impl From<i64> for Integer {
    fn from(f: i64) -> Integer {
        Integer(Some(f))
    }
}

pub type Float = f64;

pub struct IntoIter {
    items: Vec<Item>,
}

impl Iterator for IntoIter {
    type Item = Item;
    fn next(&mut self) -> Option<Self::Item> {
        match self.items.len() {
            0 => None,
            _ => {
                let next = self.items.remove(0);
                Some(next)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Item, Primitive, Integer, Ty, parse_item, ParseError};

    #[test]
    fn parse_primitives() {

        //symbols
        assert_eq!(parse_item("`"), 
                   Ok( Item::Unit( Primitive::Symbol( "".to_string() ) ) ) );
        assert_eq!(parse_item("`www2.crates.io"), 
                   Ok( Item::Unit( Primitive::Symbol( "www2.crates.io".to_string() ) ) ) );
        
        //integers
        assert_eq!(parse_item("65535"), 
                   Ok( Item::Unit( Primitive::Int( Integer( Some(65535) ) ) ) ) );
        assert_eq!(parse_item("-65535"), 
                   Ok( Item::Unit( Primitive::Int( Integer( Some(-65535) ) ) ) ) );
        assert_eq!(parse_item("0N"), 
                   Ok( Item::Unit( Primitive::Int( Integer( None ) ) ) ) );

        assert_eq!(parse_item("2.564"), 
                   Ok( Item::Unit( Primitive::Float( 2.564 ) ) ) );
    }

    #[test]
    fn parse_lists() {
        //symbols
        assert_eq!(parse_item("`a `b `c").unwrap().len(), 3);
        assert_eq!(parse_item("`a 2 3"), 
                   Err( ParseError::Ty(Ty::Symbol, Ty::Int) ) );
    }
}