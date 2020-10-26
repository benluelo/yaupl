// use crate::parse::{pointer::Pointer, ParseError, types::primitive::PrimitiveType};

use crate::{parse_error::ParseError, pointer::Pointer, types::primitive::PrimitiveType};

use super::whitespace::*;
pub(crate) mod token;
use token::*;
/// ```yaupl
/// +
/// ```
#[derive(Debug, Default, Copy, Clone)]
pub(crate) struct BinaryOperatorAdd;
impl Token for BinaryOperatorAdd {
    fn token(&self) -> &str {
        "+"
    }
}
pub(crate) fn binary_operator_add(
    i: &str,
    ptr: Pointer,
) -> Result<(&str, Pointer, BinaryOperatorAdd), ParseError> {
    let (i, ptr) = whitespace(i, ptr);
    if i.starts_with("+") {
        Ok((&i["+".len()..], ptr.add_col("+".len()), BinaryOperatorAdd))
    } else {
        Err(ParseError::Expected(Box::new(BinaryOperatorAdd)))
    }
}

/// ```yaupl
/// -
/// ```
#[derive(Debug, Default, Copy, Clone)]
pub(crate) struct BinaryOperatorSub;
impl Token for BinaryOperatorSub {
    fn token(&self) -> &str {
        "-"
    }
}
pub(crate) fn binary_operator_sub(
    i: &str,
    ptr: Pointer,
) -> Result<(&str, Pointer, BinaryOperatorSub), ParseError> {
    let (i, ptr) = whitespace(i, ptr);
    if i.starts_with("-") {
        Ok((&i["-".len()..], ptr.add_col("-".len()), BinaryOperatorSub))
    } else {
        Err(ParseError::Expected(Box::new(BinaryOperatorSub)))
    }
}

/// ```yaupl
/// *
/// ```
#[derive(Debug, Default, Copy, Clone)]
pub(crate) struct BinaryOperatorMul;
impl Token for BinaryOperatorMul {
    fn token(&self) -> &str {
        "*"
    }
}
pub(crate) fn binary_operator_mul(
    i: &str,
    ptr: Pointer,
) -> Result<(&str, Pointer, BinaryOperatorMul), ParseError> {
    let (i, ptr) = whitespace(i, ptr);
    if i.starts_with("*") {
        Ok((&i["*".len()..], ptr.add_col("*".len()), BinaryOperatorMul))
    } else {
        Err(ParseError::Expected(Box::new(BinaryOperatorMul)))
    }
}

/// ```yaupl
/// /
/// ```
#[derive(Debug, Default, Copy, Clone)]
pub(crate) struct BinaryOperatorDiv;
impl Token for BinaryOperatorDiv {
    fn token(&self) -> &str {
        "/"
    }
}
pub(crate) fn binary_operator_div(
    i: &str,
    ptr: Pointer,
) -> Result<(&str, Pointer, BinaryOperatorDiv), ParseError> {
    let (i, ptr) = whitespace(i, ptr);
    if i.starts_with("/") {
        Ok((&i["/".len()..], ptr.add_col("/".len()), BinaryOperatorDiv))
    } else {
        Err(ParseError::Expected(Box::new(BinaryOperatorDiv)))
    }
}

/// ```yaupl
/// >
/// ```
#[derive(Debug, Default, Copy, Clone)]
pub(crate) struct BinaryOperatorGt;
impl Token for BinaryOperatorGt {
    fn token(&self) -> &str {
        ">"
    }
}
pub(crate) fn binary_operator_gt(
    i: &str,
    ptr: Pointer,
) -> Result<(&str, Pointer, BinaryOperatorGt), ParseError> {
    let (i, ptr) = whitespace(i, ptr);
    if i.starts_with(">") {
        Ok((&i[">".len()..], ptr.add_col(">".len()), BinaryOperatorGt))
    } else {
        Err(ParseError::Expected(Box::new(BinaryOperatorGt)))
    }
}

