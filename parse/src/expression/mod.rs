use crate::{
    expression::{
        binary_operations::BinaryOperation, function_call::FunctionCall, literal::Literal,
    },
    parse_error::ParseError,
    pointer::Pointer,
};

pub(crate) mod binary_operations;
pub(crate) mod function_call;
pub(crate) mod literal;

pub enum Expression {
    BinaryOperation(BinaryOperation),
    FunctionCall(FunctionCall),
    Literal(Literal),
}

pub(crate) fn expression(i: &str, ptr: Pointer) -> Result<(&str, Pointer, Expression), ParseError> {
    todo!()
}
