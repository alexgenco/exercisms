extern crate rand;

use rand::Rng;

pub struct Robot {
    name: String
}

impl Robot {
    pub fn new() -> Robot {
        Robot { name: generate_name() }
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn reset_name(& mut self) {
        let mut new_name = generate_name();

        while new_name == self.name {
            new_name = generate_name();
        }

        self.name = new_name;
    }
}

pub fn generate_name() -> String {
    let mut name = String::new();
    let mut rng = rand::thread_rng();

    let c1 = rng.gen_range(b'A', b'Z') as char;
    let c2 = rng.gen_range(b'A', b'Z') as char;
    let n1 = rng.gen_range(b'1', b'9') as char;
    let n2 = rng.gen_range(b'1', b'9') as char;
    let n3 = rng.gen_range(b'1', b'9') as char;

    name.push(c1);
    name.push(c2);
    name.push(n1);
    name.push(n2);
    name.push(n3);

    name
}