/// ```yaupl
/// <
/// ```
#[derive(Debug, Default, Copy, Clone)]
pub(crate) struct BinaryOperatorLt;
impl Token for BinaryOperatorLt {
    fn token(&self) -> &str {
        "<"
    }
}
pub(crate) fn binary_operator_lt(
    i: &str,
    ptr: Pointer,
) -> Result<(&str, Pointer, BinaryOperatorLt), ParseError> {
    let (i, ptr) = whitespace(i, ptr);
    if i.starts_with("<") {
        Ok((&i["<".len()..], ptr.add_col("<".len()), BinaryOperatorLt))
    } else {
        Err(ParseError::Expected(Box::new(BinaryOperatorLt)))
    }
}

/// ```yaupl
/// >=
/// ```
#[derive(Debug, Default, Copy, Clone)]
pub(crate) struct BinaryOperatorGte;
impl Token for BinaryOperatorGte {
    fn token(&self) -> &str {
        ">="
    }
}
pub(crate) fn binary_operator_gte(
    i: &str,
    ptr: Pointer,
) -> Result<(&str, Pointer, BinaryOperatorGte), ParseError> {
    let (i, ptr) = whitespace(i, ptr);
    if i.starts_with(">=") {
        Ok((&i[">=".len()..], ptr.add_col(">=".len()), BinaryOperatorGte))
    } else {
        Err(ParseError::Expected(Box::new(BinaryOperatorGte)))
    }
}

/// ```yaupl
/// <=
/// ```
#[derive(Debug, Default, Copy, Clone)]
pub(crate) struct BinaryOperatorLte;
impl Token for BinaryOperatorLte {
    fn token(&self) -> &str {
        "<="
    }
}
pub(crate) fn binary_operator_lte(
    i: &str,
    ptr: Pointer,
) -> Result<(&str, Pointer, BinaryOperatorLte), ParseError> {
    let (i, ptr) = whitespace(i, ptr);
    if i.starts_with("<=") {
        Ok((&i["<=".len()..], ptr.add_col("<=".len()), BinaryOperatorLte))
    } else {
        Err(ParseError::Expected(Box::new(BinaryOperatorLte)))
    }
}

/// ```yaupl
/// ==
/// ```
#[derive(Debug, Default, Copy, Clone)]
pub(crate) struct BinaryOperatorEq;
impl Token for BinaryOperatorEq {
    fn token(&self) -> &str {
        "=="
    }
}
pub(crate) fn binary_operator_eq(
    i: &str,
    ptr: Pointer,
) -> Result<(&str, Pointer, BinaryOperatorEq), ParseError> {
    let (i, ptr) = whitespace(i, ptr);
    if i.starts_with("==") {
        Ok((&i["==".len()..], ptr.add_col("==".len()), BinaryOperatorEq))
    } else {
        Err(ParseError::Expected(Box::new(BinaryOperatorEq)))
    }
}

/// ```yaupl
/// !=
/// ```
#[derive(Debug, Default, Copy, Clone)]
pub(crate) struct BinaryOperatorNeq;
impl Token for BinaryOperatorNeq {
    fn token(&self) -> &str {
        "!="
    }
}
pub(crate) fn binary_operator_neq(
    i: &str,
    ptr: Pointer,
) -> Result<(&str, Pointer, BinaryOperatorNeq), ParseError> {
    let (i, ptr) = whitespace(i, ptr);
    if i.starts_with("!=") {
        Ok((&i["!=".len()..], ptr.add_col("!=".len()), BinaryOperatorNeq))
    } else {
        Err(ParseError::Expected(Box::new(BinaryOperatorNeq)))
    }
}

