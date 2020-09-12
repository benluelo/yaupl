use std::{collections::HashMap, fmt::Display, fs::read_to_string};
use crate::constants::*;
use colored::*;

#[derive(Clone, Debug, std::cmp::PartialEq)]
pub enum TokenType {
    /// ### Examples
    /// ```yaupl
    /// + - * / == != < > <= >=
    /// ```
    BinaryOperator(BinaryOperator),
    /// ### Examples
    /// ```yaupl
    /// "..."
    /// ```
    StrLiteral(String),
    /// ### Examples
    /// ```yaupl
    /// 71632
    /// ```
    NumLiteral(i128, u128),
    Infinity,
    NegativeInfinity,
    /// ### Examples
    /// ```yaupl
    /// true
    /// ```
    BlnLiteral(bool),
    /// ### Examples
    /// ```yaupl
    /// [
    /// ```
    BraceSquareOpen,
    /// ### Examples
    /// ```yaupl
    /// ]
    /// ```
    BraceSquareClose,
    /// ### Examples
    /// ```yaupl
    /// {
    /// ```
    BraceCurlyOpen,
    /// ### Examples
    /// ```yaupl
    /// }
    /// ```
    BraceCurlyClose,
    /// ### Examples
    /// ```yaupl
    /// (|
    /// ```
    BraceGroupOpen,
    /// ### Examples
    /// ```yaupl
    /// |)
    /// ```
    BraceGroupClose,
    /// ### Examples
    /// ```yaupl
    /// -|
    /// ```
    TeslaOpen,
    /// ### Examples
    /// ```yaupl
    /// |-
    /// ```
    TeslaClose,
    /// ### Examples
    /// ```yaupl
    /// <-
    /// ```
    ArrowLeft,
    /// ### Examples
    /// ```yaupl
    /// ->
    /// ```
    ArrowRight,
    /// ### Examples
    /// ```yaupl
    /// =>
    /// ```
    ArrowRightThick,
    /// ### Examples
    /// ```yaupl
    /// ~>
    /// ```
    ArrowRightCurly,
    /// ### Examples
    /// ```yaupl
    /// #[
    /// ```
    CommentOpen,
    /// ### Examples
    /// ```yaupl
    /// !!#[
    /// ```
    DocCommentOpen,
    /// ### Examples
    /// ```yaupl
    /// ]#
    /// ```
    CommentClose,
    /// ### Examples
    /// ```yaupl
    /// export
    /// ```
    KeywordExport,
    KeywordReturn,
    KeywordWith,
    KeywordAs,
    Keyword___,
    KeywordStr,
    KeywordBln,
    KeywordNum,
    KeywordEmp,
    /// ### Examples
    /// ```yaupl
    /// :
    /// ```
    Colon,
    /// ### Examples
    /// ```yaupl
    /// @
    /// ```
    Group,
    Identifier(String),
    /// ### Examples
    /// ```yaupl
    /// ,
    /// ```
    Comma,
    /// ### Examples
    /// ```yaupl
    /// ;
    /// ```
    SemiColon,
}

// impl TokenType {
//     pub fn value<T>(self) -> T {
//         match self {
//             TokenType::BinaryOperator()
//         }
//     }
// }

#[derive(Clone, Debug, std::cmp::PartialEq)]
pub enum BinaryOperator {
    Add,
    Sub,
    Mul,
    Div,
    Gt,
    Lt,
    Gte,
    Lte,
    Eq,
    Neq,
}

impl BinaryOperator {}

impl ToString for BinaryOperator {
    fn to_string(&self) -> String {
        match self {
            BinaryOperator::Add => BINARY_OPERATOR_ADD.into(),
            BinaryOperator::Sub => BINARY_OPERATOR_SUB.into(),
            BinaryOperator::Mul => BINARY_OPERATOR_MUL.into(),
            BinaryOperator::Div => BINARY_OPERATOR_DIV.into(),
            BinaryOperator::Gt => BINARY_OPERATOR_GT.into(),
            BinaryOperator::Lt => BINARY_OPERATOR_LT.into(),
            BinaryOperator::Gte => BINARY_OPERATOR_GTE.into(),
            BinaryOperator::Lte => BINARY_OPERATOR_LTE.into(),
            BinaryOperator::Eq => BINARY_OPERATOR_EQ.into(),
            BinaryOperator::Neq => BINARY_OPERATOR_NEQ.into(),
        }
    }
}

