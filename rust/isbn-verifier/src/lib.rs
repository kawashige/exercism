/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    isbn.replace("-", "")
        .chars()
        .try_fold((0, 0), |(acc, i), c| {
            let n = if i == 9 && c == 'X' {
                10
            } else {
                c.to_digit(10)?
            };
            Some(((acc + (n as usize) * (10 - i)) % 11, i + 1))
        })
        .unwrap_or((0, 0))
        == (0, 10)
}
