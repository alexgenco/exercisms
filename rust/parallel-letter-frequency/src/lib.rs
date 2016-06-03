use std::collections::HashMap;
use std::sync::mpsc::channel;

pub fn frequency(lines: &[&'static str], _: usize) -> HashMap<char, usize> {
    let (tx, rx) = channel();

    for line in lines.iter() {
        tx.send(count_line(line)).unwrap();
    }

    let mut result = HashMap::new();

    for _ in 0..lines.len() {
        let hm = rx.recv().unwrap();

        for (&k, &v) in hm.iter() {
            *result.entry(k).or_insert(0) += v;
        }
    }

    result
}

fn count_line(line: &'static str) -> HashMap<char, usize> {
    let mut res = HashMap::new();

    for ch in line.chars() {
        if !ch.is_alphabetic() { continue; }

        if let Some(ltr) = ch.to_lowercase().next() {
            *res.entry(ltr).or_insert(0) += 1;
        }
    }

    res
}
