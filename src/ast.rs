// pub(crate) mod defs {
//     use crate::token::*;
//     use std::{collections::BTreeMap, convert::From};
//     use std::{collections::HashMap, fmt::Debug};

    #[derive(Debug, Eq, Hash, PartialEq, std::cmp::Ord, std::cmp::PartialOrd, Copy, Clone)]
    pub(crate) struct Position {
        pub(crate) row: usize,
        pub(crate) col: usize,
    }

    impl From<(usize, usize)> for Position {
        fn from(val: (usize, usize)) -> Self {
            Position {
                row: val.0,
                col: val.1,
            }
        }
    }

    impl Position {
        pub(crate) fn new(val: (usize, usize)) -> Self {
            Position {
                row: val.0,
                col: val.1,
            }
        }
    }

    #[derive(Debug, Eq, Hash, PartialEq, std::cmp::Ord, std::cmp::PartialOrd, Copy, Clone)]
    pub(crate) struct Location {
        pub(crate) start: Position,
        pub(crate) end: Position,
    }

    impl From<((usize, usize), (usize, usize))> for Location {
        fn from(tuple: ((usize, usize), (usize, usize))) -> Self {
            Location {
                start: tuple.0.into(),
                end: tuple.1.into(),
            }
        }
    }

    // impl Location {
    //     /// Creates a new `Location` instance from a `Token`, using the
    //     /// `Token`'s `location_start` and `location_end` as the `Location`'s
    //     /// `start` and `end`, respectively.
    //     pub(crate) fn from_token(tok: &Token) -> Self {
    //         Location {
    //             start: tok.location_start.into(),
    //             end: tok.location_end.into(),
    //         }
    //     }
    // }

//     #[derive(Debug)]
//     pub(crate) struct Program {
//         pub(crate) imports: Vec<AstNode<WithStatement>>,
//         pub(crate) body: Vec<AstNode<Box<dyn TopLevelNode>>>,
//     }

//     // #[derive(Debug)]
//     // pub enum TopLevel {
//     //     Assignment(Assignment),
//     //     Expression(Expression),
//     //     FunctionDefinition(FunctionDefinition),
//     // }

//     pub(crate) trait TopLevel:
//         TopLevelNode + Debug + Clone + Default + Sized + 'static
//     {
//     }
//     pub(crate) trait TopLevelNode: Debug {}

//     #[derive(Debug)]
//     pub(crate) struct FunctionDefinition<P: Type, R: Type, FB: TopLevel, FR: TopLevel> {
//         pub(crate) return_type: AstNode<R>,
//         pub(crate) name: AstNode<Identifier>,
//         pub(crate) parameters: AstNode<HashMap<AstNode<Identifier>, AstNode<P>>>,
//         pub(crate) body: FunctionBody<FB, FR>,
//     }

//     #[derive(Debug)]
//     pub(crate) struct FunctionBody<B: TopLevel, R: TopLevel> {
//         pub(crate) body: AstNode<Vec<AstNode<B>>>,
//         pub(crate) return_statement: AstNode<R>,
//     }

    #[derive(Debug, Eq, Hash, PartialEq, std::cmp::Ord, std::cmp::PartialOrd)]
    pub(crate) struct AstNode<T: ?Sized> {
        pub(crate) location: Location,
        pub(crate) body: T,
    }

    impl<T> AstNode<T> {
        pub(crate) fn new(location: Location, body: T) -> Self {
            Self { location, body }
        }

        pub(crate) fn boxed(self) -> AstNode<Box<T>> {
            AstNode {
                location: self.location,
                body: Box::new(self.body),
            }
        }
    }

//     pub(crate) trait FromToken {
//         fn from_token(token: &Token) -> Self;
//     }

//     impl<T: FromToken> FromToken for AstNode<T> {
//         fn from_token(token: &Token) -> AstNode<T> {
//             AstNode {
//                 body: T::from_token(token),
//                 location: Location::from_token(&token),
//             }
//         }
//     }

