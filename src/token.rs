use colored::*;

#[derive(Clone, Debug, std::cmp::PartialEq)]
pub enum TokenType {
    /// ### Examples
    /// ```hmm
    /// + - * / == != < > <= >=
    /// ```
    BinaryOperator(String),
    /// ### Examples
    /// ```hmm
    /// "..."
    /// ```
    StrLiteral(String),
    /// ### Examples
    /// ```hmm
    /// 71632
    /// ```
    NumLiteral(f64),
    /// ### Examples
    /// ```hmm
    /// true
    /// ```
    BlnLiteral(bool),
    /// ### Examples
    /// ```hmm
    /// [
    /// ```
    BraceSquareOpen,
    /// ### Examples
    /// ```hmm
    /// ]
    /// ```
    BraceSquareClose,
    /// ### Examples
    /// ```hmm
    /// {
    /// ```
    BraceCurlyOpen,
    /// ### Examples
    /// ```hmm
    /// }
    /// ```
    BraceCurlyClose,
    /// ### Examples
    /// ```hmm
    /// (|
    /// ```
    BraceGroupOpen,
    /// ### Examples
    /// ```hmm
    /// |)
    /// ```
    BraceGroupClose,
    /// ### Examples
    /// ```hmm
    /// -|)
    /// ```
    TeslaOpen,
    /// ### Examples
    /// ```hmm
    /// (|-
    /// ```
    TeslaClose,
    /// ### Examples
    /// ```hmm
    /// <-
    /// ```
    ArrowLeft,
    /// ### Examples
    /// ```hmm
    /// ->
    /// ```
    ArrowRight,
    /// ### Examples
    /// ```hmm
    /// =>
    /// ```
    ArrowRightThick,
    /// ### Examples
    /// ```hmm
    /// ~>
    /// ```
    ArrowRightCurly,
    /// ### Examples
    /// ```hmm
    /// #[
    /// ```
    CommentOpen,
    /// ### Examples
    /// ```hmm
    /// !!#[
    /// ```
    DocCommentOpen,
    /// ### Examples
    /// ```hmm
    /// ]#
    /// ```
    CommentClose,
    /// ### Examples
    /// ```hmm
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
    /// ```hmm
    /// :
    /// ```
    Colon,
    /// ### Examples
    /// ```hmm
    /// @
    /// ```
    Group,
    Identifier(String),
    /// ### Examples
    /// ```hmm
    /// ,
    /// ```
    Comma,
    /// ### Examples
    /// ```hmm
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

impl std::fmt::Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match &*self {
                TokenType::BinaryOperator(val) => val.to_string(),
                TokenType::StrLiteral(val) => format!("\"{}\"", val).green().to_string(),
                TokenType::NumLiteral(val) => val.to_string().red().to_string(),
                TokenType::BlnLiteral(val) => val.to_string().red().to_string(),
                TokenType::BraceSquareOpen => "[".to_string(),
                TokenType::BraceSquareClose => "]".to_string(),
                TokenType::BraceCurlyOpen => "{".to_string(),
                TokenType::BraceCurlyClose => "}".to_string(),
                TokenType::BraceGroupOpen => "(|".to_string(),
                TokenType::BraceGroupClose => "|)".to_string(),
                TokenType::TeslaOpen => "(|-".to_string(),
                TokenType::TeslaClose => "-|)".to_string(),
                TokenType::ArrowLeft => "<-".to_string(),
                TokenType::ArrowRight => "->".to_string(),
                TokenType::ArrowRightThick => "=>".to_string(),
                TokenType::ArrowRightCurly => "~>".to_string(),

                TokenType::CommentOpen => "#[".dimmed().to_string(),
                TokenType::DocCommentOpen => "!!#[".dimmed().to_string(),
                TokenType::CommentClose => "]#".dimmed().to_string(),

                // keywords (magenta)
                TokenType::KeywordExport => "export".magenta().to_string(),
                TokenType::KeywordReturn => "return".magenta().to_string(),
                TokenType::KeywordWith => "with".magenta().to_string(),
                TokenType::KeywordAs => "as".magenta().to_string(),

                // types (blue)
                TokenType::Keyword___ => "___".blue().to_string(),
                TokenType::KeywordStr => "str".blue().to_string(),
                TokenType::KeywordBln => "bln".blue().to_string(),
                TokenType::KeywordNum => "num".blue().to_string(),
                TokenType::KeywordEmp => "emp".blue().to_string(),

                TokenType::Colon => ":".to_string(),
                TokenType::Group => "@".to_string(),
                TokenType::Identifier(val) => val.to_string(),
                TokenType::Comma => ",".to_string(),
                TokenType::SemiColon => ";".to_string(),
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
    pub fn from_string(string: &str) -> Self {
        Tokenizer {
            file: string.chars().collect(),
            keywords: {
                let mut map: std::collections::HashMap<&'static str, TokenType> =
                    std::collections::HashMap::new();
                map.insert("export", TokenType::KeywordExport);
                map.insert("with", TokenType::KeywordWith);
                map.insert("as", TokenType::KeywordAs);
                map.insert("return", TokenType::KeywordReturn);
                map.insert("true", TokenType::BlnLiteral(true));
                map.insert("false", TokenType::BlnLiteral(false));
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
                map.insert(":", TokenType::Colon);
                map
            },
        }
    }
    pub fn new(file_name: String) -> Self {
        Tokenizer::from_string(&std::fs::read_to_string(file_name).unwrap())
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
            dbg!(location.0, location.1 - (token.len())),
            dbg!(location),
        ))
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
            println!("c: {}", c);
            println!("current_token: {}", current_token);
            println!("in_comment: {}", in_comment);
            println!("in_string: {}", in_string);
            println!("row: {}, col: {}", row, col);
            match *c {
                '\r' => {}
                '\n' => {
                    if in_comment {
                    } else if in_string {
                        panic!(format!("unterminated string literal at character {}.", ptr));
                    } else if current_token.len() > 0 {
                        println!("current: \"{}\"", current_token);
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
                    TokenType::Identifier(val) => {
                        if val.split('.').all(|t| t.chars().all(|c| c.is_numeric())) {
                            TokenType::NumLiteral(val.parse::<f64>().unwrap())
                        } else {
                            tok.token_type.clone()
                        }
                    }
                    _ => tok.token_type.clone(),
                },
                ..*tok
            })
            .collect()
    }

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
                            if new_line { "\t".repeat(indent) } else { String::new() },
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
