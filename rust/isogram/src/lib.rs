use std::collections::HashSet;

pub fn check(candidate: &str) -> bool {
    let mut set = HashSet::new();
    for c in candidate.to_ascii_lowercase().chars() {
        match c {
            '-' | ' ' => {}
            _ if set.contains(&c) => return false,
            _ => {
                set.insert(c);
            }
        }
    }
    true
}