//     #[derive(Debug)]
//     pub(crate) struct Arrow;
//     #[derive(Debug)]
//     pub(crate) struct ArrowLeft;
//     #[derive(Debug)]
//     pub(crate) struct CurlyArrow;
//     #[derive(Debug)]
//     pub(crate) struct With;
//     #[derive(Debug)]
//     pub(crate) struct As;
//     #[derive(Debug)]
//     pub(crate) struct Export;
//     #[derive(Debug)]
//     pub(crate) struct Colon;
//     #[derive(Debug)]
//     pub(crate) struct Comma;
//     #[derive(Debug)]
//     pub(crate) struct LeftSquareBracket;
//     #[derive(Debug)]
//     pub(crate) struct RightSquareBracket;
//     #[derive(Debug)]
//     pub(crate) struct LeftRoundBracket;
//     #[derive(Debug)]
//     pub(crate) struct RightRoundBracket;
//     #[derive(Debug)]
//     pub(crate) struct LeftCurlyBracket;
//     #[derive(Debug)]
//     pub(crate) struct RightCurlyBracket;
//     #[derive(Debug)]
//     pub(crate) struct Group;
//     #[derive(Debug)]
//     pub(crate) struct FatArrow;
//     #[derive(Debug)]
//     pub(crate) struct Quote;
//     pub(crate) mod types {
//         use super::*;
//         #[derive(Debug)]
//         pub(crate) struct Str;
//         #[derive(Debug)]
//         pub(crate) struct Bln;
//         #[derive(Debug)]
//         pub(crate) struct Num;
//         #[derive(Debug)]
//         pub(crate) struct Emp;

//         #[derive(Debug)]
//         pub(crate) struct GroupType(AstNode<Box<dyn TypeNode>>);
//         #[derive(Debug)]
//         pub(crate) struct FunctionType {
//             parameters: AstNode<Vec<AstNode<Box<dyn TypeNode>>>>,
//             return_type: Box<AstNode<dyn TypeNode>>,
//         }
//         #[derive(Debug)]
//         pub(crate) struct Complex(BTreeMap<AstNode<Identifier>, AstNode<Box<dyn TypeNode>>>);

//         impl Type for GroupType {}
//         impl TypeNode for GroupType {}

//         impl PrimitiveType for Str {}
//         impl PrimitiveType for Bln {}
//         impl PrimitiveType for Num {}
//         impl PrimitiveType for Emp {}
//         impl PrimitiveTypeNode for Str {}
//         impl PrimitiveTypeNode for Bln {}
//         impl PrimitiveTypeNode for Num {}
//         impl PrimitiveTypeNode for Emp {}

//         impl<T> Type for T where T: PrimitiveType {}
//         impl<T> TypeNode for T where T: PrimitiveTypeNode {}

//         // top level type trait
//         pub(crate) trait Type: TypeNode + Debug {}
//         pub(crate) trait TypeNode: Debug {}

//         pub(crate) trait PrimitiveType: PrimitiveTypeNode + Debug {}
//         pub(crate) trait PrimitiveTypeNode: Debug {}
//     }

//     use types::*;

//     #[derive(Debug)]
//     pub(crate) struct WithStatement {
//         pub(crate) file_name: AstNode<Identifier>,
//         pub(crate) as_name: AstNode<Identifier>,
//     }

//     #[derive(Debug)]
//     pub(crate) struct ExportStatement {
//         pub(crate) content: AstNode<Vec<AstNode<Box<dyn TopLevelNode>>>>,
//     }

//     #[derive(Debug)]
//     pub(crate) struct Assignment {
//         pub(crate) ident_type: AstNode<Box<dyn TypeNode>>,
//         pub(crate) identifier: AstNode<Identifier>,
//         pub(crate) value: AstNode<dyn ExpressionNode>,
//     }

//     // impl<T: PrimitiveType> FromToken for T {
//     //     fn from_token(tok: &Token) -> Self {
//     //         match tok.token_type {
//     //             TokenType::KeywordStr => Str,
//     //             TokenType::KeywordBln => Bln,
//     //             TokenType::KeywordNum => Num,
//     //             TokenType::KeywordEmp => Emp,
//     //             _ => panic!("Invalid conversion."),
//     //         }
//     //     }
//     // }

//     // TODO: shits fucked
//     // #[derive(Debug, Eq, PartialEq)]
//     // pub enum Type<T: PrimitiveType> {
//     //     /// ### Examples
//     //     /// ```yaupl
//     //     /// str
//     //     /// ```
//     //     Primitive(T),
//     //     /// ### Examples
//     //     /// ```yaupl
//     //     /// num@@
//     //     /// ```
//     //     Group(AstNode<Box<Type<T>>>),
//     //     /// ### Examples
//     //     /// ```yaupl
//     //     /// [a: str, b: bln@, c: [num, bln]=>___]
//     //     /// ```
//     //     Complex(BTreeMap<AstNode<Identifier>, AstNode<Type<T>>>),
//     //     /// ### Examples
//     //     /// ```yaupl
//     //     /// [num, bln]=>___
//     //     /// ```
//     //     Function {
//     //         parameters: AstNode<Vec<AstNode<Type>>>,
//     //         return_type: Box<AstNode<Type>>,
//     //     },
//     // }
//     impl FromToken for &dyn TypeNode {
//         fn from_token(tok: &Token) -> Self {
//             match tok.token_type {
//                 TokenType::KeywordStr => &Str,
//                 TokenType::KeywordBln => &Bln,
//                 TokenType::KeywordNum => &Num,
//                 TokenType::Keyword___ => &Emp,
//                 _ => panic!(
//                     "Invalid conversion: {:?} cannot be converted to a PrimitiveType.",
//                     tok.token_type
//                 ),
//             }
//         }
//     }

