//! AST

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Assign(Name, Train),
    Train(Train),
    Nil,
}

pub type Train = Vec<KValue>;
pub type Block = Vec<Expr>;
pub type Name = String;

#[derive(Debug, Clone, PartialEq)]
pub enum Vector<T> {
    Single(T),
    Multiple(Vec<T>),
}

pub type KValue = String;


