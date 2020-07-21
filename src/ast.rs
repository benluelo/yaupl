use crate::token::*;

#[derive(Debug)]
pub struct Position {
    pub row: usize,
    pub col: usize,
}

impl std::convert::From<(usize, usize)> for Position {
    fn from(val: (usize, usize)) -> Self {
        Position {
            row: val.0,
            col: val.1,
        }
    }
}

impl Position {
    pub fn new(val: (usize, usize)) -> Self {
        Position {
            row: val.0,
            col: val.1,
        }
    }
}

#[derive(Debug)]
pub struct Location {
    pub start: Position,
    pub end: Position,
}

impl Location {
    pub fn from_token(tok: &Token) -> Self {
        Location {
            start: tok.location_start.into(),
            end: tok.location_end.into(),
        }
    }
}

#[derive(Debug)]
pub struct Program {
    body: Vec<AstNode<Box<dyn TopLevel>>>,
}

pub trait TopLevel {}
use core::fmt::Debug;
impl Debug for dyn TopLevel {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "TopLevel{{{:?}}}", self)
    }
}

#[derive(Debug)]
pub struct AstNode<T> {
    location: Location,
    body: T,
}

#[derive(Debug)]
pub struct WithStatement {
    pub file_name: AstNode<Identifier>,
    pub as_name: AstNode<Identifier>,
}

#[derive(Debug)]
pub struct Identifier(String);

type TokenStream<'token> = &'token mut Vec<Token>;

pub fn match_with(tokens: TokenStream) -> AstNode<WithStatement> {
    let mut tokens_iter = tokens.iter();
    dbg!(&tokens);
    dbg!(&tokens_iter);
    if let (
        kw_with @ Token {
            token_type: TokenType::KeywordWith,
            ..
        },
        file @ Token {
            token_type: TokenType::Variable(_),
            ..
        },
        _kw_as,
        name @ Token {
            token_type: TokenType::Variable(_),
            ..
        },
    ) = (
        tokens_iter
            .next()
            .expect("expected token \"with\"; this code should be unreachable"),
        tokens_iter.next().expect("expected a file name"),
        tokens_iter.next().expect("expected keyword \"as \""),
        tokens_iter.next().expect("identifier expected"),
    ) {
        AstNode {
            location: Location {
                start: Position::new(kw_with.location_start),
                end: Position::new(name.location_end),
            },
            body: WithStatement {
                file_name: AstNode {
                    location: Location::from_token(file),
                    body: Identifier(
                        match &file.token_type {
                            TokenType::Variable(val) => val,
                            _ => unreachable!(),
                        }
                        .into(),
                    ),
                },
                as_name: AstNode {
                    location: Location::from_token(name),
                    body: Identifier(
                        match &name.token_type {
                            TokenType::Variable(val) => val,
                            _ => unreachable!(),
                        }
                        .into(),
                    ),
                },
            },
        }
    } else {
        panic!();
    }
}
