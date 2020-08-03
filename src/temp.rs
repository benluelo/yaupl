use std::collections::HashMap;

fn do_stuff() {
    let hm: HashMap<&str, &str> = HashMap::new();

    match hm.iter().collect::<Vec<(&&str, &&str)>>().as_slice() {
        [(&"a", &"b"), (&"d", &"e"), (&"g", &"h")] => {},
        _ => {}
    }
}
