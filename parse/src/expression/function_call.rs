use crate::{expression::Expression, parse_error::ParseError, pointer::Pointer, Identifier};

pub(crate) fn function_call(
    i: &str,
    ptr: Pointer,
) -> Result<(&str, Pointer, FunctionCall), ParseError> {
    todo!()
}
pub struct FunctionCall {
    name: Identifier,
    args: Vec<Expression>,
}
