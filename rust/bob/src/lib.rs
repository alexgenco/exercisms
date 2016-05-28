pub fn reply(sentence: &str) -> &str {
    if is_silent(sentence) {
        "Fine. Be that way!"
    } else if is_yelling(sentence) {
        "Whoa, chill out!"
    } else if is_question(sentence) {
        "Sure."
    } else {
        "Whatever."
    }
}

fn is_silent(sentence: &str) -> bool {
    sentence.is_empty()
}

fn is_yelling(sentence: &str) -> bool {
    for ch in sentence.chars() {
        if ch.is_lowercase() { return false; }
    }

    true
}

fn is_question(sentence: &str) -> bool {
    sentence.chars().last().unwrap() == '?'
}