impl Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match &*self {
                TokenType::BinaryOperator(val) => val.to_string(),
                TokenType::StrLiteral(val) => format!("\"{}\"", val).green().to_string(),
                TokenType::NumLiteral(int, frac) => format!("{}.{:0>38}", int, frac)
                    .trim_end_matches('0')
                    .trim_end_matches('.')
                    .red()
                    .to_string(),
                TokenType::Infinity => INFINITY.to_string().red().to_string(),
                TokenType::NegativeInfinity => NEGATIVE_INFINITY.to_string().red().to_string(),
                TokenType::BlnLiteral(val) => val.to_string().red().to_string(),
                TokenType::BraceSquareOpen => BRACE_SQUARE_OPEN.to_string(),
                TokenType::BraceSquareClose => BRACE_SQUARE_CLOSE.to_string(),
                TokenType::BraceCurlyOpen => BRACE_CURLY_OPEN.to_string(),
                TokenType::BraceCurlyClose => BRACE_CURLY_CLOSE.to_string(),
                TokenType::BraceGroupOpen => BRACE_GROUP_OPEN.to_string(),
                TokenType::BraceGroupClose => BRACE_GROUP_CLOSE.to_string(),
                TokenType::TeslaOpen => TESLA_OPEN.to_string(),
                TokenType::TeslaClose => TESLA_CLOSE.to_string(),
                TokenType::ArrowLeft => ARROW_LEFT.to_string(),
                TokenType::ArrowRight => ARROW_RIGHT.to_string(),
                TokenType::ArrowRightThick => ARROW_RIGHT_THICK.to_string(),
                TokenType::ArrowRightCurly => ARROW_RIGHT_CURLY.to_string(),

                TokenType::CommentOpen => COMMENT_OPEN.dimmed().to_string(),
                TokenType::DocCommentOpen => DOC_COMMENT_OPEN.dimmed().to_string(),
                TokenType::CommentClose => COMMENT_CLOSE.dimmed().to_string(),

                // keywords (magenta)
                TokenType::KeywordExport => KEYWORD_EXPORT.magenta().to_string(),
                TokenType::KeywordReturn => KEYWORD_RETURN.magenta().to_string(),
                TokenType::KeywordWith => KEYWORD_WITH.magenta().to_string(),
                TokenType::KeywordAs => KEYWORD_AS.magenta().to_string(),

                // types (blue)
                TokenType::Keyword___ => KEYWORD____.blue().to_string(),
                TokenType::KeywordStr => KEYWORD_STR.blue().to_string(),
                TokenType::KeywordBln => KEYWORD_BLN.blue().to_string(),
                TokenType::KeywordNum => KEYWORD_NUM.blue().to_string(),
                TokenType::KeywordEmp => KEYWORD_EMP.blue().to_string(),

                TokenType::Colon => COLON.to_string(),
                TokenType::Group => GROUP.to_string(),
                TokenType::Identifier(val) => val.to_string(),
                TokenType::Comma => COMMA.to_string(),
                TokenType::SemiColon => SEMI_COLON.to_string(),
            }
        )
    }
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub location_start: (usize, usize),
    pub location_end: (usize, usize),
}

impl Token {
    pub fn new(
        token_type: TokenType,
        location_start: (usize, usize),
        location_end: (usize, usize),
    ) -> Token {
        Token {
            token_type,
            location_start,
            location_end,
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} @ {:?}:{:?}",
            self.token_type, self.location_start, self.location_end
        )
    }
}

pub struct Tokenizer {
    pub file: Vec<char>,
    pub keywords: HashMap<&'static str, TokenType>,
}

