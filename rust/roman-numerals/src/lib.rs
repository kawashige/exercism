use std::fmt::{Display, Formatter, Result};

pub struct Roman {
    num: u32,
}

impl Display for Roman {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let romans = [
            ["I", "V", "X"],
            ["X", "L", "C"],
            ["C", "D", "M"],
            ["M", "*", "*"],
        ];
        let mut v = Vec::new();
        let mut n = self.num;
        for i in 0..4 {
            match n % 10 {
                1 => v.push(romans[i][0].to_string()),
                2 => v.push(romans[i][0].repeat(2)),
                3 => v.push(romans[i][0].repeat(3)),
                4 => v.push(format!("{}{}", romans[i][0], romans[i][1])),
                5 => v.push(romans[i][1].to_string()),
                6 => v.push(format!("{}{}", romans[i][1], romans[i][0])),
                7 => v.push(format!("{}{}", romans[i][1], romans[i][0].repeat(2))),
                8 => v.push(format!("{}{}", romans[i][1], romans[i][0].repeat(3))),
                9 => v.push(format!("{}{}", romans[i][0], romans[i][2])),
                _ => {}
            }
            n /= 10;
        }
        write!(f, "{}", v.into_iter().rev().collect::<String>())
    }
}

impl From<u32> for Roman {
    fn from(num: u32) -> Self {
        Self { num }
    }
}
