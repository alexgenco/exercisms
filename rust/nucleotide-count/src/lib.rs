use std::collections::HashMap;

pub fn count(nuc: char, strand: &str) -> usize {
    let mut count: usize = 0;

    for ch in strand.chars() {
        if ch == nuc { count += 1; }
    }

    count
}

pub fn nucleotide_counts(strand: &str) -> HashMap<char, usize> {
    let mut res: HashMap<char, usize> = HashMap::new();

    res.insert('A', 0);
    res.insert('T', 0);
    res.insert('C', 0);
    res.insert('G', 0);

    for nuc in strand.chars() {
        *res.get_mut(&nuc).unwrap() += 1;
    }

    res
}