//     #[derive(Debug)]
//     //     /// ### Examples
//     //     /// ```yaupl
//     //     /// true
//     //     /// ```
//     pub(crate) struct BooleanLiteral(pub(crate) bool);

//     //     /// ### Examples
//     //     /// ```yaupl
//     //     /// "..."
//     //     /// ```
//     #[derive(Debug)]
//     pub(crate) struct StringLiteral(pub(crate) String);

//     //     /// ### Examples
//     //     /// ```yaupl
//     //     /// 1597
//     //     /// ```
//     #[derive(Debug)]
//     pub(crate) struct NumericLiteral(pub(crate) i128, pub(crate) u128);

//     /// ### Examples
//     /// ```yaupl
//     /// func_name [arg1] [arg2] [arg3]
//     /// ```
//     #[derive(Debug)]
//     pub(crate) struct FunctionCall {
//         identifier: AstNode<Identifier>,
//         arguments: Vec<AstNode<Box<dyn ExpressionNode>>>,
//     }

//     impl ExpressionNode for BooleanLiteral {}
//     impl ExpressionNode for NumericLiteral {}
//     impl ExpressionNode for StringLiteral {}
//     impl ExpressionNode for FunctionCall {}

//     impl Expression for BooleanLiteral {}
//     impl Expression for NumericLiteral {}
//     impl Expression for StringLiteral {}
//     impl Expression for FunctionCall {}

//     pub(crate) trait Expression: ExpressionNode + Debug {}
//     pub(crate) trait ExpressionNode: Debug {}

//     #[derive(Debug, Eq, Hash, PartialEq, std::cmp::Ord, std::cmp::PartialOrd)]
//     pub(crate) struct Identifier(pub(crate) String);

//     impl Identifier {
//         pub(crate) fn new(ident: String) -> Self {
//             Identifier(ident)
//         }
//     }

//     impl FromToken for Identifier {
//         fn from_token(token: &Token) -> Self {
//             match token.token_type {
//                 TokenType::Identifier(ref val) => Identifier(val.into()),
//                 _ => panic!("Not an identifier."),
//             }
//         }
//     }
// }

// // pub mod funcs {
// //     #[cfg(test)]
// //     mod test_funcs {
// //         use super::*;

// //         #[test]
// //         fn test_primitive_type() {
// //             let primitive = "str";
// //             let tokens = Tokenizer::from_string(primitive).tokenize();
// //             let result = try_parse_type(&mut tokens.iter());
// //             assert!(matches!(
// //                 dbg!(&result),
// //                 Some(AstNode {
// //                     location: Location {
// //                         start: Position { row: 1, col: 1 },
// //                         end: Position { row: 1, col: 4 },
// //                     },
// //                     body: Type::Primitive(PrimitiveType::Str),
// //                 })
// //             ));
// //         }

// //         #[test]
// //         fn test_function_type() {
// //             // test function types
// //             const FUNCTION: &str = "[bln, num]=>___";
// //             let tokens = Tokenizer::from_string(FUNCTION).tokenize();
// //             let result = try_parse_type(&mut tokens.iter());
// //             let expected = Some(AstNode {
// //                 location: ((1, 0), (1, 16)).into(),
// //                 body: Type::Function {
// //                     parameters: AstNode {
// //                         location: ((1, 2), (1, 10)).into(),
// //                         body: vec![
// //                             AstNode {
// //                                 location: ((1, 2), (1, 5)).into(),
// //                                 body: Type::Primitive(PrimitiveType::Bln),
// //                             },
// //                             AstNode {
// //                                 location: ((1, 7), (1, 10)).into(),
// //                                 body: Type::Primitive(PrimitiveType::Num),
// //                             },
// //                         ],
// //                     },

