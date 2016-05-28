use std::collections::BTreeMap;

pub fn transform(legacy: &BTreeMap<i32, Vec<String>>) -> BTreeMap<String, i32> {
    let mut res: BTreeMap<String, i32> = BTreeMap::new();

    for (score, letters) in legacy.iter() {
        for letter in letters {
            res.insert(letter.to_string().to_lowercase(), *score);
        }
    }

    res
}
