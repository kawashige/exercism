/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    if let Some(input) = code
        .replace(" ", "")
        .chars()
        .map(|c| c.to_digit(10))
        .collect::<Option<Vec<u32>>>()
    {
        1 < input.len()
            && input
                .into_iter()
                .rev()
                .zip(1..)
                .map(|(n, i)| {
                    if i % 2 == 0 {
                        if 4 < n {
                            n * 2 - 9
                        } else {
                            n * 2
                        }
                    } else {
                        n
                    }
                })
                .sum::<u32>()
                % 10
                == 0
    } else {
        false
    }
}
