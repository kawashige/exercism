/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let input = code.replace(" ", "");
    if input.len() < 2 {
        return false;
    }
    if let Some(n) = input.chars().rev().enumerate().try_fold(0, |acc, (i, c)| {
        let n = c.to_digit(10)?;
        (acc + if i % 2 == 0 {
            n
        } else {
            if 4 < n {
                n * 2 - 9
            } else {
                n * 2
            }
        })
        .into()
    }) {
        n % 10 == 0
    } else {
        false
    }
}
