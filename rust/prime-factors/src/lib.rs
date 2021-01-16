use std::collections::BTreeSet;

pub fn factors(n: u64) -> Vec<u64> {
    let max = (n as f64).sqrt() as u64;
    let mut targets: BTreeSet<u64> = (2..=max).collect();
    let mut results = Vec::new();

    let mut remains = n;

    while 1 < remains && !targets.is_empty() {
        let next = *targets.iter().next().unwrap();
        while remains % next == 0 {
            results.push(next);
            remains /= next;
        }
        for i in (1..).take_while(|i| i * next <= max) {
            if targets.contains(&(i * next)) {
                targets.remove(&(i * next));
            }
        }
    }
    if 1 < remains {
        results.push(remains);
    }

    results
}
