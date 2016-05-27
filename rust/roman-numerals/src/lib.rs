static MAPPING: [(usize, &'static str); 13] = [
    (1000, "M"),
    (900, "CM"),
    (500, "D"),
    (400, "CD"),
    (100, "C"),
    (90, "XC"),
    (50, "L"),
    (40, "XL"),
    (10, "X"),
    (9, "IX"),
    (5, "V"),
    (4, "IV"),
    (1, "I")
];

pub struct Roman {
    string: String
}

impl From<usize> for Roman {
    fn from(integer: usize) -> Self {
        let mut string = String::new();
        convert(integer, &mut string);

        Roman { string: string }
    }
}

impl ToString for Roman {
    fn to_string(&self) -> String {
        self.string.clone()
    }
}

fn convert(integer: usize, acc: &mut String) {
    let mut curr: usize = integer.clone();

    for &(step, repr) in MAPPING.iter() {
        while curr >= step {
            curr -= step;
            acc.push_str(repr);
        }
    }
}