/// ```yaupl
/// oo
/// ```
#[derive(Debug, Default, Copy, Clone)]
pub(crate) struct Infinity;
impl Token for Infinity {
    fn token(&self) -> &str {
        "oo"
    }
}
pub(crate) fn infinity(i: &str, ptr: Pointer) -> Result<(&str, Pointer, Infinity), ParseError> {
    let (i, ptr) = whitespace(i, ptr);
    if i.starts_with("oo") {
        Ok((&i["oo".len()..], ptr.add_col("oo".len()), Infinity))
    } else {
        Err(ParseError::Expected(Box::new(Infinity)))
    }
}

/// ```yaupl
/// -oo
/// ```
#[derive(Debug, Default, Copy, Clone)]
pub(crate) struct NegativeInfinity;
impl Token for NegativeInfinity {
    fn token(&self) -> &str {
        "-oo"
    }
}
pub(crate) fn negative_infinity(
    i: &str,
    ptr: Pointer,
) -> Result<(&str, Pointer, NegativeInfinity), ParseError> {
    let (i, ptr) = whitespace(i, ptr);
    if i.starts_with("-oo") {
        Ok((
            &i["-oo".len()..],
            ptr.add_col("-oo".len()),
            NegativeInfinity,
        ))
    } else {
        Err(ParseError::Expected(Box::new(NegativeInfinity)))
    }
}

/// ```yaupl
/// true
/// ```
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub(crate) struct KeywordTrue;
impl Token for KeywordTrue {
    fn token(&self) -> &str {
        "true"
    }
}
pub(crate) fn keyword_true(
    i: &str,
    ptr: Pointer,
) -> Result<(&str, Pointer, KeywordTrue), ParseError> {
    let (i, ptr) = whitespace(i, ptr);
    if i.starts_with("true") {
        Ok((&i["true".len()..], ptr.add_col("true".len()), KeywordTrue))
    } else {
        Err(ParseError::Expected(Box::new(KeywordTrue)))
    }
}

/// ```yaupl
/// false
/// ```
#[derive(Debug, Default, Copy, Clone)]
pub(crate) struct KeywordFalse;
impl Token for KeywordFalse {
    fn token(&self) -> &str {
        "false"
    }
}
pub(crate) fn keyword_false(
    i: &str,
    ptr: Pointer,
) -> Result<(&str, Pointer, KeywordFalse), ParseError> {
    let (i, ptr) = whitespace(i, ptr);
    if i.starts_with("false") {
        Ok((
            &i["false".len()..],
            ptr.add_col("false".len()),
            KeywordFalse,
        ))
    } else {
        Err(ParseError::Expected(Box::new(KeywordFalse)))
    }
}

/// ```yaupl
/// [
/// ```
#[derive(Debug, Default, Copy, Clone)]
pub(crate) struct BraceSquareOpen;
impl Token for BraceSquareOpen {
    fn token(&self) -> &str {
        "["
    }
}
pub(crate) fn brace_square_open(
    i: &str,
    ptr: Pointer,
) -> Result<(&str, Pointer, BraceSquareOpen), ParseError> {
    let (i, ptr) = whitespace(i, ptr);
    if i.starts_with("[") {
        Ok((&i["[".len()..], ptr.add_col("[".len()), BraceSquareOpen))
    } else {
        Err(ParseError::Expected(Box::new(BraceSquareOpen)))
    }
}

/// ```yaupl
/// ]
/// ```
#[derive(Debug, Default, Copy, Clone)]
pub(crate) struct BraceSquareClose;
impl Token for BraceSquareClose {
    fn token(&self) -> &str {
        "]"
    }
}
pub(crate) fn brace_square_close(
    i: &str,
    ptr: Pointer,
) -> Result<(&str, Pointer, BraceSquareClose), ParseError> {
    let (i, ptr) = whitespace(i, ptr);
    if i.starts_with("]") {
        Ok((&i["]".len()..], ptr.add_col("]".len()), BraceSquareClose))
    } else {
        Err(ParseError::Expected(Box::new(BraceSquareClose)))
    }
}

