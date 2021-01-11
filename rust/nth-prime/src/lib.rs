pub fn nth(n: u32) -> u32 {
    let mut primes = Vec::new();
    let mut i = 2;
    while primes.len() < n as usize + 1 {
        if primes
            .iter()
            .take_while(|p| p <= &&((i as f32).sqrt() as u32))
            .all(|p| i % p != 0)
        {
            primes.push(i);
        }
        i += 1;
    }
    *primes.last().unwrap()
}
