extern crate proc_macro;
use std::any::TypeId;

use proc_macro::TokenStream;
#[proc_macro]
pub fn tokens(toks: TokenStream) -> TokenStream {
    eprintln!("TOKENS: {}", toks);
    let mut fin = String::new();
    for tok in toks.clone().into_iter().collect::<Vec<_>>().chunks(11) {
        fin.push_str(&*format!(
            "
/// ```yaupl
/// {e}
/// ```
pub(crate) struct {t};
impl Token for {t} {{
    fn token(&self) -> &str {{
        {s}
    }}
}}
pub(crate) fn {n}(i: &str, ptr: Pointer) -> Res<{t}> {{
    if i.starts_with({s}) {{
        Ok((&i[{s}.len()..], ptr.add_row({s}.len()), {t}))
    }} else {{
        Err(ParseError::Expected({t}))
    }}
}}

",
            s = tok[9],
            n = tok[2].to_string().to_ascii_lowercase(),
            t = tok[2],
            e = &tok[9].to_string()[1..tok[9].to_string().len() - 1]
        ))
    }
    eprintln!("{}", fin);
    fin.parse().unwrap()
}