/// ```yaupl
/// {
/// ```
#[derive(Debug, Default, Copy, Clone)]
pub(crate) struct BraceCurlyOpen;
impl Token for BraceCurlyOpen {
    fn token(&self) -> &str {
        "{"
    }
}
pub(crate) fn brace_curly_open(
    i: &str,
    ptr: Pointer,
) -> Result<(&str, Pointer, BraceCurlyOpen), ParseError> {
    let (i, ptr) = whitespace(i, ptr);
    if i.starts_with("{") {
        Ok((&i["{".len()..], ptr.add_col("{".len()), BraceCurlyOpen))
    } else {
        Err(ParseError::Expected(Box::new(BraceCurlyOpen)))
    }
}

/// ```yaupl
/// }
/// ```
#[derive(Debug, Default, Copy, Clone)]
pub(crate) struct BraceCurlyClose;
impl Token for BraceCurlyClose {
    fn token(&self) -> &str {
        "}"
    }
}
pub(crate) fn brace_curly_close(
    i: &str,
    ptr: Pointer,
) -> Result<(&str, Pointer, BraceCurlyClose), ParseError> {
    let (i, ptr) = whitespace(i, ptr);
    if i.starts_with("}") {
        Ok((&i["}".len()..], ptr.add_col("}".len()), BraceCurlyClose))
    } else {
        Err(ParseError::Expected(Box::new(BraceCurlyClose)))
    }
}

/// ```yaupl
/// (|
/// ```
#[derive(Debug, Default, Copy, Clone)]
pub(crate) struct BraceGroupOpen;
impl Token for BraceGroupOpen {
    fn token(&self) -> &str {
        "(|"
    }
}
pub(crate) fn brace_group_open(
    i: &str,
    ptr: Pointer,
) -> Result<(&str, Pointer, BraceGroupOpen), ParseError> {
    let (i, ptr) = whitespace(i, ptr);
    if i.starts_with("(|") {
        Ok((&i["(|".len()..], ptr.add_col("(|".len()), BraceGroupOpen))
    } else {
        Err(ParseError::Expected(Box::new(BraceGroupOpen)))
    }
}

/// ```yaupl
/// |)
/// ```
#[derive(Debug, Default, Copy, Clone)]
pub(crate) struct BraceGroupClose;
impl Token for BraceGroupClose {
    fn token(&self) -> &str {
        "|)"
    }
}
pub(crate) fn brace_group_close(
    i: &str,
    ptr: Pointer,
) -> Result<(&str, Pointer, BraceGroupClose), ParseError> {
    let (i, ptr) = whitespace(i, ptr);
    if i.starts_with("|)") {
        Ok((&i["|)".len()..], ptr.add_col("|)".len()), BraceGroupClose))
    } else {
        Err(ParseError::Expected(Box::new(BraceGroupClose)))
    }
}

/// ```yaupl
/// |-
/// ```
#[derive(Debug, Default, Copy, Clone)]
pub(crate) struct TeslaOpen;
impl Token for TeslaOpen {
    fn token(&self) -> &str {
        "|-"
    }
}
pub(crate) fn tesla_open(i: &str, ptr: Pointer) -> Result<(&str, Pointer, TeslaOpen), ParseError> {
    let (i, ptr) = whitespace(i, ptr);
    if i.starts_with("|-") {
        Ok((&i["|-".len()..], ptr.add_col("|-".len()), TeslaOpen))
    } else {
        Err(ParseError::Expected(Box::new(TeslaOpen)))
    }
}

/// ```yaupl
/// -|
/// ```
#[derive(Debug, Default, Copy, Clone)]
pub(crate) struct TeslaClose;
impl Token for TeslaClose {
    fn token(&self) -> &str {
        "-|"
    }
}
pub(crate) fn tesla_close(
    i: &str,
    ptr: Pointer,
) -> Result<(&str, Pointer, TeslaClose), ParseError> {
    let (i, ptr) = whitespace(i, ptr);
    if i.starts_with("-|") {
        Ok((&i["-|".len()..], ptr.add_col("-|".len()), TeslaClose))
    } else {
        Err(ParseError::Expected(Box::new(TeslaClose)))
    }
}