// //                     return_type: Box::new(AstNode {
// //                         location: ((1, 13), (1, 16)).into(),
// //                         body: Type::Primitive(PrimitiveType::Emp),
// //                     }),
// //                 },
// //             });
// //             assert!(dbg!(result) == dbg!(expected));
// //         }

// //         #[test]
// //         fn test_function_type_nested() {
// //             // test function types
// //             const FUNCTION_NESTED: &str = "[bln, num]=>[str, num]=>[str, bln]=>str";
// //             let tokens = Tokenizer::from_string(FUNCTION_NESTED).tokenize();
// //             let result = try_parse_type(&mut tokens.iter());
// //             assert!(dbg!(result).is_some());
// //         }

// //         #[test]
// //         fn test_complex_type() {
// //             // test complex types
// //             const COMPLEX: &str = "[a: num, b: bln, c: str]";
// //             let tokens = Tokenizer::from_string(COMPLEX).tokenize();
// //             let result = try_parse_type(&mut tokens.iter());
// //             assert!(result.is_some());
// //         }

// //         #[test]
// //         fn test_group_type_simple() {
// //             // test group types
// //             const GROUP: &str = "num@";
// //             let tokens = Tokenizer::from_string(GROUP).tokenize();
// //             let result = try_parse_type(&mut tokens.iter());
// //             // dbg!(&result);
// //             assert!(result.is_some());
// //         }

// //         #[test]
// //         fn test_type_nested() {
// //             // test group types
// //             const NESTED: &str = "[a: [num]=>num@, b: bln, c: [d: str, e: ___]]@";
// //             let tokens = Tokenizer::from_string(NESTED).tokenize();
// //             let result = try_parse_type(&mut tokens.iter());
// //             // dbg!(&result);
// //             assert!(result.is_some());
// //         }

// //         #[test]
// //         fn test_group_type_nested() {
// //             // test group types
// //             const GROUP: &str = "num@@@@@";
// //             let tokens = Tokenizer::from_string(GROUP).tokenize();
// //             let result = try_parse_type(&mut tokens.iter());
// //             // dbg!(&result);
// //             assert!(result.is_some());
// //         }

// //         #[test]
// //         fn test_complex_type_nested() {
// //             // test group types
// //             const COMPLEX_NESTED: &str = "[c: [d: str, e: ___]]";
// //             let tokens = Tokenizer::from_string(COMPLEX_NESTED).tokenize();
// //             let result = try_parse_type(&mut tokens.iter());
// //             // dbg!(&result);
// //             assert!(result.is_some());
// //         }
// //     }
// //     use super::defs::*;
// //     use crate::token::*;
// //     use core::slice::Iter;

// //     pub fn match_with(tokens: &mut Iter<Token>) -> AstNode<WithStatement> {
// //         dbg!(&tokens);
// //         if let (
// //             kw_with
// //             @
// //             Token {
// //                 token_type: TokenType::KeywordWith,
// //                 ..
// //             },
// //             file
// //             @
// //             Token {
// //                 token_type: TokenType::Identifier(_),
// //                 ..
// //             },
// //             _kw_as,
// //             name
// //             @
// //             Token {
// //                 token_type: TokenType::Identifier(_),
// //                 ..
// //             },
// //         ) = (
// //             tokens
// //                 .next()
// //                 .expect("expected token \"with\"; this code should be unreachable"),
// //             tokens.next().expect("expected a file name"),
// //             tokens.next().expect("expected keyword \"as \""),
// //             tokens.next().expect("identifier expected"),
// //         ) {
// //             AstNode::new(
// //                 Location {
// //                     start: Position::new(kw_with.location_start),
// //                     end: Position::new(name.location_end),
// //                 },
// //                 WithStatement {
// //                     file_name: AstNode::new(
// //                         Location::from_token(file),
// //                         Identifier::new(
// //                             match &file.token_type {
// //                                 TokenType::Identifier(val) => val,
// //                                 _ => unreachable!(),
// //                             }
// //                             .into(),
// //                         ),
// //                     ),
// //                     as_name: AstNode::new(
// //                         Location::from_token(name),
// //                         Identifier::new(
// //                             match &name.token_type {
// //                                 TokenType::Identifier(val) => val,
// //                                 _ => unreachable!(),
// //                             }
// //                             .into(),
// //                         ),
// //                     ),
// //                 },
// //             )
// //         } else {
// //             panic!();
// //         }
// //     }

