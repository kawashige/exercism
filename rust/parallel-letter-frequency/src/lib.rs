use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    thread,
};

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let counts: Arc<Mutex<HashMap<char, usize>>> = Arc::new(Mutex::new(HashMap::new()));
    let mut handles = vec![];

    let worker_count = std::cmp::min(input.len(), worker_count);

    for chunk in input.chunks(input.len() / worker_count) {
        let counts = Arc::clone(&counts);
        let inputs: Vec<String> = chunk.iter().map(|c| c.to_lowercase().to_string()).collect();
        let handle = thread::spawn(move || {
            for s in inputs {
                for c in s.chars() {
                    if c.is_alphabetic() {
                        let mut map = counts.lock().unwrap();
                        *map.entry(c).or_insert(0) += 1;
                    }
                }
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    Arc::try_unwrap(counts).unwrap().into_inner().unwrap()
}