/// ```yaupl
/// |-
/// ```
#[derive(Debug, Default, Copy, Clone)]
pub(crate) struct TupleOpen;
impl Token for TupleOpen {
    fn token(&self) -> &str {
        "[|"
    }
}
pub(crate) fn tuple_open(i: &str, ptr: Pointer) -> Result<(&str, Pointer, TeslaOpen), ParseError> {
    let (i, ptr) = whitespace(i, ptr);
    if i.starts_with("[|") {
        Ok((&i["[|".len()..], ptr.add_col("[|".len()), TeslaOpen))
    } else {
        Err(ParseError::Expected(Box::new(TeslaOpen)))
    }
}

/// ```yaupl
/// -|
/// ```
#[derive(Debug, Default, Copy, Clone)]
pub(crate) struct TupleClose;
impl Token for TupleClose {
    fn token(&self) -> &str {
        "|]"
    }
}
pub(crate) fn tuple_close(
    i: &str,
    ptr: Pointer,
) -> Result<(&str, Pointer, TeslaClose), ParseError> {
    let (i, ptr) = whitespace(i, ptr);
    if i.starts_with("|]") {
        Ok((
            &i[TupleClose.token().len()..],
            ptr.add_col("|]".len()),
            TeslaClose,
        ))
    } else {
        Err(ParseError::Expected(Box::new(TeslaClose)))
    }
}

/// ```yaupl
/// <-
/// ```
#[derive(Debug, Default, Copy, Clone)]
pub(crate) struct ArrowLeft;
impl Token for ArrowLeft {
    fn token(&self) -> &str {
        "<-"
    }
}
pub(crate) fn arrow_left(i: &str, ptr: Pointer) -> Result<(&str, Pointer, ArrowLeft), ParseError> {
    let (i, ptr) = whitespace(i, ptr);
    if i.starts_with("<-") {
        let (i, ptr) = whitespace(i, ptr);
        Ok((&i["<-".len()..], ptr.add_col("<-".len()), ArrowLeft))
    } else {
        Err(ParseError::Expected(Box::new(ArrowLeft)))
    }
}

/// ```yaupl
/// ->
/// ```
#[derive(Debug, Default, Copy, Clone)]
pub(crate) struct ArrowRight;
impl Token for ArrowRight {
    fn token(&self) -> &str {
        "->"
    }
}
pub(crate) fn arrow_right(
    i: &str,
    ptr: Pointer,
) -> Result<(&str, Pointer, ArrowRight), ParseError> {
    let (i, ptr) = whitespace(i, ptr);
    if i.starts_with("->") {
        Ok((&i["->".len()..], ptr.add_col("->".len()), ArrowRight))
    } else {
        Err(ParseError::Expected(Box::new(ArrowRight)))
    }
}

/// ```yaupl
/// =>
/// ```
#[derive(Debug, Default, Copy, Clone)]
pub(crate) struct ArrowRightThick;
impl Token for ArrowRightThick {
    fn token(&self) -> &str {
        "=>"
    }
}
pub(crate) fn arrow_right_thick(
    i: &str,
    ptr: Pointer,
) -> Result<(&str, Pointer, ArrowRightThick), ParseError> {
    let (i, ptr) = whitespace(i, ptr);
    if i.starts_with("=>") {
        Ok((&i["=>".len()..], ptr.add_col("=>".len()), ArrowRightThick))
    } else {
        Err(ParseError::Expected(Box::new(ArrowRightThick)))
    }
}

