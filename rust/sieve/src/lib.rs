use std::collections::HashSet;

pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    let mut candidates: HashSet<u64> = (2..=upper_bound).collect();
    let mut primes = Vec::new();

    for n in 2..=upper_bound {
        if !candidates.contains(&n) {
            continue;
        }
        primes.push(n);
        let mut del = n;
        while del <= upper_bound {
            candidates.remove(&del);
            del += n;
        }
    }

    primes.into_iter().collect()
}
