pub mod defs {
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
        /// Creates a new `Location` instance from a `Token`, using the
        /// `Token`'s `location_start` and `location_end` as the `Location`'s
        /// `start` and `end`, respectively.
        pub fn from_token(tok: &Token) -> Self {
            Location {
                start: tok.location_start.into(),
                end: tok.location_end.into(),
            }
        }
    }

    #[derive(Debug)]
    pub struct Program {
        pub imports: Vec<AstNode<WithStatement>>,
        pub body: Vec<AstNode<TopLevel>>,
    }

    #[derive(Debug)]
    pub enum TopLevel {
        Assignment(Assignment),
        Expression(Expression),
        FunctionDefinition(FunctionDefinition),
    }

    #[derive(Debug)]
    pub struct FunctionDefinition {
        pub return_type: AstNode<Type>,
        pub name: AstNode<Identifier>,
        pub parameters: AstNode<std::collections::HashMap<String, Type>>,
        pub body: FunctionBody,
    }

    #[derive(Debug)]
    pub struct FunctionBody {
        pub body: AstNode<Vec<AstNode<Box<TopLevel>>>>,
        pub return_statement: AstNode<Box<TopLevel>>,
    }

    #[derive(Debug)]
    pub struct AstNode<T> {
        pub location: Location,
        pub body: T,
    }

    impl<T> AstNode<T> {
        pub fn new(location: Location, body: T) -> Self {
            Self { location, body }
        }
    }

    #[derive(Debug)]
    pub struct WithStatement {
        pub file_name: AstNode<Identifier>,
        pub as_name: AstNode<Identifier>,
    }

    #[derive(Debug)]
    pub struct ExportStatement {
        pub content: AstNode<Vec<AstNode<TopLevel>>>,
    }

    #[derive(Debug)]
    pub struct Assignment {
        ident_type: AstNode<Type>,
        identifier: AstNode<Identifier>,
        value: AstNode<Expression>,
    }

    #[derive(Debug)]
    pub enum PrimitiveType {
        Str,
        Bln,
        Num,
        Emp,
    }

    #[derive(Debug)]
    pub enum Type {
        /// ### Examples
        /// ```hmm
        /// str
        /// ```
        Primitive(PrimitiveType),
        /// ### Examples
        /// ```hmm
        /// num@@
        /// ```
        Group(AstNode<Box<Type>>),
        /// ### Examples
        /// ```hmm
        /// [a: str, b: bln@, c: [num, bln]=>___]
        /// ```
        Complex(std::collections::HashMap<String, Type>),
        /// ### Examples
        /// ```hmm
        /// [num, bln]=>___
        /// ```
        Function {
            parameters: Vec<Type>,
            return_type: Box<Type>,
        },
    }

    #[derive(Debug)]
    pub enum Expression {
        /// ### Examples
        /// ```hmm
        /// true
        /// ```
        BooleanLiteral(bool),
        /// ### Examples
        /// ```hmm
        /// "..."
        /// ```
        StringLiteral(String),
        /// ### Examples
        /// ```hmm
        /// 1597
        /// ```
        NumericLiteral(f64),
        /// ### Examples
        /// ```hmm
        /// func_name [arg1] [arg2] [arg3]
        /// ```
        FunctionCall {
            identifier: AstNode<Identifier>,
            arguments: Vec<AstNode<Expression>>,
        },
    }

    #[derive(Debug)]
    pub struct Identifier(String);

    impl Identifier {
        pub fn new(ident: String) -> Self {
            Identifier(ident)
        }
    }

    pub type TokenStream<'token> = &'token mut core::slice::Iter<'token, Token>;
}

pub mod funcs {
    use super::defs::*;
    use crate::token::*;
    pub fn match_with(tokens: TokenStream) -> AstNode<WithStatement> {
        dbg!(&tokens);
        if let (
            kw_with
            @
            Token {
                token_type: TokenType::KeywordWith,
                ..
            },
            file
            @
            Token {
                token_type: TokenType::Identifier(_),
                ..
            },
            _kw_as,
            name
            @
            Token {
                token_type: TokenType::Identifier(_),
                ..
            },
        ) = (
            tokens
                .next()
                .expect("expected token \"with\"; this code should be unreachable"),
            tokens.next().expect("expected a file name"),
            tokens.next().expect("expected keyword \"as \""),
            tokens.next().expect("identifier expected"),
        ) {
            AstNode::new(
                Location {
                    start: Position::new(kw_with.location_start),
                    end: Position::new(name.location_end),
                },
                WithStatement {
                    file_name: AstNode::new(
                        Location::from_token(file),
                        Identifier::new(
                            match &file.token_type {
                                TokenType::Identifier(val) => val,
                                _ => unreachable!(),
                            }
                            .into(),
                        ),
                    ),
                    as_name: AstNode::new(
                        Location::from_token(name),
                        Identifier::new(
                            match &name.token_type {
                                TokenType::Identifier(val) => val,
                                _ => unreachable!(),
                            }
                            .into(),
                        ),
                    ),
                },
            )
        } else {
            panic!();
        }
    }