/// ```yaupl
/// ~>
/// ```
#[derive(Debug, Default, Copy, Clone)]
pub(crate) struct ArrowRightCurly;
impl Token for ArrowRightCurly {
    fn token(&self) -> &str {
        "~>"
    }
}
pub(crate) fn arrow_right_curly(
    i: &str,
    ptr: Pointer,
) -> Result<(&str, Pointer, ArrowRightCurly), ParseError> {
    let (i, ptr) = whitespace(i, ptr);
    if i.starts_with("~>") {
        Ok((&i["~>".len()..], ptr.add_col("~>".len()), ArrowRightCurly))
    } else {
        Err(ParseError::Expected(Box::new(ArrowRightCurly)))
    }
}

/// ```yaupl
/// #[
/// ```
#[derive(Debug, Default, Copy, Clone)]
pub(crate) struct CommentOpen;
impl Token for CommentOpen {
    fn token(&self) -> &str {
        "#["
    }
}
pub(crate) fn comment_open(
    i: &str,
    ptr: Pointer,
) -> Result<(&str, Pointer, CommentOpen), ParseError> {
    let (i, ptr) = whitespace(i, ptr);
    if i.starts_with("#[") {
        Ok((&i["#[".len()..], ptr.add_col("#[".len()), CommentOpen))
    } else {
        Err(ParseError::Expected(Box::new(CommentOpen)))
    }
}

/// ```yaupl
/// !!#[
/// ```
#[derive(Debug, Default, Copy, Clone)]
pub(crate) struct DocCommentOpen;
impl Token for DocCommentOpen {
    fn token(&self) -> &str {
        "!!#["
    }
}
pub(crate) fn doc_comment_open(
    i: &str,
    ptr: Pointer,
) -> Result<(&str, Pointer, DocCommentOpen), ParseError> {
    let (i, ptr) = whitespace(i, ptr);
    if i.starts_with("!!#[") {
        Ok((
            &i["!!#[".len()..],
            ptr.add_col("!!#[".len()),
            DocCommentOpen,
        ))
    } else {
        Err(ParseError::Expected(Box::new(DocCommentOpen)))
    }
}

/// ```yaupl
/// ]#
/// ```
#[derive(Debug, Default, Copy, Clone)]
pub(crate) struct CommentClose;
impl Token for CommentClose {
    fn token(&self) -> &str {
        "]#"
    }
}
pub(crate) fn comment_close(
    i: &str,
    ptr: Pointer,
) -> Result<(&str, Pointer, CommentClose), ParseError> {
    let (i, ptr) = whitespace(i, ptr);
    if i.starts_with("]#") {
        Ok((&i["]#".len()..], ptr.add_col("]#".len()), CommentClose))
    } else {
        Err(ParseError::Expected(Box::new(CommentClose)))
    }
}

/// ```yaupl
/// export
/// ```
#[derive(Debug, Default, Copy, Clone)]
pub(crate) struct KeywordExport;
impl Token for KeywordExport {
    fn token(&self) -> &str {
        "export"
    }
}
pub(crate) fn keyword_export(
    i: &str,
    ptr: Pointer,
) -> Result<(&str, Pointer, KeywordExport), ParseError> {
    let (i, ptr) = whitespace(i, ptr);
    if i.starts_with("export") {
        Ok((
            &i["export".len()..],
            ptr.add_col("export".len()),
            KeywordExport,
        ))
    } else {
        Err(ParseError::Expected(Box::new(KeywordExport)))
    }
}

/// ```yaupl
/// return
/// ```
#[derive(Debug, Default, Copy, Clone)]
pub(crate) struct KeywordReturn;
impl Token for KeywordReturn {
    fn token(&self) -> &str {
        "return"
    }
}
pub(crate) fn keyword_return(
    i: &str,
    ptr: Pointer,
) -> Result<(&str, Pointer, KeywordReturn), ParseError> {
    let (i, ptr) = whitespace(i, ptr);
    if i.starts_with("return") {
        Ok((
            &i["return".len()..],
            ptr.add_col("return".len()),
            KeywordReturn,
        ))
    } else {
        Err(ParseError::Expected(Box::new(KeywordReturn)))
    }
}

