use crate::constants::*;
use rand::{thread_rng, Rng};
const STRING_CHARSET: &[u8] =
    b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789!@#$%^&*()_+-=";
const IDENTIFIER_CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz_";

fn random_string() -> String {
    format!(
        "\"{}\"",
        (0..16)
            .map(|_| {
                let idx = thread_rng().gen_range(0, STRING_CHARSET.len());
                STRING_CHARSET[idx] as char
            })
            .collect::<String>()
    )
}

fn random_identifier() -> String {
    (0..16)
        .map(|_| {
            let idx = thread_rng().gen_range(0, IDENTIFIER_CHARSET.len());
            IDENTIFIER_CHARSET[idx] as char
        })
        .collect::<String>()
}

pub fn fuzz(length: u32) -> (String, Vec<String>) {
    let mut random_tokens_vec: Vec<String> = vec![];
    let mut rng = thread_rng();

    // REVIEW: Make sure that all the tokens are used
    // TODO: Maybe redo this? Very error prone if we want to add a new token
    // TODO: make the range a constant
    // generate the random tokens into the vec
    let mut all: Vec<Box<dyn Fn() -> String>> = (0..ALL_TOKEN_TYPES.len())
        .into_iter()
        .map(|s| {
            Box::new(move || {
                let i = s.clone();
                ALL_TOKEN_TYPES[i].to_string()
            }) as Box<dyn Fn() -> String>
        })
        .collect();
    all.push(Box::new(random_identifier));
    all.push(Box::new(random_string));
    all.push(Box::new(|| {
        thread_rng().gen_range(f64::MIN, f64::MAX).to_string()
    }));
    (0..length)
        .into_iter()
        .map(|_| rng.gen_range(1, all.len()))
        .map(|i| all.get(i).unwrap()())
        .into_iter()
        .fold((String::new(), vec![]), |mut acc, elem| {
            acc.0.push_str(&elem);
            acc.0.push(' ');
            acc.1.push(elem);
            acc
        })

    // let token_string = random_tokens_vec
    //     .iter_mut()
    //     .map(|tok| {
    //         tok.push(' ');
    //         tok.clone().to_string()
    //     })
    //     .collect::<String>();
    // (
    //     token_string,
    //     random_tokens_vec
    //         .iter_mut()
    //         .map(|tok| {
    //             tok.push(' ');
    //             tok.clone().to_string()
    //         })
    //         .collect::<Vec<String>>(),
    // )
}
