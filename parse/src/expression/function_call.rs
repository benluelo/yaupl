use crate::{expression::Expression, Identifier};

pub struct FunctionCall {
    name: Identifier,
    args: Vec<Expression>,
}