/// ```yaupl
/// with
/// ```
#[derive(Debug, Default, Copy, Clone)]
pub(crate) struct KeywordWith;
impl Token for KeywordWith {
    fn token(&self) -> &str {
        "with"
    }
}
pub(crate) fn keyword_with(
    i: &str,
    ptr: Pointer,
) -> Result<(&str, Pointer, KeywordWith), ParseError> {
    let (i, ptr) = whitespace(i, ptr);
    if i.starts_with("with") {
        Ok((&i["with".len()..], ptr.add_col("with".len()), KeywordWith))
    } else {
        Err(ParseError::Expected(Box::new(KeywordWith)))
    }
}

/// ```yaupl
/// as
/// ```
#[derive(Debug, Default, Copy, Clone)]
pub(crate) struct KeywordAs;
impl Token for KeywordAs {
    fn token(&self) -> &str {
        "as"
    }
}
pub(crate) fn keyword_as(i: &str, ptr: Pointer) -> Result<(&str, Pointer, KeywordAs), ParseError> {
    let (i, ptr) = whitespace(i, ptr);
    if i.starts_with("as") {
        Ok((&i["as".len()..], ptr.add_col("as".len()), KeywordAs))
    } else {
        Err(ParseError::Expected(Box::new(KeywordAs)))
    }
}

/// ```yaupl
/// str
/// ```
#[derive(Debug, Default, Copy, Clone)]
pub(crate) struct KeywordStr;
impl Token for KeywordStr {
    fn token(&self) -> &str {
        "str"
    }
}
pub(crate) fn keyword_str(
    i: &str,
    ptr: Pointer,
) -> Result<(&str, Pointer, PrimitiveType), ParseError> {
    let (i, ptr) = whitespace(i, ptr);
    if i.starts_with("str") {
        Ok((
            &i["str".len()..],
            ptr.add_col("str".len()),
            PrimitiveType::Str,
        ))
    } else {
        Err(ParseError::Expected(Box::new(KeywordStr)))
    }
}

/// ```yaupl
/// bln
/// ```
#[derive(Debug, Default, Copy, Clone)]
pub(crate) struct KeywordBln;
impl Token for KeywordBln {
    fn token(&self) -> &str {
        "bln"
    }
}
pub(crate) fn keyword_bln(
    i: &str,
    ptr: Pointer,
) -> Result<(&str, Pointer, PrimitiveType), ParseError> {
    let (i, ptr) = whitespace(i, ptr);
    if i.starts_with("bln") {
        Ok((
            &i["bln".len()..],
            ptr.add_col("bln".len()),
            PrimitiveType::Bln,
        ))
    } else {
        Err(ParseError::Expected(Box::new(KeywordBln)))
    }
}

/// ```yaupl
/// num
/// ```
#[derive(Debug, Default, Copy, Clone)]
pub(crate) struct KeywordNum;
impl Token for KeywordNum {
    fn token(&self) -> &str {
        "num"
    }
}
pub(crate) fn keyword_num(
    i: &str,
    ptr: Pointer,
) -> Result<(&str, Pointer, PrimitiveType), ParseError> {
    let (i, ptr) = whitespace(i, ptr);
    if i.starts_with("num") {
        Ok((
            &i["num".len()..],
            ptr.add_col("num".len()),
            PrimitiveType::Num,
        ))
    } else {
        Err(ParseError::Expected(Box::new(KeywordNum)))
    }
}

/// ```yaupl
/// ___
/// ```
#[derive(Debug, Default, Copy, Clone)]
pub(crate) struct KeywordEmp;
impl Token for KeywordEmp {
    fn token(&self) -> &str {
        "___"
    }
}
pub(crate) fn keyword_emp(
    i: &str,
    ptr: Pointer,
) -> Result<(&str, Pointer, PrimitiveType), ParseError> {
    let (i, ptr) = whitespace(i, ptr);
    if i.starts_with("___") {
        Ok((
            &i["___".len()..],
            ptr.add_col("___".len()),
            PrimitiveType::Emp,
        ))
    } else {
        Err(ParseError::Expected(Box::new(KeywordEmp)))
    }
}

