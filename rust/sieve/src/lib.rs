pub fn primes_up_to(limit: usize) -> Vec<usize> {
    let mut primes = (2..limit+1).collect::<Vec<usize>>();
    let mut index = 0;
    let mut curr: usize;

    while index < primes.len() {
        curr = primes[index];
        primes.retain(|&elem| elem == curr || elem % curr != 0);
        index += 1;
    }

    primes
}
