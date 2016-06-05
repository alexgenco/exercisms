pub fn number(input: &str) -> Option<String> {
    let standard = standardize(input);

    if standard.len() == 10 {
        Some(standard)
    } else {
        None
    }
}

pub fn area_code(input: &str) -> Option<String> {
    let number = number(input);

    match number {
        Some(n) => Some(n.chars().take(3).collect()),
        None => None
    }
}

pub fn pretty_print(input: &str) -> String {
    if let Some(number) = number(input) {
        let area_code = number.chars().take(3).collect::<String>();
        let middle = number.chars().skip(3).take(3).collect::<String>();
        let last = number.chars().skip(6).take(4).collect::<String>();

        format!("({}) {}-{}", area_code, middle, last)
    } else {
        "invalid".to_string()
    }

}

fn standardize(input: &str) -> String {
    let mut digits = input.to_string().
        chars().
        filter(|ch| ch.is_digit(10)).
        collect();

    trim_leading_one(&mut digits);
    digits
}

fn trim_leading_one(input: &mut String) {
    if input.len() != 11 { return; }

    match input.chars().nth(0) {
        Some(ch) => if ch == '1' { input.remove(0); },
        None => ()
    }
}