// //     pub fn match_export(tokens: &mut Iter<Token>) -> AstNode<ExportStatement> {
// //         if let Some(
// //             _kw_export
// //             @
// //             Token {
// //                 token_type: TokenType::KeywordExport,
// //                 ..
// //             },
// //         ) = tokens.next()
// //         {
// //             // found export keyword, look for `->`:
// //             if let Some(Token {
// //                 token_type: TokenType::ArrowRight,
// //                 ..
// //             }) = tokens.next()
// //             {
// //                 if let Some(
// //                     _block_start
// //                     @
// //                     Token {
// //                         token_type: TokenType::BraceCurlyOpen,
// //                         ..
// //                     },
// //                 ) = tokens.next()
// //                 {
// //                     let _body: Vec<AstNode<TopLevel>> = vec![];
// //                     fn top_level(
// //                         tokens: &mut Iter<Token>,
// //                         mut body: Vec<AstNode<TopLevel>>,
// //                     ) -> Vec<AstNode<TopLevel>> {
// //                         // build a stream containing all the tokens until a comma, then try to parse an expression out of that.
// //                         let mut temp_stream: Vec<Token> = vec![];
// //                         {
// //                             let mut depth = 0;
// //                             while let Some(tok) = tokens.next() {
// //                                 // on `{` increase depth
// //                                 if let Token {
// //                                     token_type: TokenType::BraceCurlyOpen,
// //                                     ..
// //                                 } = tok
// //                                 {
// //                                     depth += 1
// //                                 }
// //                                 // on `}` decrease depth
// //                                 else if let Token {
// //                                     token_type: TokenType::BraceCurlyClose,
// //                                     ..
// //                                 } = tok
// //                                 {
// //                                     if depth == 0 {
// //                                         panic!(
// //                                             "unexpected token \"{{\" at {:#?}, {:#?}.",
// //                                             tok.location_end, tok.location_start
// //                                         )
// //                                     } else {
// //                                         depth -= 1
// //                                     }
// //                                 }
// //                                 // on `,` check if depth is zero, if it is then break else add the `,` to the tokens
// //                                 else if let Token {
// //                                     token_type: TokenType::Comma,
// //                                     ..
// //                                 } = tok
// //                                 {
// //                                     if depth == 0 {
// //                                         break;
// //                                     } else {
// //                                         temp_stream.push(tok.clone());
// //                                     }
// //                                 } else {
// //                                     temp_stream.push(tok.clone());
// //                                 }
// //                             }
// //                         }
// //                         let expression_stream = temp_stream.iter();

// //                         // first, try to parse a literal expression (`1`, `"hi"`, `true`, `___`)
// //                         if let Some(lit_val) = try_parse_literal(&mut expression_stream.clone()) {
// //                             body.push(AstNode::new(
// //                                 lit_val.location,
// //                                 TopLevel::Expression(lit_val.body),
// //                             ));
// //                         // body;
// //                         } else {
// //                             todo!()
// //                         }
// //                         todo!();
// //                     }
// //                 }
// //             }
// //         }
// //         todo!();
// //     }

// //     pub fn try_parse_expression(tokens: &mut Iter<Token>) -> Option<AstNode<Expression>> {
// //         todo!()
// //     }

// //     pub fn try_parse_literal<'tok>(tokens: &mut Iter<Token>) -> Option<AstNode<Expression>> {
// //         match tokens.next() {
// //             // boolean token
// //             Some(
// //                 bln_tok
// //                 @
// //                 Token {
// //                     token_type: TokenType::BlnLiteral(_),
// //                     ..
// //                 },
// //             ) => Some(AstNode::new(
// //                 Location::from_token(bln_tok),
// //                 Expression::BooleanLiteral(match bln_tok.token_type {
// //                     TokenType::BlnLiteral(val) => val,
// //                     _ => unreachable!(),
// //                 }),
// //             )),

// //             // string token
// //             Some(
// //                 str_tok
// //                 @
// //                 Token {
// //                     token_type: TokenType::StrLiteral(_),
// //                     ..
// //                 },
// //             ) => Some(AstNode::new(
// //                 Location::from_token(str_tok),
// //                 Expression::StringLiteral(match &str_tok.token_type {
// //                     TokenType::StrLiteral(val) => val.clone(),
// //                     _ => unreachable!(),
// //                 }),
// //             )),

// //             // number token
// //             Some(Token {
// //                 token_type: TokenType::NumLiteral(int, frac),
// //                 location_end: end,
// //                 location_start: start,
// //             }) => Some(AstNode::new(
// //                 (*start, *end).into(),
// //                 Expression::NumericLiteral(*int, *frac),
// //             )),

