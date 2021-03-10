use std::fmt::Display;

pub trait Luhn {
    fn valid_luhn(&self) -> bool;
}

/// Here is the example of how to implement custom Luhn trait
/// for the &str type. Naturally, you can implement this trait
/// by hand for the every other type presented in the test suite,
/// but your solution will fail if a new type is presented.
/// Perhaps there exists a better solution for this problem?
impl<T> Luhn for T
where
    T: Display,
{
    fn valid_luhn(&self) -> bool {
        let input = self.to_string().replace(" ", "");
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
