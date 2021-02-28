#[derive(Debug, PartialEq)]
pub enum Error {
    SpanTooLong,
    InvalidDigit(char),
}

pub fn lsp(string_digits: &str, span: usize) -> Result<u64, Error> {
    if span == 0 {
        return Ok(1);
    } else if string_digits.len() < span {
        return Err(Error::SpanTooLong);
    }

    match string_digits
        .chars()
        .map(|c| c.to_digit(10).ok_or(c))
        .collect::<Result<Vec<u32>, char>>()
    {
        Ok(v) => Ok(v
            .windows(span)
            .map(|v| v.iter().product::<u32>() as u64)
            .max()
            .unwrap()),
        Err(c) => Err(Error::InvalidDigit(c)),
    }
}
