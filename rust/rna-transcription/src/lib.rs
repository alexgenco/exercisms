#[derive(Debug, PartialEq)]
pub struct DeoxyribonucleicAcid {
    string: String
}

#[derive(Debug, PartialEq)]
pub struct RibonucleicAcid {
    string: String
}

fn translate(string: &String) -> String {
    string.chars().
        map(|ch|
            match ch {
                'G' => 'C',
                'C' => 'G',
                'T' => 'A',
                'A' => 'U',
                _ => ch
            }).collect::<String>()
}

impl DeoxyribonucleicAcid {
    pub fn new(string: &str) -> DeoxyribonucleicAcid {
        DeoxyribonucleicAcid { string: string.to_string() }
    }

    pub fn to_rna(&self) -> RibonucleicAcid {
        RibonucleicAcid { string: translate(&self.string) }
    }
}

impl RibonucleicAcid {
    pub fn new(string: &str) -> RibonucleicAcid {
        RibonucleicAcid { string: string.to_string() }
    }
}