// //             // if its not any of those three, then it's not a literal expression and as such return `None`
// //             _ => None,
// //         }
// //     }

// //     pub fn try_parse_assignment(tokens: &mut Iter<Token>) -> Option<AstNode<Assignment>> {
// //         let assignment_type = if let Some(_assignment_type) = try_parse_type(tokens) {
// //             _assignment_type
// //         } else {
// //             return None;
// //         };

// //         let ident = if let Some(
// //             ident
// //             @
// //             Token {
// //                 token_type: TokenType::Identifier(_),
// //                 ..
// //             },
// //         ) = tokens.next()
// //         {
// //             AstNode::from_token(ident)
// //         } else {
// //             return None;
// //         };

// //         let _arrow = if let Some(
// //             _arrow
// //             @
// //             Token {
// //                 token_type: TokenType::ArrowRight,
// //                 ..
// //             },
// //         ) = tokens.next()
// //         {
// //             _arrow
// //         } else {
// //             return None;
// //         };

// //         let expr = if let Some(expression) = try_parse_expression(tokens) {
// //             expression
// //         } else {
// //             return None;
// //         };

// //         Some(AstNode {
// //             location: assignment_type.location,
// //             body: Assignment {
// //                 identifier: ident,
// //                 ident_type: assignment_type,
// //                 value: expr,
// //             },
// //         })
// //     }

// //     /// Tries to parse a type out of the stream.
// //     /// ```yaupl
// //     /// str some_str <- "hello",
// //     /// ^^^
// //     /// ```
// //     ///
// //     /// # Arguments
// //     /// - `tokens`: The iterator containing the tokens.
// //     ///
// //     /// # Returns
// //     /// An `Option<AstNode<Type>>`, containing the type if it was found.
// //     ///
// //     pub fn try_parse_type<'parse_type>(
// //         tokens: &mut Iter<'parse_type, Token>,
// //     ) -> Option<AstNode<Type>> {
// //         /// Tries to parse a primitive value out of the stream.
// //         /// ```yaupl
// //         /// str some_str <- "hello",
// //         /// ^^^
// //         /// ```
// //         ///
// //         /// # Arguments
// //         /// - `tokens`: The iterator containing the tokens.
// //         ///
// //         /// # Returns
// //         /// An `Option<AstNode<Type>>`, containing the type if it was found.
// //         ///
// //         fn try_parse_primitive<'parse_prim>(
// //             tokens: &mut Iter<'parse_prim, Token>,
// //         ) -> Option<AstNode<Type>> {
// //             match tokens.next() {
// //                 Some(tok) => match tok.token_type {
// //                     TokenType::KeywordBln
// //                     | TokenType::KeywordStr
// //                     | TokenType::KeywordNum
// //                     | TokenType::Keyword___ => Some(AstNode::from_token(tok)),
// //                     _ => return None,
// //                 },
// //                 None => None,
// //             }
// //         };

// //         /// Tries to parse a complex value out of the stream.
// //         /// ```yaupl
// //         /// [a: str, b: num] some_str <- (|-
// //         /// ^^^^^^^^^^^^^^^^
// //         ///     a: "hello",
// //         ///     b: 42
// //         /// -|),
// //         /// ```
// //         ///
// //         /// # Arguments
// //         /// - `tokens`: The iterator containing the tokens.
// //         ///
// //         /// # Returns
// //         /// An `Option<AstNode<Type>>`, containing the type if it was found.
// //         ///
// //         fn try_parse_complex<'parse_comp>(
// //             tokens: &mut Iter<'parse_comp, Token>,
// //         ) -> Option<AstNode<Type>> {
// //             let mut temp_typemap: std::collections::BTreeMap<AstNode<Identifier>, AstNode<Type>> =
// //                 std::collections::BTreeMap::new();

// //             /// This matches the `identifier: type` syntax. It takes in a mutable reference to both the token stream and a BTree.
// //             ///
// //             /// This function doesn't return anything, it merely has side effects.
// //             ///
// //             /// ```yaupl
// //             /// [a: str, b: num] some_str <- (|-
// //             ///  ^^^^^^  ^^^^^^
// //             ///     a: "hello",
// //             ///     b: 42
// //             /// -|),
// //             /// ```
// //             ///
// //             /// # Arguments
// //             /// - toks: The token stream.
// //             /// - tm: The BTree to put the found types in.
// //             ///
// //             /// # Returns
// //             /// `Option<()>`<br/>
// //             /// `Some(())` means the function was successful and a type was found,
// //             /// `None` means it was not and no type was found.
// //             fn complex(
// //                 toks: &mut Iter<Token>,
// //                 tm: &mut std::collections::BTreeMap<AstNode<Identifier>, AstNode<Type>>,
// //             ) -> Option<()> {
// //                 // get an identifier
// //                 let ident = if let Some(
// //                     ident_tok
// //                     @
// //                     Token {
// //                         token_type: TokenType::Identifier(_),
// //                         ..
// //                     },
// //                 ) = toks.next()
// //                 {
// //                     ident_tok.clone()
// //                 } else {
// //                     return None;
// //                 };

