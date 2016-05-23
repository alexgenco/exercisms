use std::collections::HashMap;

pub fn word_count(string: &str) -> HashMap<String, u32> {
    let mut res: HashMap<String, u32> = HashMap::new();

    for word in string.split(|ch: char| !ch.is_alphanumeric()) {
        if word.len() == 0 { continue; }
        let val = res.entry(word.to_lowercase()).or_insert(0);
        *val += 1;
    }

    res
}
