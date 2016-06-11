#[derive(Debug)]
pub struct CustomSet<T> {
    items: Vec<T>
}

impl<T: PartialEq + Clone> PartialEq for CustomSet<T> {
    fn eq(&self, other: &CustomSet<T>) -> bool {
        for item in self.items.iter() {
            if !other.contains(item) { return false; }
        }

        for item in other.items.iter() {
            if !self.contains(item) { return false; }
        }

        true
    }
}

impl<T: PartialEq + Clone> CustomSet<T> {
    pub fn new(v: Vec<T>) -> CustomSet<T> {
        CustomSet { items: v }
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn contains(&self, item: &T) -> bool {
        for _item in self.items.iter() {
            if _item == item { return true; }
        }
        false
    }

    pub fn is_subset(&self, other: &CustomSet<T>) -> bool {
        for item in self.items.iter() {
            if !other.contains(item) { return false; }
        }
        true
    }

    pub fn is_disjoint(&self, other: &CustomSet<T>) -> bool {
        for item in self.items.iter() {
            if other.contains(item) { return false; }
        }
        true
    }

    pub fn add(&mut self, item: T) {
        if !self.contains(&item) {
            self.items.push(item);
        }
    }

    pub fn intersection(&self, other: &CustomSet<T>) -> CustomSet<T> {
        let mut res = CustomSet::new(Vec::new());

        for item in self.items.iter().cloned() {
            if other.contains(&item) {
                res.add(item);
            }
        }

        res
    }

    pub fn difference(&self, other: &CustomSet<T>) -> CustomSet<T> {
        let mut res = CustomSet::new(Vec::new());

        for item in self.items.iter().cloned() {
            if !other.contains(&item) {
                res.add(item);
            }
        }

        res
    }

    pub fn union(&self, other: &CustomSet<T>) -> CustomSet<T> {
        let mut res = CustomSet::new(Vec::new());
        let self_iter = self.items.iter().cloned();
        let other_iter = other.items.iter().cloned();

        for item in self_iter.chain(other_iter) {
            res.add(item);
        }

        res
    }
}
