use std::collections::BTreeMap;

pub struct School {
    students: BTreeMap<usize, Vec<String>>
}

impl School {
    pub fn new() -> School {
        School { students: BTreeMap::new() }
    }

    pub fn grades(&self) -> Vec<usize> {
        self.students.keys().cloned().collect()
    }

    pub fn add(&mut self, grade: usize, name: &str) {
        let entry = self.students.entry(grade).or_insert(Vec::new());

        entry.push(name.to_string());
        entry.sort();
    }

    pub fn grade(&self, grade: usize) -> Option<&Vec<String>> {
        self.students.get(&grade)
    }
}
