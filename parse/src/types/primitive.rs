use crate::{
    combinators::one_of,
    parse_error::ParseError,
    pointer::Pointer,
    tokens::{
        keyword_bln, keyword_emp, keyword_num, keyword_str, token::Token, KeywordBln, KeywordEmp,
        KeywordNum, KeywordStr,
    },
};

use super::Type;

// REFACTOR: make the enum variants tuple structs containing their respective tokens
#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
pub enum PrimitiveType {
    Str,
    Bln,
    Num,
    Emp,
}

impl Token for PrimitiveType {
    fn token(&self) -> &str {
        match self {
            PrimitiveType::Str => KeywordStr.token(),
            PrimitiveType::Bln => KeywordBln.token(),
            PrimitiveType::Num => KeywordNum.token(),
            PrimitiveType::Emp => KeywordEmp.token(),
        }
    }
}

pub(crate) fn primitive<'a>(
    i: &'a str,
    ptr: Pointer,
) -> Result<(&'a str, Pointer, Type), ParseError> {
    one_of(
        i,
        ptr,
        &[&keyword_str, &keyword_num, &keyword_bln, &keyword_emp],
    )
    .map(|res| (res.0, res.1, Type::Primitive(res.2)))
}

#[cfg(test)]
mod test_types {
    use super::*;

    #[test]
    fn test_primitive() {
        assert_eq!(
            primitive("    str,  not_str", Pointer { row: 0, col: 0 }).unwrap(),
            (
                ",  not_str",
                Pointer { row: 0, col: 7 },
                Type::Primitive(PrimitiveType::Str),
            )
        );
    }
}
