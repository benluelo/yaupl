use crate::{
    combinators::csv,
    parse_error::ParseError,
    pointer::Pointer,
    tokens::{arrow_right_thick, brace_square_close, brace_square_open},
    types::{yaupl_type, Type},
};

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
pub struct Function {
    parameters: Vec<Type>,
    return_type: Box<Type>,
}

/// REFACTOR
/// TODO: can't rely on the tuple function anymore since the tuples have a different syntax (`[| |]` vs `[ ]`)
pub(crate) fn function(i: &str, ptr: Pointer) -> Result<(&str, Pointer, Type), ParseError> {
    let (i, ptr, _bracket_open) = brace_square_open(i, ptr)?;
    let (i, ptr, parameters) = csv(i, ptr, &yaupl_type)?;
    let (i, ptr, _bracket_close) = brace_square_close(i, ptr)?;

    let (i, ptr, _arrow) = arrow_right_thick(i, ptr)?;

    let (i, ptr, return_type) = yaupl_type(i, ptr)?;

    Ok((
        i,
        ptr,
        Type::Function(Function {
            parameters,
            return_type: Box::new(return_type),
        }),
    ))
}
