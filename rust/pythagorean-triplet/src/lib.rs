use std::collections::HashSet;

pub fn find(sum: u32) -> HashSet<[u32; 3]> {
    let mut result = HashSet::new();
    for i in 1..sum {
        for j in (i + 1)..sum {
            if i + j > sum || i + 2 * j > sum {
                break;
            }
            let k = sum - i - j;
            if i * i + j * j == k * k {
                result.insert([i, j, k]);
            }
        }
    }
    result
}