// //                 // get a colon
// //                 let _colon = if let Some(
// //                     colon_tok
// //                     @
// //                     Token {
// //                         token_type: TokenType::Colon,
// //                         ..
// //                     },
// //                 ) = toks.next()
// //                 {
// //                     colon_tok.clone()
// //                 } else {
// //                     return None;
// //                 };

// //                 // get the type
// //                 let param_type = if let Some(_type) = try_parse_type(toks) {
// //                     _type
// //                 } else {
// //                     return None;
// //                 };

// //                 // got full `ident: type` format, check for a comma
// //                 // if a comma is found, call complex again
// //                 // else, return the AstNode<Type>

// //                 // first, add current found type to a BTree (not a hashmap, so it stays ordered)
// //                 tm.insert(AstNode::from_token(&ident), param_type);

// //                 return match toks.next() {
// //                     // if a comma is found, call complex again
// //                     Some(
// //                         _comma
// //                         @
// //                         Token {
// //                             token_type: TokenType::Comma,
// //                             ..
// //                         },
// //                     ) => complex(toks, tm),

// //                     // if a clsing square bracket is found, the type is finished and return Some(())
// //                     Some(
// //                         _close_bracket
// //                         @
// //                         Token {
// //                             token_type: TokenType::BraceSquareClose,
// //                             ..
// //                         },
// //                     ) => Some(()),

// //                     // anything else is an invalid token, return None
// //                     _ => None,
// //                 };
// //             }

// //             // TODO: figure out where this is supposed to be used
// //             let first_token: Position = if let Some(
// //                 first
// //                 @
// //                 Token {
// //                     token_type: TokenType::BraceSquareOpen,
// //                     ..
// //                 },
// //             ) = tokens.next()
// //             {
// //                 first.location_start.into()
// //             } else {
// //                 return None;
// //             };
// //             match complex(tokens, &mut temp_typemap) {
// //                 Some(_) => {
// //                     let first = if let Some(first) = temp_typemap.iter().next() {
// //                         first.0.location.start
// //                     } else {
// //                         return None;
// //                     };
// //                     let last = if let Some(last) = temp_typemap.iter().next_back() {
// //                         last.1.location.end
// //                     } else {
// //                         return None;
// //                     };
// //                     let hm = temp_typemap.into_iter().collect();
// //                     Some(AstNode::new(
// //                         Location {
// //                             start: first,
// //                             end: last,
// //                         },
// //                         Type::Complex(hm),
// //                     ))
// //                 }
// //                 None => None,
// //             }
// //         }

// //         /// Tries to parse a function value out of the stream.
// //         /// ```yaupl
// //         /// bln some_function [a: str, b: num] -> {
// //         ///     #[ some stuff happens here... ]#
// //         ///     return true,
// //         /// },
// //         ///
// //         /// [str, num]=>bln  some_str <- some_function,
// //         /// ^^^^^^^^^^^^^^^
// //         /// ```
// //         ///
// //         /// # Arguments
// //         /// - `tokens`: The iterator containing the tokens.
// //         ///
// //         /// # Returns
// //         /// An `Option<AstNode<Type>>`, containing the type if it was found.
// //         ///
// //         fn try_parse_function<'parse_func>(
// //             tokens: &mut Iter<'parse_func, Token>,
// //         ) -> Option<AstNode<Type>> {
// //             dbg!(&tokens);
// //             let params = vec![];
// //             /// This matches the `[type(, type)*]` syntax.
// //             ///
// //             /// This function doesn't return anything, it merely has side effects.
// //             ///
// //             /// ```yaupl
// //             /// [str, num]=>bln some_str [a: str, b: num] -> {
// //             /// ^^^^^^^^^^
// //             ///     #[ some stuff happens here... ]#
// //             ///     return true,
// //             /// }
// //             /// ```
// //             ///
// //             /// # Arguments
// //             /// - toks: The token stream.
// //             /// - params: The Vec to put the found types in.
// //             ///
// //             /// # Returns
// //             /// `Option<Vec<AstNode<Type>>>`<br/>
// //             /// `Some` means the function was successful and a type was found,
// //             /// `None` means it was not and no type was found.
// //             fn function<'func>(
// //                 tokens: &mut Iter<'func, Token>,
// //                 mut params: Vec<AstNode<Type>>,
// //             ) -> Option<Vec<AstNode<Type>>> {
// //                 dbg!(&params);
// //                 // get a type
// //                 if let Some(param_type) = try_parse_type(tokens) {
// //                     println!("found a type: {:#?}", param_type);
// //                     params.push(param_type);
// //                 } else {
// //                     println!("found none");
// //                     return None;
// //                 };

