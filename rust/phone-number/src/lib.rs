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
    let number = number(input);
    if number.is_none() { return "invalid".to_string(); }

    let unwrapped = number.unwrap();
    let mut digits = unwrapped.chars();
    let mut res = "(".to_string();

    res.push(digits.next().unwrap());
    res.push(digits.next().unwrap());
    res.push(digits.next().unwrap());

    res.push_str(") ");

    res.push(digits.next().unwrap());
    res.push(digits.next().unwrap());
    res.push(digits.next().unwrap());

    res.push('-');

    res.push(digits.next().unwrap());
    res.push(digits.next().unwrap());
    res.push(digits.next().unwrap());
    res.push(digits.next().unwrap());

    res
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
