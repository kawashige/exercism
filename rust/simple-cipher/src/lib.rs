use rand::{thread_rng, Rng};

pub fn encode(key: &str, s: &str) -> Option<String> {
    if key.is_empty() {
        return None;
    }
    s.chars()
        .zip(key.chars().cycle())
        .try_fold(String::new(), |mut s, (c, k)| {
            if k.is_numeric() || k.is_uppercase() {
                None
            } else {
                s.push((0x61 + (c as u8 - 0x61 + k as u8 - 0x61) % 26) as char);
                Some(s)
            }
        })
}

pub fn decode(key: &str, s: &str) -> Option<String> {
    if key.is_empty() {
        return None;
    }
    s.chars()
        .zip(key.chars().cycle())
        .try_fold(String::new(), |mut s, (c, k)| {
            if k.is_numeric() || k.is_uppercase() {
                None
            } else {
                s.push((0x61 + (26 + c as u8 - 0x61 - (k as u8 - 0x61)) % 26) as char);
                Some(s)
            }
        })
}

pub fn encode_random(s: &str) -> (String, String) {
    let mut rng = thread_rng();
    let key = (0..101)
        .map(|_| (0x61 + rng.gen_range(0..26) as u8) as char)
        .collect::<String>();
    let encoded = encode(&key, s);
    (key, encoded.unwrap())
}
