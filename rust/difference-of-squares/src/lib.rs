pub fn square_of_sum(n: i64) -> i64 {
    let mut sum = 0;

    for i in 1..n+1 {
        sum += i;
    }

    sum * sum
}

pub fn sum_of_squares(n: i64) -> i64 {
    let mut sum = 0;

    for i in 1..n+1 {
        sum += i * i;
    }

    sum
}

pub fn difference(n: i64) -> i64 {
    square_of_sum(n) - sum_of_squares(n)
}
