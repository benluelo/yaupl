pub const BINARY_OPERATOR_ADD: &'static str = "+";
pub const BINARY_OPERATOR_SUB: &'static str = "-";
pub const BINARY_OPERATOR_MUL: &'static str = "*";
pub const BINARY_OPERATOR_DIV: &'static str = "/";
pub const BINARY_OPERATOR_GT: &'static str = ">";
pub const BINARY_OPERATOR_LT: &'static str = "<";
pub const BINARY_OPERATOR_GTE: &'static str = ">=";
pub const BINARY_OPERATOR_LTE: &'static str = "<=";
pub const BINARY_OPERATOR_EQ: &'static str = "==";
pub const BINARY_OPERATOR_NEQ: &'static str = "!=";
pub const INFINITY: &'static str = "oo";
pub const NEGATIVE_INFINITY: &'static str = "-oo";
pub const KEYWORD_TRUE: &'static str = "true";
pub const KEYWORD_FALSE: &'static str = "false";
pub const BRACE_SQUARE_OPEN: &'static str = "[";
pub const BRACE_SQUARE_CLOSE: &'static str = "]";
pub const BRACE_CURLY_OPEN: &'static str = "{";
pub const BRACE_CURLY_CLOSE: &'static str = "}";
pub const BRACE_GROUP_OPEN: &'static str = "(|";
pub const BRACE_GROUP_CLOSE: &'static str = "|)";
pub const TESLA_OPEN: &'static str = "|-";
pub const TESLA_CLOSE: &'static str = "-|";
pub const ARROW_LEFT: &'static str = "<-";
pub const ARROW_RIGHT: &'static str = "->";
pub const ARROW_RIGHT_THICK: &'static str = "=>";
pub const ARROW_RIGHT_CURLY: &'static str = "~>";
pub const COMMENT_OPEN: &'static str = "#[";
pub const DOC_COMMENT_OPEN: &'static str = "!!#[";
pub const COMMENT_CLOSE: &'static str = "]#";
pub const KEYWORD_EXPORT: &'static str = "export";
pub const KEYWORD_RETURN: &'static str = "return";
pub const KEYWORD_WITH: &'static str = "with";
pub const KEYWORD_AS: &'static str = "as";
pub const KEYWORD____: &'static str = "___";
pub const KEYWORD_STR: &'static str = "str";
pub const KEYWORD_BLN: &'static str = "bln";
pub const KEYWORD_NUM: &'static str = "num";
pub const KEYWORD_EMP: &'static str = "emp";
pub const COLON: &'static str = ":";
pub const GROUP: &'static str = "@";
pub const COMMA: &'static str = ",";
pub const SEMI_COLON: &'static str = ";";

//NOTE: Make sure to add the token here if a new one is added to the constants
pub const ALL_TOKEN_TYPES: [&str; 42] = [
    BINARY_OPERATOR_ADD,
    BINARY_OPERATOR_SUB,
    BINARY_OPERATOR_MUL,
    BINARY_OPERATOR_DIV,
    BINARY_OPERATOR_GT,
    BINARY_OPERATOR_LT,
    BINARY_OPERATOR_GTE,
    BINARY_OPERATOR_LTE,
    BINARY_OPERATOR_EQ,
    BINARY_OPERATOR_NEQ,
    INFINITY,
    NEGATIVE_INFINITY,
    KEYWORD_TRUE,
    KEYWORD_FALSE,
    BRACE_SQUARE_OPEN,
    BRACE_SQUARE_CLOSE,
    BRACE_CURLY_OPEN,
    BRACE_CURLY_CLOSE,
    BRACE_GROUP_OPEN,
    BRACE_GROUP_CLOSE,
    TESLA_OPEN,
    TESLA_CLOSE,
    ARROW_LEFT,
    ARROW_RIGHT,
    ARROW_RIGHT_THICK,
    ARROW_RIGHT_CURLY,
    COMMENT_OPEN,
    DOC_COMMENT_OPEN,
    COMMENT_CLOSE,
    KEYWORD_EXPORT,
    KEYWORD_RETURN,
    KEYWORD_WITH,
    KEYWORD_AS,
    KEYWORD____,
    KEYWORD_STR,
    KEYWORD_BLN,
    KEYWORD_NUM,
    KEYWORD_EMP,
    COLON,
    GROUP,
    COMMA,
    SEMI_COLON,
];
