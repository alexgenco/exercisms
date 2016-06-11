use std::ascii::AsciiExt;

pub fn is_pangram(sentence: &str) -> bool {
    let mut chars = sentence.
        to_lowercase().
        chars().
        filter(|ch| ch.is_alphabetic()).
        filter(|ch| ch.is_ascii()).
        collect::<Vec<char>>();

    chars.sort();
    chars.dedup();

    chars.len() == 26
}
