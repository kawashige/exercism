pub fn rotate(input: &str, key: i8) -> String {
    let key = ((key + 26) % 26) as u8;
    input
        .chars()
        .map(|c| {
            if c.is_alphabetic() {
                let base: u8 = if c.is_uppercase() { 0x41 } else { 0x61 };
                (base + ((c as u8 - base + key) % 26)) as char
            } else {
                c
            }
        })
        .collect()
}
