/// While the problem description indicates a return status of 1 should be returned on errors,
/// it is much more common to return a `Result`, so we provide an error type for the result here.
#[derive(Debug, Eq, PartialEq)]
pub enum AffineCipherError {
    NotCoprime(i32),
}

/// Encodes the plaintext using the affine cipher with key (`a`, `b`). Note that, rather than
/// returning a return code, the more common convention in Rust is to return a `Result`.
pub fn encode(plaintext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    if not_coprime(a) {
        Err(AffineCipherError::NotCoprime(a))
    } else {
        Ok(plaintext
            .to_ascii_lowercase()
            .chars()
            .filter(|c| c.is_ascii_alphanumeric())
            .enumerate()
            .map(|(i, c)| {
                let c = if c.is_numeric() {
                    c
                } else {
                    (((a * (c as usize - 0x61) as i32 + b) % 26) as u8 + 0x61) as char
                };
                if 0 < i && i % 5 == 0 {
                    vec![' ', c]
                } else {
                    vec![c]
                }
            })
            .flatten()
            .collect::<String>())
    }
}

/// Decodes the ciphertext using the affine cipher with key (`a`, `b`). Note that, rather than
/// returning a return code, the more common convention in Rust is to return a `Result`.
pub fn decode(ciphertext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    if not_coprime(a) {
        Err(AffineCipherError::NotCoprime(a))
    } else {
        Ok(ciphertext
            .to_ascii_lowercase()
            .chars()
            .filter(|c| c != &' ')
            .map(|c| {
                if c.is_ascii_alphabetic() {
                    ((((mmi(a % 26) * ((c as usize - 0x61) as i32 - b)) % 26 + 26) % 26) as u8
                        + 0x61) as char
                } else {
                    c
                }
            })
            .collect::<String>())
    }
}

fn mmi(a: i32) -> i32 {
    (1..).find(|n| (n * a) % 26 == 1).unwrap()
}

fn not_coprime(a: i32) -> bool {
    a % 2 == 0 || a % 13 == 0
}
