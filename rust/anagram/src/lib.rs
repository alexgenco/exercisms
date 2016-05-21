pub fn anagrams_for<'a>(word: &str, candidates: &[&str]) -> Vec<String> {
    candidates.into_iter().
        filter(|cand| is_anagram(cand, word)).
        map(|ana| ana.to_string()).
        collect()
}

fn is_anagram(candidate: &str, target: &str) -> bool {
    let lower_cand = candidate.to_lowercase();
    let lower_targ = target.to_lowercase();

    if lower_cand == lower_targ {
        return false;
    }

    let mut cand_chars: Vec<char> = lower_cand.chars().collect();
    let mut targ_chars: Vec<char> = lower_targ.chars().collect();

    cand_chars.sort();
    targ_chars.sort();

    cand_chars == targ_chars
}
