use crate::token;

pub struct Position {
    pub row: usize,
    pub col: usize,
}

pub struct Location {
    pub start: Position,
    pub end: Position,
}

pub struct Program {
    body: Vec<Box<AstNode>>,
}
pub trait AstNode {}
pub struct WithExpression {
    location: Location,


}
