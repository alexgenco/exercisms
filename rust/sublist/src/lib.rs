#[derive(PartialEq, Debug)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal
}

pub fn sublist<T: PartialEq>(left: &[T], right: &[T]) -> Comparison {
    if left == right {
        Comparison::Equal
    } else if contains(left, right) {
        Comparison::Sublist
    } else if contains(right, left) {
        Comparison::Superlist
    } else {
        Comparison::Unequal
    }
}

fn contains<T: PartialEq>(inner: &[T], outer: &[T]) -> bool {
    if outer.len() < inner.len() {
        false
    } else if outer.starts_with(inner) {
        true
    } else {
        contains(&inner, &outer[1..])
    }
}
