#[derive(Clone, Debug)]
pub enum TokenType {
    BinaryOperator(String),
    StrLiteral(String),
    NumLiteral(f64),
    BlnLiteral(bool),
    BraceSquareOpen,
    BraceSquareClose,
    BraceCurlyOpen,
    BraceCurlyClose,
    BraceGroupOpen,
    BraceGroupClose,
    TeslaOpen,
    TeslaClose,
    ArrowLeft,
    ArrowRight,
    ArrowRightThick,
    ArrowRightCurly,
    CommentOpen,
    DocCommentOpen,
    CommentClose,
    KeywordExport,
    KeywordReturn,
    KeywordWith,
    KeywordAs,
    KeywordTrue,
    KeywordFalse,
    Keyword___,
    KeywordStr,
    KeywordBln,
    KeywordNum,
    KeywordEmp,
    Colon,
    Group,
    Variable(String),
    Comma,
}

impl std::fmt::Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match &*self {
                TokenType::BinaryOperator(val) => format!("BinaryOperator({})", val),
                TokenType::StrLiteral(val) => format!("StrLiteral({})", val),
                TokenType::NumLiteral(val) => format!("NumLiteral({})", val),
                TokenType::BlnLiteral(val) => format!("BlnLiteral({})", val),
                TokenType::BraceSquareOpen => "BraceSquareOpen".to_string(),
                TokenType::BraceSquareClose => "BraceSquareClose".to_string(),
                TokenType::BraceCurlyOpen => "BraceCurlyOpen".to_string(),
                TokenType::BraceCurlyClose => "BraceCurlyClose".to_string(),
                TokenType::BraceGroupOpen => "BraceGroupOpen".to_string(),
                TokenType::BraceGroupClose => "BraceGroupClose".to_string(),
                TokenType::TeslaOpen => "TeslaOpen".to_string(),
                TokenType::TeslaClose => "TeslaClose".to_string(),
                TokenType::ArrowLeft => "ArrowLeft".to_string(),
                TokenType::ArrowRight => "ArrowRight".to_string(),
                TokenType::ArrowRightThick => "ArrowRightThick".to_string(),
                TokenType::ArrowRightCurly => "ArrowRightCurly".to_string(),
                TokenType::CommentOpen => "CommentOpen".to_string(),
                TokenType::DocCommentOpen => "DocCommentOpen".to_string(),
                TokenType::CommentClose => "CommentClose".to_string(),
                TokenType::KeywordExport => "KeywordExport".to_string(),
                TokenType::KeywordReturn => "KeywordReturn".to_string(),
                TokenType::KeywordWith => "KeywordWith".to_string(),
                TokenType::KeywordAs => "KeywordAs".to_string(),
                TokenType::KeywordTrue => "KeywordTrue".to_string(),
                TokenType::KeywordFalse => "KeywordFalse".to_string(),
                TokenType::Keyword___ => "Keyword___".to_string(),
                TokenType::KeywordStr => "KeywordStr".to_string(),
                TokenType::KeywordBln => "KeywordBln".to_string(),
                TokenType::KeywordNum => "KeywordNum".to_string(),
                TokenType::KeywordEmp => "KeywordEmp".to_string(),
                TokenType::Colon => "Colon".to_string(),
                TokenType::Group => "Group".to_string(),
                TokenType::Variable(val) => format!("Variable({})", val),
                TokenType::Comma => "Comma".to_string(),
            }
        )
    }
}

#[derive(Debug)]
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

impl std::fmt::Display for Token {
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
    pub keywords: std::collections::HashMap<&'static str, TokenType>,
}

