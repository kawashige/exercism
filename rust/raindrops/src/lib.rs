pub fn raindrops(n: u32) -> String {
    let mut sounds = "".to_string();
    if n % 3 == 0 {
        sounds.push_str("Pling");
    }
    if n % 5 == 0 {
        sounds.push_str("Plang");
    }
    if n % 7 == 0 {
        sounds.push_str("Plong");
    }
    if sounds.is_empty() {
        n.to_string()
    } else {
        sounds
    }
}
