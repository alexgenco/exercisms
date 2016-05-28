pub fn hex_to_int(hex: &str) -> Option<u32> {
    let base: u32 = 16;
    let mut sum: u32 = 0;

    for (i, ch) in hex.chars().rev().enumerate() {
        let magn = base.pow(i as u32);

        match ch.to_digit(base) {
            Some(digit) => sum += magn * digit,
            None => return None
        }

    }

    Some(sum)
}
