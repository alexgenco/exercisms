pub fn hamming_distance<'a>(left: &'a str, right: &'a str) -> Result<usize, &'a str> {
    if left.len() != right.len() {
        return Err("inputs of different length");
    }

    let mut dist: usize = 0;

    for (l, r) in left.chars().zip(right.chars()) {
        if l != r { dist += 1; }
    }

    Ok(dist)
}