    pub fn match_export(tokens: TokenStream) -> AstNode<ExportStatement> {
        if let Some(
            kw_export
            @
            Token {
                token_type: TokenType::KeywordExport,
                ..
            },
        ) = tokens.next()
        {
            // found export keyword, look for `->`:
            if let Some(Token {
                token_type: TokenType::ArrowRight,
                ..
            }) = tokens.next()
            {
                if let Some(
                    block_start
                    @
                    Token {
                        token_type: TokenType::BraceCurlyOpen,
                        ..
                    },
                ) = tokens.next()
                {
                    let mut body: Vec<AstNode<TopLevel>> = vec![];
                    loop {
                        // build a stream containing all the tokens until a comma, then try to parse an expression out of that.
                        let mut expression_stream = {
                            let mut temp_stream: Vec<Token> = vec![];
                            let mut depth = 0;
                            while let Some(tok) = tokens.next() {
                                // on `{` increase depth
                                if let Token {
                                    token_type: TokenType::BraceCurlyOpen,
                                    ..
                                } = tok
                                {
                                    depth += 1
                                }

                                // on `}` decrease depth
                                if let Token {
                                    token_type: TokenType::BraceCurlyClose,
                                    ..
                                } = tok
                                {
                                    if depth == 0 {
                                        panic!(
                                            "unexpected token \"{{\" at {:#?}, {:#?}.",
                                            tok.location_end, tok.location_start
                                        )
                                    } else {
                                        depth -= 1
                                    }
                                }

                                // on `,` check if depth is zero, if it is then break else add the `,` to the tokens
                                if let Token {
                                    token_type: TokenType::Comma,
                                    ..
                                } = tok
                                {
                                    if depth == 0 {
                                        break;
                                    } else {
                                        temp_stream.push(tok.clone());
                                    }
                                }
                            }
                            temp_stream.iter()
                        };

                        // first, try to parse a literal expression (`1`, `"hi"`, `true`)
                        if let Some(lit_val) = try_parse_literal(&mut expression_stream) {
                            body.push(AstNode::new(
                                lit_val.location,
                                TopLevel::Expression(lit_val.body),
                            ))
                        } else if let Some(lit_val) = try_parse_literal(&mut expression_stream) {
                        }
                    }
                }
            }
        }
        todo!();
    }

    pub fn try_parse_literal<'tok>(tokens: TokenStream) -> Option<AstNode<Expression>> {
        match tokens.next() {
            // bolean token
            Some(
                bln_tok
                @
                Token {
                    token_type: TokenType::BlnLiteral(_),
                    ..
                },
            ) => Some(AstNode::new(
                Location::from_token(bln_tok),
                Expression::BooleanLiteral(match bln_tok.token_type {
                    TokenType::BlnLiteral(val) => val,
                    _ => unreachable!(),
                }),
            )),

            // string token
            Some(
                str_tok
                @
                Token {
                    token_type: TokenType::StrLiteral(_),
                    ..
                },
            ) => Some(AstNode::new(
                Location::from_token(str_tok),
                Expression::StringLiteral(match &str_tok.token_type {
                    TokenType::StrLiteral(val) => val.clone(),
                    _ => unreachable!(),
                }),
            )),

            // number token
            Some(
                num_tok
                @
                Token {
                    token_type: TokenType::NumLiteral(_),
                    ..
                },
            ) => Some(AstNode::new(
                Location::from_token(num_tok),
                Expression::NumericLiteral(match &num_tok.token_type {
                    TokenType::NumLiteral(val) => val.clone(),
                    _ => unreachable!(),
                }),
            )),

            // if its not any of those three, then it's not a literal expression and as such return `None`
            _ => None,
        }
    }

    pub fn try_parse_assignment(tokens: TokenStream) -> Option<AstNode<Assignment>> {
        if let Some(assignment_type) = try_parse_type(tokens) {};
        todo!()
    }

    pub fn try_parse_type(tokens: TokenStream) -> Option<AstNode<Type>> {
        fn try_parse_primitive(t: Token) -> Option<Type> {
            match t.token_type {
                TokenType::KeywordBln => Some(Type::Primitive(PrimitiveType::Bln)),
                TokenType::KeywordStr => Some(Type::Primitive(PrimitiveType::Str)),
                TokenType::KeywordNum => Some(Type::Primitive(PrimitiveType::Num)),
                TokenType::Keyword___ => Some(Type::Primitive(PrimitiveType::Emp)),
                _ => return None,
            }
        };

        fn try_parse_complex(tokens: TokenStream) -> Option<AstNode<Type>> {
            if let ident
            @
            Some(Token {
                token_type: TokenType::Identifier(_),
                ..
            }) = tokens.next()
            {
                if let colon
                @
                Some(Token {
                    token_type: TokenType::Colon,
                    ..
                }) = tokens.next()
                {
                    if let  =  {
                        
                    }
                }
            }
            todo!()
        }
        // match tokens.next() {
        //     Some(tok) => Some(AstNode::new(
        //         Location::from_token(tok),
        //         try_parse_primitive(tok),
        //     )),
        //     None => None,
        // }
    }
}
