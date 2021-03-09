use std::fmt::Display;

pub struct Luhn {
    code: String,
}

impl Luhn {
    pub fn is_valid(&self) -> bool {
        let input = self.code.replace(" ", "");
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
}

/// Here is the example of how the From trait could be implemented
/// for the &str type. Naturally, you can implement this trait
/// by hand for the every other type presented in the test suite,
/// but your solution will fail if a new type is presented.
/// Perhaps there exists a better solution for this problem?
impl<T> From<T> for Luhn
where
    T: Display,
{
    fn from(input: T) -> Self {
        Self {
            code: input.to_string(),
        }
    }
}