// macro_rules! add {
//     ($s: ident, $c: ident, $e:expr) => {
//         $s.add_token(&mut $t, $e, (row, col));
//                         $c.clear();
//     };
// }
impl Tokenizer {
    pub fn from_string(string: &str) -> Self {
        Tokenizer {
            file: string.chars().collect(),
            keywords: {
                let mut map: HashMap<&'static str, TokenType> = HashMap::new();
                map.insert(KEYWORD_EXPORT, TokenType::KeywordExport);
                map.insert(KEYWORD_WITH, TokenType::KeywordWith);
                map.insert(KEYWORD_AS, TokenType::KeywordAs);
                map.insert(KEYWORD_RETURN, TokenType::KeywordReturn);
                map.insert(KEYWORD_TRUE, TokenType::BlnLiteral(true));
                map.insert(KEYWORD_FALSE, TokenType::BlnLiteral(false));
                map.insert(KEYWORD____, TokenType::Keyword___);
                map.insert(KEYWORD_NUM, TokenType::KeywordNum);
                map.insert(KEYWORD_STR, TokenType::KeywordStr);
                map.insert(KEYWORD_BLN, TokenType::KeywordBln);
                map.insert(KEYWORD_EMP, TokenType::KeywordEmp);
                map.insert(INFINITY, TokenType::Infinity);
                map.insert(NEGATIVE_INFINITY, TokenType::NegativeInfinity);
                map.insert(BINARY_OPERATOR_ADD, TokenType::BinaryOperator(BinaryOperator::Add));
                map.insert(BINARY_OPERATOR_SUB, TokenType::BinaryOperator(BinaryOperator::Sub));
                map.insert(BINARY_OPERATOR_MUL, TokenType::BinaryOperator(BinaryOperator::Mul));
                map.insert(BINARY_OPERATOR_DIV, TokenType::BinaryOperator(BinaryOperator::Div));
                map.insert(BINARY_OPERATOR_EQ, TokenType::BinaryOperator(BinaryOperator::Eq));
                map.insert(BINARY_OPERATOR_NEQ, TokenType::BinaryOperator(BinaryOperator::Neq));
                map.insert(BINARY_OPERATOR_GT, TokenType::BinaryOperator(BinaryOperator::Gt));
                map.insert(BINARY_OPERATOR_LT, TokenType::BinaryOperator(BinaryOperator::Lt));
                map.insert(BINARY_OPERATOR_GTE, TokenType::BinaryOperator(BinaryOperator::Gte));
                map.insert(BINARY_OPERATOR_LTE, TokenType::BinaryOperator(BinaryOperator::Lte));
                map.insert(ARROW_LEFT, TokenType::ArrowLeft);
                map.insert(ARROW_RIGHT, TokenType::ArrowRight);
                map.insert(ARROW_RIGHT_CURLY, TokenType::ArrowRightCurly);
                map.insert(ARROW_RIGHT_THICK, TokenType::ArrowRightThick);
                map.insert(GROUP, TokenType::Group);
                map.insert(BRACE_SQUARE_OPEN, TokenType::BraceSquareOpen);
                map.insert(BRACE_SQUARE_CLOSE, TokenType::BraceSquareClose);
                map.insert(BRACE_CURLY_OPEN, TokenType::BraceCurlyOpen);
                map.insert(BRACE_CURLY_CLOSE, TokenType::BraceCurlyClose);
                map.insert(BRACE_GROUP_OPEN, TokenType::BraceGroupOpen);
                map.insert(BRACE_GROUP_CLOSE, TokenType::BraceGroupClose);
                map.insert(TESLA_OPEN, TokenType::TeslaOpen);
                map.insert(TESLA_CLOSE, TokenType::TeslaClose);
                map.insert(COMMENT_OPEN, TokenType::CommentOpen);
                map.insert(DOC_COMMENT_OPEN, TokenType::DocCommentOpen);
                map.insert(COMMENT_CLOSE, TokenType::CommentClose);
                map.insert(COMMA, TokenType::Comma);
                map.insert(COLON, TokenType::Colon);
                map
            },
        }
    }
    pub fn new(file_name: String) -> Self {
        Tokenizer::from_string(&read_to_string(file_name).unwrap())
    }

    fn peak(&self, ptr: &usize, distance: usize) -> Option<&char> {
        self.file.get(ptr + distance)
    }

    #[allow(dead_code)]
    fn current(&self, ptr: &usize) -> Option<&char> {
        self.peak(ptr, 0)
    }

    fn eat(&self, ptr: &mut usize) -> Option<&char> {
        let res = self.peak(ptr, 0);
        *ptr += 1;
        res
    }

    fn add_token(&self, tokens: &mut Vec<Token>, token: &'_ str, location: (usize, usize)) {
        if token.trim().is_empty() {
            panic!("blank token");
        }
        println!(" --> {:?}, {:?}", token.len(), location);
        tokens.push(Token::new(
            self.keywords
                .get(token)
                .unwrap_or(&TokenType::Identifier(token.to_string()))
                .clone(),
            (location.0, location.1 - (token.len())),
            location,
        ))
    }

