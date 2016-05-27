pub fn hamming_distance(left: &str, right: &str) -> Result<usize, &'static str> {
    if left.len() != right.len() {
        return Err("inputs of different length");
    }

    let mut dist: usize = 0;

    for (l, r) in left.chars().zip(right.chars()) {
        if l != r { dist += 1; }
    }

    Ok(dist)
}
