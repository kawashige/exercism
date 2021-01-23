use std::collections::HashMap;

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let mut counts = HashMap::new();
    let worker_count = std::cmp::min(input.len(), worker_count);

    for chunk in input.chunks(input.len() / worker_count) {
        crossbeam::scope(|s| {
            let handle = s.spawn(|_| {
                let mut counts = HashMap::new();
                for s in chunk {
                    for c in s.to_ascii_lowercase().chars() {
                        if c.is_alphabetic() {
                            *counts.entry(c).or_insert(0) += 1;
                        }
                    }
                }
                counts
            });
            let thread_counts = handle.join().unwrap();
            for (k, v) in thread_counts {
                *counts.entry(k).or_insert(0) += v;
            }
        })
        .unwrap();
    }

    counts
}
