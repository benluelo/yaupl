use rand::{thread_rng, Rng};
use crate::constants;

fn get_random_chars(string: bool) -> String {
    const STRING_CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789!@#$%^&*()_+-=";
    const IDENTIFIER_CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz_";
    
    let mut rng = rand::thread_rng();
    if string {
        let random_string: String = (0..16)
            .map(|_| {
                let idx = rng.gen_range(0, STRING_CHARSET.len());
                STRING_CHARSET[idx] as char
            })
            .collect();

        return String::from(format!("\"{}\"", random_string.as_str()));
    } else {
        let random_identifier: String = (0..16)
            .map(|_| {
                let idx = rng.gen_range(0, IDENTIFIER_CHARSET.len());
                IDENTIFIER_CHARSET[idx] as char
            })
            .collect();

        return random_identifier;
    }
}

pub fn fuzz(length: u32) -> (String, Vec<String>) {
    let mut random_tokens_string = String::from("");
    let mut random_tokens_vec: Vec<String> = Vec::new();

    // group open and close braces
    let mut brace_square = constants::BRACE_SQUARE_OPEN.to_string();
    brace_square.push_str(constants::BRACE_SQUARE_CLOSE);
    let mut brace_curly = constants::BRACE_CURLY_OPEN.to_string();
    brace_curly.push_str(constants::BRACE_CURLY_CLOSE);
    let mut brace_group = constants::BRACE_GROUP_OPEN.to_string();
    brace_group.push_str(constants::BRACE_GROUP_CLOSE);
    let mut tesla = constants::TESLA_OPEN.to_string();
    tesla.push_str(constants::TESLA_CLOSE);
    let mut comment = constants::COMMENT_OPEN.to_string();
    comment.push_str(constants::COMMENT_CLOSE);
    let mut doc_comment = constants::DOC_COMMENT_OPEN.to_string();
    doc_comment.push_str(constants::COMMENT_CLOSE);

    // generate the random tokens into the vec
    for _i in 0..length {
        let random_token_picked = thread_rng().gen_range(1, 44);
        match random_token_picked {
            1 => random_tokens_vec.push(constants::BINARY_OPERATOR_ADD.to_string()),
            2 => random_tokens_vec.push(constants::BINARY_OPERATOR_SUB.to_string()),
            3 => random_tokens_vec.push(constants::BINARY_OPERATOR_MUL.to_string()),
            4 => random_tokens_vec.push(constants::BINARY_OPERATOR_DIV.to_string()),
            5 => random_tokens_vec.push(constants::BINARY_OPERATOR_GT.to_string()),
            6 => random_tokens_vec.push(constants::BINARY_OPERATOR_LT.to_string()),
            7 => random_tokens_vec.push(constants::BINARY_OPERATOR_GTE.to_string()),
            8 => random_tokens_vec.push(constants::BINARY_OPERATOR_LTE.to_string()),
            9 => random_tokens_vec.push(constants::BINARY_OPERATOR_EQ.to_string()),
            10 => random_tokens_vec.push(constants::BINARY_OPERATOR_NEQ.to_string()),
            11 => random_tokens_vec.push(constants::INFINITY.to_string()),
            12 => random_tokens_vec.push(constants::NEGATIVE_INFINITY.to_string()),
            13 => random_tokens_vec.push(constants::KEYWORD_TRUE.to_string()),
            14 => random_tokens_vec.push(constants::KEYWORD_FALSE.to_string()),
            15 => random_tokens_vec.push(brace_square.to_string()),
            16 => random_tokens_vec.push(constants::BRACE_SQUARE_OPEN.to_string()),
            17 => random_tokens_vec.push(brace_curly.to_string()),
            18 => random_tokens_vec.push(constants::BRACE_CURLY_OPEN.to_string()),
            19 => random_tokens_vec.push(brace_group.to_string()),
            20 => random_tokens_vec.push(constants::BRACE_CURLY_OPEN.to_string()),
            21 => random_tokens_vec.push(tesla.to_string()),
            22 => random_tokens_vec.push(constants::ARROW_LEFT.to_string()),
            23 => random_tokens_vec.push(constants::ARROW_RIGHT.to_string()),
            24 => random_tokens_vec.push(constants::ARROW_RIGHT_THICK.to_string()),
            25 => random_tokens_vec.push(constants::ARROW_RIGHT_CURLY.to_string()),
            26 => random_tokens_vec.push(comment.to_string()),
            27 => random_tokens_vec.push(doc_comment.to_string()),
            28 => random_tokens_vec.push(constants::KEYWORD_EXPORT.to_string()),
            29 => random_tokens_vec.push(constants::KEYWORD_RETURN.to_string()),
            30 => random_tokens_vec.push(constants::KEYWORD_WITH.to_string()),
            31 => random_tokens_vec.push(constants::KEYWORD_AS.to_string()),
            32 => random_tokens_vec.push(constants::KEYWORD____.to_string()),
            33 => random_tokens_vec.push(constants::KEYWORD_STR.to_string()),
            34 => random_tokens_vec.push(constants::KEYWORD_BLN.to_string()),
            35 => random_tokens_vec.push(constants::KEYWORD_NUM.to_string()),
            36 => random_tokens_vec.push(constants::KEYWORD_EMP.to_string()),
            37 => random_tokens_vec.push(constants::COLON.to_string()),
            38 => random_tokens_vec.push(constants::GROUP.to_string()),
            39 => random_tokens_vec.push(constants::COMMA.to_string()),
            40 => random_tokens_vec.push(constants::SEMI_COLON.to_string()),
            41 => random_tokens_vec.push(get_random_chars(true)),
            42 => random_tokens_vec.push(get_random_chars(false)),
            43 => random_tokens_vec.push(thread_rng().gen_range(1, 1000).to_string()),
            _ => println!("no")
        }
    }

    // create random tokens string
    for token in 0..random_tokens_vec.len() {
        random_tokens_string.push_str(random_tokens_vec[token].as_str());
        random_tokens_string.push_str(" ");
    }

    (random_tokens_string, random_tokens_vec)
}