// //                 // check for either a `,` or a `]`; anything else is invalid
// //                 match tokens.next() {
// //                     Some(
// //                         _comma
// //                         @
// //                         Token {
// //                             token_type: TokenType::Comma,
// //                             ..
// //                         },
// //                     ) => {
// //                         println!("found a comma");
// //                         function(tokens, params)
// //                     }
// //                     Some(
// //                         _brace_close
// //                         @
// //                         Token {
// //                             token_type: TokenType::BraceSquareClose,
// //                             ..
// //                         },
// //                     ) => Some(params),
// //                     Some(invalid) => panic!(
// //                         "Unexpected token {} at {}, {}",
// //                         invalid.token_type, invalid.location_start.0, invalid.location_start.1
// //                     ),
// //                     None => return None,
// //                 }
// //             }

// //             let first_token: Position = if let Some(
// //                 first
// //                 @
// //                 Token {
// //                     token_type: TokenType::BraceSquareOpen,
// //                     ..
// //                 },
// //             ) = tokens.next()
// //             {
// //                 dbg!(&first);
// //                 dbg!(Location::from_token(&first).start)
// //             } else {
// //                 return None;
// //             };
// //             let res = function(tokens, params);
// //             match res {
// //                 Some(params) => {
// //                     if let Some(_arrow) = tokens.next() {
// //                         if let Some(return_type) = try_parse_type(tokens) {
// //                             let start = params.first().unwrap().location.start;
// //                             let params_end = params.last().unwrap().location.end;
// //                             let end = return_type.location.end;

// //                             Some(AstNode::new(
// //                                 Location {
// //                                     start: first_token,
// //                                     end,
// //                                 },
// //                                 Type::Function {
// //                                     parameters: AstNode {
// //                                         location: Location {
// //                                             start,
// //                                             end: params_end,
// //                                         },
// //                                         body: params,
// //                                     },
// //                                     return_type: Box::new(return_type),
// //                                 },
// //                             ))
// //                         } else {
// //                             None
// //                         }
// //                     } else {
// //                         panic!("Expected token ArrowThick")
// //                     }
// //                 }
// //                 None => None,
// //             }
// //         }

// //         /// TODO: refactor to not use an `Option` since there is no use case for `None`
// //         fn try_parse_group<'parse_group>(
// //             tokens: &mut Iter<'parse_group, Token>,
// //             found_type: AstNode<Type>,
// //         ) -> Option<AstNode<Type>> {
// //             println!("looking for a group");
// //             dbg!(&tokens);
// //             let mut tokens_peekable = tokens.clone().peekable();

// //             let peeked = if let Some(
// //                 _peeked
// //                 @
// //                 &&Token {
// //                     token_type: TokenType::Group,
// //                     ..
// //                 },
// //             ) = tokens_peekable.peek()
// //             {
// //                 println!("found a group sigil");
// //                 _peeked.location_end
// //             } else {
// //                 return Some(found_type);
// //             };
// //             tokens.next();
// //             try_parse_group(
// //                 tokens,
// //                 AstNode {
// //                     location: Location {
// //                         start: found_type.location.start,
// //                         end: peeked.into(),
// //                     },
// //                     body: Type::Group(found_type.boxed()),
// //                 },
// //             )
// //         }

// //         if let Some(found_type) = if let Some(_) = try_parse_primitive(&mut tokens.clone()) {
// //             try_parse_primitive(tokens)
// //         } else if let Some(_) = try_parse_complex(&mut tokens.clone()) {
// //             try_parse_complex(tokens)
// //         } else if let Some(_) = try_parse_function(&mut tokens.clone()) {
// //             try_parse_function(tokens)
// //         } else {
// //             None
// //         } {
// //             try_parse_group(tokens, found_type)
// //         } else {
// //             None
// //         }
// //     }
// // }