impl Tokenizer {
    pub fn new(file_name: String) -> Self {
        Tokenizer {
            file: std::fs::read_to_string(file_name)
                .unwrap()
                .chars()
                .collect(),
            keywords: {
                let mut map: std::collections::HashMap<&'static str, TokenType> =
                    std::collections::HashMap::new();
                map.insert("export", TokenType::KeywordExport);
                map.insert("with", TokenType::KeywordWith);
                map.insert("as", TokenType::KeywordAs);
                map.insert("return", TokenType::KeywordReturn);
                map.insert("true", TokenType::KeywordTrue);
                map.insert("false", TokenType::KeywordFalse);
                map.insert("___", TokenType::Keyword___);
                map.insert("num", TokenType::KeywordNum);
                map.insert("str", TokenType::KeywordStr);
                map.insert("bln", TokenType::KeywordBln);
                map.insert("emp", TokenType::KeywordEmp);
                map.insert("+", TokenType::BinaryOperator("+".to_string()));
                map.insert("-", TokenType::BinaryOperator("-".to_string()));
                map.insert("*", TokenType::BinaryOperator("*".to_string()));
                map.insert("/", TokenType::BinaryOperator("/".to_string()));
                map.insert("==", TokenType::BinaryOperator("==".to_string()));
                map.insert("!=", TokenType::BinaryOperator("!=".to_string()));
                map.insert("<-", TokenType::ArrowLeft);
                map.insert("->", TokenType::ArrowRight);
                map.insert("~>", TokenType::ArrowRightCurly);
                map.insert("=>", TokenType::ArrowRightThick);
                map.insert("@", TokenType::Group);
                map.insert("[", TokenType::BraceSquareOpen);
                map.insert("]", TokenType::BraceSquareClose);
                map.insert("{", TokenType::BraceCurlyOpen);
                map.insert("}", TokenType::BraceCurlyClose);
                map.insert("(|", TokenType::BraceGroupOpen);
                map.insert("|)", TokenType::BraceGroupClose);
                map.insert("(|-", TokenType::TeslaOpen);
                map.insert("-|)", TokenType::TeslaClose);
                map.insert("#[", TokenType::CommentOpen);
                map.insert("!!#[", TokenType::DocCommentOpen);
                map.insert("]#", TokenType::CommentClose);
                map.insert(",", TokenType::Comma);
                map
            },
        }
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
        println!(" --> {}, {:?}", token.len(), location);
        tokens.push(Token::new(
            self.keywords
                .get(token)
                .unwrap_or(&TokenType::Variable(token.to_string()))
                .clone(),
            (location.0, location.1 - token.len()),
            location,
        ))
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![];
        let mut current_token: String = String::new();

        let mut row = 0usize;
        let mut col = 0usize;
        let mut in_string: bool = false;
        let mut in_comment: bool = false;
        let mut ptr = 0usize;
        while let Some(c) = self.eat(&mut ptr) {
            println!("{}", c);
            println!("current_token: {}", current_token);
            println!("in_comment: {}", in_comment);
            println!("in_string: {}", in_string);
            println!("row: {}, col: {}", row, col);
            match *c {
                '\n' => {
                    if in_comment {
                    } else if in_string {
                        panic!(format!("unterminated string literal at character {}.", ptr));
                    } else if current_token.len() > 0 {
                        self.add_token(&mut tokens, &current_token, (row, col));
                        current_token.clear();
                    }
                    row += 1;
                    col = 0;
                }
                ' ' => {
                    if in_comment {
                    } else if in_string {
                        current_token.push(*c);
                    } else if current_token.len() > 0 {
                        self.add_token(&mut tokens, &current_token, (row, col));
                        current_token.clear();
                    }
                }
                ',' => {
                    if in_comment {
                    } else if in_string {
                        current_token.push(*c);
                    } else {
                        if current_token.len() > 0 {
                            self.add_token(&mut tokens, &current_token, (row, col));
                        }
                        self.add_token(&mut tokens, &",".to_string(), (row, col));
                        current_token.clear();
                    }
                }

                '=' => {
                    if in_comment {
                    } else if in_string {
                        current_token.push(*c);
                    } else {
                        if self.peak(&mut ptr, 0) == Some(&'>')
                            || self.peak(&mut ptr, 0) == Some(&'=')
                        {
                            if current_token.len() > 0 {
                                self.add_token(&mut tokens, &current_token, (row, col));
                            }
                            self.add_token(
                                &mut tokens,
                                &format!("{}{}", *c, self.peak(&mut ptr, 0).unwrap()),
                                (row, col),
                            );
                            self.eat(&mut ptr);
                            col += 1;
                            current_token.clear();
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
                            self.add_token(&mut tokens, &"]#".to_string(), (row, col));
                            // System.Console.WriteLine("current token: '" + current_token + "'");
                            in_comment = false;
                        } else {
                            if !in_comment {
                                // System.Console.WriteLine("ct: " + current_token);
                                if current_token.len() > 0 {
                                    self.add_token(&mut tokens, &current_token, (row, col));
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
                        self.add_token(&mut tokens, &"!!#[".to_string(), (row, col));
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

                // (| (|-
                '(' => {
                    if in_comment {
                    } else {
                        match self.eat(&mut ptr) {
                            Some(&'|') => {
                                if let Some(&'-') = self.peak(&mut ptr, 0) {
                                    col += 1;
                                    self.add_token(&mut tokens, &"(|-".to_string(), (row, col));
                                    self.eat(&mut ptr);
                                } else {
                                    col += 1;
                                    self.add_token(&mut tokens, &"(|".to_string(), (row, col));
                                }
                            }
                            Some(next1) => {
                                panic!(format!("Unexpected token {} at character {}.", next1, ptr))
                            }
                            None => panic!("Unexpected EOF."),
                        }
                    }
                }

                // |)
                '|' => {
                    if in_comment {
                    } else {
                        match self.peak(&mut ptr, 0) {
                            Some(x @ &')') /* | Some(x @ &'-') */ => {
                                if current_token.len() > 0 {
                                    self.add_token(&mut tokens, &current_token, (row, col));
                                    current_token.clear();
                                }
                                self.eat(&mut ptr);
                                col += 1;
                                self.add_token(
                                    &mut tokens,
                                    &vec!['|', *x].into_iter().collect::<String>(),
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
                    TokenType::Variable(val) => {
                        if val.split('.').all(|t| t.chars().all(|c| c.is_numeric())) {
                            TokenType::NumLiteral(val.parse::<f64>().unwrap())
                        } else {
                            tok.token_type.clone()
                        }
                    }
                    TokenType::KeywordTrue => TokenType::BlnLiteral(true),
                    TokenType::KeywordFalse => TokenType::BlnLiteral(false),
                    _ => tok.token_type.clone(),
                },
                ..*tok
            })
            .collect()
    }
}
