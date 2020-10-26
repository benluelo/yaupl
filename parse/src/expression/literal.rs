use std::marker::PhantomData;

use crate::{
    combinators::one_or_more,
    combinators::optionally,
    digit,
    expression::Expression,
    parse_error::ParseError,
    pointer::Pointer,
    tokens::KeywordTrue,
    tokens::{binary_operator_sub, decimal, keyword_emp, keyword_false, keyword_true, quote},
    Digit,
};

pub(crate) fn literal(i: &str, ptr: Pointer) -> Result<(&str, Pointer, Expression), ParseError> {
    todo!()
}

fn numeric(i: &str, ptr: Pointer) -> Result<(&str, Pointer, Num), ParseError> {
    let (i, ptr, sign) = optionally(i, ptr, &binary_operator_sub);
    let (i, ptr, integer) = one_or_more(i, ptr, &digit)?;
    if let Ok((i, ptr, _decimal_point)) = decimal(i, ptr) {
        let (i, ptr, decimal) = one_or_more(i, ptr, &digit)?;
        Ok((
            i,
            ptr,
            Num {
                positive: sign.is_some(),
                integer,
                decimal,
            },
        ))
    } else {
        Ok((
            i,
            ptr,
            Num {
                positive: sign.is_some(),
                integer,
                decimal: vec![],
            },
        ))
    }
}

fn boolean(i: &str, ptr: Pointer) -> Result<(&str, Pointer, Bln), ParseError> {
    keyword_true(i, ptr)
        .map(|res| (res.0, res.1, Bln(true)))
        .or(keyword_false(i, ptr).map(|res| (res.0, res.1, Bln(false))))
}

fn empty(i: &str, ptr: Pointer) -> Result<(&str, Pointer, Emp), ParseError> {
    keyword_emp(i, ptr).map(|res| (res.0, res.1, Emp(())))
}

fn string(i: &str, ptr: Pointer) -> Result<(&str, Pointer, Str), ParseError> {
    let (i, ptr, quote_open) = quote(i, ptr)?;
    // let found_string = i.chars().take_while(|ch| *ch != '"' || *ch != '\r' || *ch != '\n').collect::<String>();
    let mut found_string = String::new();
    while let Some(ch) = i.chars().next() {
        match ch {
            '"' => break,
            a @ '\n' | a @ '\r' => return Err(ParseError::UnterminatedStringLiteral),
            a => found_string.push(a),
        }
    }
    let (i, ptr, quote_close) = quote(i, ptr)?;

    Ok((
        &i[..found_string.len()],
        ptr.add_col(found_string.len()),
        Str(found_string),
    ))
}

pub enum Literal {
    Bln(Bln),
    Str(Str),
    Num(Num),
    Emp(Emp),
}

// TODO: Implement `Str` and `Num` properly

pub struct Bln(bool);

pub struct Str(String);

pub struct Num {
    positive: bool,
    integer: Vec<Digit>,
    decimal: Vec<Digit>,
}

pub struct Emp(());
