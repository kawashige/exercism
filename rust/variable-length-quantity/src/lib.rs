#[derive(Debug, PartialEq)]
pub enum Error {
    IncompleteNumber,
    Overflow,
}

/// Convert a list of numbers to a stream of bytes encoded with variable length encoding.
pub fn to_bytes(values: &[u32]) -> Vec<u8> {
    values
        .iter()
        .map(|v| {
            let binary_string = format!("{:b}", v);
            binary_string
                .chars()
                .rev()
                .collect::<Vec<char>>()
                .chunks(7)
                .enumerate()
                .map(|(i, chunk)| {
                    let s = format!(
                        "{}{}",
                        if i == 0 { 0 } else { 1 },
                        std::iter::repeat(&'0')
                            .take(7 - chunk.len())
                            .chain(chunk.iter().rev())
                            .collect::<String>()
                    );
                    u8::from_str_radix(&s, 2).unwrap()
                })
                .rev()
                .collect::<Vec<u8>>()
        })
        .flatten()
        .collect()
}

/// Given a stream of bytes, extract all numbers which are encoded in there.
pub fn from_bytes(bytes: &[u8]) -> Result<Vec<u32>, Error> {
    let mut results = Vec::new();
    let mut s = String::new();
    for b in bytes {
        let str = format!("{:b}", b);
        if str.len() == 8 {
            s += &str[1..];
        } else {
            s.push_str(&format!("{}{}", "0".repeat(7 - str.len()), str));
        };
        if 0 == b & 1 << 7 {
            if let Ok(n) = u32::from_str_radix(&s, 2) {
                results.push(n);
                s.clear();
            } else {
                return Err(Error::Overflow);
            }
        }
    }
    if s.is_empty() {
        Ok(results)
    } else {
        Err(Error::IncompleteNumber)
    }
}