    /// adds the value of the token to the tokens, the clears the supplied token
    fn add(&self, tokens: &mut Vec<Token>, token: &'_ mut String, location: (usize, usize)) {
        self.add_token(tokens, &token, location);
        token.clear();
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![];
        let mut current_token: String = String::new();

        let mut row = 1usize;
        let mut col = 1usize;
        let mut in_string: bool = false;
        let mut in_comment: bool = false;
        let mut ptr = 0usize;
        while let Some(c) = self.eat(&mut ptr) {
            // println!("c: {}", c);
            // println!("current_token: {}", current_token);
            // println!("in_comment: {}", in_comment);
            // println!("in_string: {}", in_string);
            // println!("row: {}, col: {}", row, col);

            // TODO: deal with comments
            match *c {
                '\r' => {}
                '\n' => {
                    if in_comment {
                    } else if in_string {
                        panic!(format!("unterminated string literal at character {}.", ptr));
                    } else if current_token.len() > 0 {
                        println!("current: \"{}\"", current_token);
                        self.add(&mut tokens, &mut current_token, (row, col));
                    }
                    row += 1;
                    col = 0;
                }
                ' ' => {
                    if in_comment {
                    } else if in_string {
                        current_token.push(*c);
                    } else if current_token.len() > 0 {
                        self.add(&mut tokens, &mut current_token, (row, col));
                    }
                }
                ',' => {
                    if in_comment {
                    } else if in_string {
                        current_token.push(*c);
                    } else {
                        if current_token.len() > 0 {
                            self.add(&mut tokens, &mut current_token, (row, col));
                        }
                        self.add_token(&mut tokens, &",".to_string(), (row, col));
                    }
                }

                // check for:
                // ==, =>,  !=, <=, >=
                '=' => {
                    if in_comment {
                    } else if in_string {
                        current_token.push(*c);
                    } else {
                        // if the preceding token is a !, > or <, parse a !=, <= or >=, respectively, then continue
                        // note that it must be succeeded by a whitespace to be valid
                        if (current_token == "!" || current_token == ">" || current_token == "<")
                            && (self.peak(&mut ptr, 1).is_none()
                                || self.peak(&mut ptr, 1).unwrap().is_ascii_whitespace())
                        {
                            current_token.push(*c);
                            self.add(&mut tokens, &mut current_token, (row, col));
                            continue;
                        // else, check if the next character is = or >
                        } else if self.peak(&mut ptr, 0) == Some(&'>')
                            || self.peak(&mut ptr, 0) == Some(&'=')
                        {
                            // clear the current token first
                            if current_token.len() > 0 {
                                self.add(&mut tokens, &mut current_token, (row, col));
                            }
                            current_token.push(*self.eat(&mut ptr).unwrap());
                            self.add(&mut tokens, &mut current_token, (row, col));
                            col += 1;
                        // anything else directly succeeding a = is invalid, so panic
                        } else {
                            panic!(format!("Unexpected token {} at character {}.", *c, ptr));
                        }
                    }
                }

                // check for ]#
                ']' => {
                    if in_string {
                        current_token.push(*c);
                    } else {
                        let next = self.peak(&mut ptr, 0);
                        if in_comment && next == Some(&'#') {
                            self.eat(&mut ptr);
                            col += 1;
                            self.add_token(&mut tokens, &COMMENT_OPEN.to_string(), (row, col));
                            // System.Console.WriteLine("current token: '" + current_token + "'");
                            in_comment = false;
                            current_token.clear();
                        } else {
                            if !in_comment {
                                // System.Console.WriteLine("ct: " + current_token);
                                if current_token.len() > 0 {
                                    self.add(&mut tokens, &mut current_token, (row, col));
                                }
                                self.add_token(&mut tokens, &c.to_string(), (row, col));
                                current_token.clear();
                            }
                        }
                    }
                }

                '!' => {
                    if in_comment {
                    } else if self.peak(&mut ptr, 0) == Some(&'!')
                        && self.peak(&mut ptr, 1) == Some(&'#')
                        && self.peak(&mut ptr, 2) == Some(&'[')
                    {
                        self.eat(&mut ptr);
                        self.eat(&mut ptr);
                        self.eat(&mut ptr);
                        col += 1;
                        col += 1;
                        col += 1;
                        println!("row: {}, col: {}", row, col);
                        self.add_token(&mut tokens, &DOC_COMMENT_OPEN.to_string(), (row, col));
                        in_comment = true;
                    } else {
                        println!(
                            "{:#?}, {:#?}, {:#?}",
                            self.peak(&mut ptr, 0),
                            self.peak(&mut ptr, 1),
                            self.peak(&mut ptr, 2)
                        );
                        panic!(format!("Unexpected token '!' at character {}.", ptr));
                    }
                }

                '#' => {
                    if in_comment {
                    } else if in_string {
                        current_token.push(*c);
                    } else {
                        let next = self.peak(&mut ptr, 0);
                        if next == Some(&'[') {
                            self.eat(&mut ptr);
                            col += 1;
                            self.add_token(&mut tokens, &"#[".to_string(), (row, col));
                            in_comment = true;
                        }
                    }
                }

                '[' | '{' | '}' | '@' => {
                    if in_comment {
                    } else if in_string {
                        current_token.push(*c);
                    } else {
                        if current_token.len() > 0 {
                            self.add_token(&mut tokens, &current_token, (row, col));
                        }
                        self.add_token(&mut tokens, &c.to_string(), (row, col));
                        current_token.clear();
                    }
                }

                ':' => {
                    if in_comment {
                    } else if in_string {
                        current_token.push(*c);
                    } else {
                        if current_token.len() > 0 {
                            self.add_token(&mut tokens, &current_token, (row, col));
                        }
                        self.add_token(&mut tokens, &c.to_string(), (row, col));
                        current_token.clear();
                    }
                }

                // (|
                '(' => {
                    if in_comment {
                    } else {
                        match self.eat(&mut ptr) {
                            // FIXME: get rid of - option
                            Some(&'|') => {
                                if let Some(&'-') = self.peak(&mut ptr, 0) {
                                    col += 1;
                                    self.add_token(&mut tokens, &"|-".to_string(), (row, col));
                                    self.eat(&mut ptr);
                                } else {
                                    col += 1;
                                    self.add_token(&mut tokens, &BRACE_GROUP_OPEN.to_string(), (row, col));
                                }
                            }
                            Some(next1) => {
                                panic!(format!("Unexpected token {} at character {}.", next1, ptr))
                            }
                            None => panic!("Unexpected EOF."),
                        }
                    }
                }

                // |), |-
                '|' => {
                    if in_comment {
                    } else {
                        match self.peak(&mut ptr, 0) {
                            Some(x @ (&')' | &'-')) /* | Some(x @ &'-') */ => {
                                if current_token.len() > 0 {
                                    self.add(&mut tokens, &mut current_token, (row, col));
                                }
                                self.eat(&mut ptr);
                                col += 1;
                                self.add_token(
                                    &mut tokens,
                                    &format!("|{}", *x),
                                    (row, col),
                                );
                            }
                            Some(x) => {
                                panic!(format!("Unexpected token {} at character {}.", x, ptr));
                            }
                            None => panic!("Unexpected EOF."),
                        }
                    }
                }

                // "quoted strings"
                '"' => {
                    if in_comment {
                    } else if in_string {
                        tokens.push(Token::new(
                            TokenType::StrLiteral(current_token.clone()),
                            (row, col - current_token.len()),
                            (row, col),
                        ));
                    }

                    in_string = !in_string;
                    current_token.clear();
                }
                _ => {
                    if in_comment {
                    } else {
                        current_token.push(*c);
                    }
                }
            }
            col += 1;
        }

        if current_token.len() > 0 {
            self.add_token(&mut tokens, &current_token, (row, col));
        }
        tokens
            .iter()
            .map(|tok| Token {
                token_type: match &tok.token_type {
                    TokenType::Identifier(val) => {
                        let mut split_val = val.split('.');
                        if split_val.clone().collect::<Vec<_>>().len() > 2 {
                            panic!("Inalid numeric literal: `{}`", val)
                        }
                        match (split_val.next(), split_val.next()) {
                            (Some(int), Some(frac)) => TokenType::NumLiteral(
                                int.parse().expect("Invalid numeric literal"),
                                format!("{:0<38}", frac)
                                    .parse()
                                    .expect("Invalid numeric literal"),
                            ),
                            (Some(int), None) => TokenType::NumLiteral(
                                int.parse().expect("Invalid numeric literal"),
                                0,
                            ),
                            (None, None) => tok.token_type.clone(),
                            _ => unreachable!(),
                        }
                    }
                    _ => tok.token_type.clone(),
                },
                ..*tok
            })
            .collect()
    }

    #[deprecated = "it sucks don't use it"]
    pub fn print(tokens: &Vec<Token>) -> String {
        let mut final_string = String::new();
        let mut indent = 0;
        let mut new_line: bool = false;
        for tok in tokens.iter() {
            match tok.token_type {
                TokenType::BraceCurlyOpen | TokenType::BraceSquareOpen => {
                    final_string
                        .push_str(format!("{}{}\n", "\t".repeat(indent), tok.token_type).as_str());
                    indent += 1;
                    new_line = true;
                }
                TokenType::BraceCurlyClose | TokenType::BraceSquareClose => {
                    indent -= 1;
                    final_string
                        .push_str(format!("\n{}{}", "\t".repeat(indent), tok.token_type).as_str());
                    final_string.push('\n');
                    new_line = true;
                }
                _ => {
                    final_string.push_str(
                        format!(
                            "{}{} ",
                            if new_line {
                                "\t".repeat(indent)
                            } else {
                                String::new()
                            },
                            tok.token_type
                        )
                        .as_str(),
                    );
                    new_line = false;
                }
            }
        }
        final_string
    }
}

fn parse_num_lit() {}
