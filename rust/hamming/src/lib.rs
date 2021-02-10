/// Return the Hamming distance between the strings,
/// or None if the lengths are mismatched.
pub fn hamming_distance(s1: &str, s2: &str) -> Option<usize> {
    if s1.len() != s2.len() {
        None
    } else {
        s1.chars()
            .zip(s2.chars())
            .fold(0, |d, (c1, c2)| if c1 == c2 { d } else { d + 1 })
            .into()
    }
}