/// ```yaupl
/// :
/// ```
#[derive(Debug, Default, Copy, Clone)]
pub(crate) struct Colon;
impl Token for Colon {
    fn token(&self) -> &str {
        ":"
    }
}
pub(crate) fn colon(i: &str, ptr: Pointer) -> Result<(&str, Pointer, Colon), ParseError> {
    let (i, ptr) = whitespace(i, ptr);
    if i.starts_with(":") {
        Ok((&i[":".len()..], ptr.add_col(":".len()), Colon))
    } else {
        Err(ParseError::Expected(Box::new(Colon)))
    }
}

/// ```yaupl
/// @
/// ```
#[derive(Debug, Default, Copy, Clone)]
pub(crate) struct Group;
impl Token for Group {
    fn token(&self) -> &str {
        "@"
    }
}
pub(crate) fn group(i: &str, ptr: Pointer) -> Result<(&str, Pointer, Group), ParseError> {
    let (i, ptr) = whitespace(i, ptr);
    if i.starts_with("@") {
        Ok((&i["@".len()..], ptr.add_col("@".len()), Group))
    } else {
        Err(ParseError::Expected(Box::new(Group)))
    }
}

/// ```yaupl
/// ,
/// ```
#[derive(Debug, Default, Copy, Clone)]
pub(crate) struct Comma;
impl Token for Comma {
    fn token(&self) -> &str {
        ","
    }
}
pub(crate) fn comma(i: &str, ptr: Pointer) -> Result<(&str, Pointer, Comma), ParseError> {
    let (i, ptr) = whitespace(i, ptr);
    if i.starts_with(",") {
        Ok((&i[",".len()..], ptr.add_col(",".len()), Comma))
    } else {
        Err(ParseError::Expected(Box::new(Comma)))
    }
}

/// ```yaupl
/// ,
/// ```
#[derive(Debug, Default, Copy, Clone)]
pub(crate) struct Decimal;
impl Token for Decimal {
    fn token(&self) -> &str {
        "."
    }
}
pub(crate) fn decimal(i: &str, ptr: Pointer) -> Result<(&str, Pointer, Decimal), ParseError> {
    let (i, ptr) = whitespace(i, ptr);
    if i.starts_with(",") {
        Ok((&i[".".len()..], ptr.add_col(".".len()), Decimal))
    } else {
        Err(ParseError::Expected(Box::new(Decimal)))
    }
}

/// ```yaupl
/// ;
/// ```
#[derive(Debug, Default, Copy, Clone)]
pub(crate) struct Semicolon;
impl Token for Semicolon {
    fn token(&self) -> &str {
        ";"
    }
}
pub(crate) fn semicolon(i: &str, ptr: Pointer) -> Result<(&str, Pointer, Semicolon), ParseError> {
    let (i, ptr) = whitespace(i, ptr);
    if i.starts_with(";") {
        Ok((&i[";".len()..], ptr.add_col(";".len()), Semicolon))
    } else {
        Err(ParseError::Expected(Box::new(Semicolon)))
    }
}

/// ```yaupl
/// ;
/// ```
#[derive(Debug, Default, Copy, Clone)]
pub(crate) struct Quote;
impl Token for Quote {
    fn token(&self) -> &str {
        "\""
    }
}
pub(crate) fn quote(i: &str, ptr: Pointer) -> Result<(&str, Pointer, Quote), ParseError> {
    let (i, ptr) = whitespace(i, ptr);
    if i.starts_with(";") {
        Ok((&i["\"".len()..], ptr.add_col("\"".len()), Quote))
    } else {
        Err(ParseError::Expected(Box::new(Quote)))
    }
}

