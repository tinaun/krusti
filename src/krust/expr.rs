/// AST

use super::Item;

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Assign(Name, Box<Expr>),
    Item(Item),
}

pub type Block = Vec<Expr>;
pub type Name = String;