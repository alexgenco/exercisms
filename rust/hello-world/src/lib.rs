pub fn hello<'a>(name: Option<&'a str>) -> String {
    match name {
        Some(n) => format!("Hello, {}!", n),
        None => "Hello, World!".to_string()
    }
}
