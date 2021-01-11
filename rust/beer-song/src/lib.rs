struct Beer {
    n: u32,
}

impl Beer {
    pub fn new(n: u32) -> Self {
        Self { n }
    }

    pub fn sentences(&self) -> String {
        self.first_sentence() + &self.second_sentence()
    }

    fn first_sentence(&self) -> String {
        let b = Self::bottle_text(self.n);
        format!(
            "{}, {}.\n",
            Self::beer_on_wall_text(Self::to_first_capital_letter(&b)),
            b,
        )
    }

    fn second_sentence(&self) -> String {
        format!(
            "{}, {}.\n",
            self.take_or_buy_text(),
            Self::beer_on_wall_text(self.remaining_bottle_text())
        )
    }

    fn beer_on_wall_text(bottle_text: String) -> String {
        format!("{} on the wall", bottle_text)
    }

    fn take_or_buy_text(&self) -> String {
        match self.n {
            0 => "Go to the store and buy some more".into(),
            1 => "Take it down and pass it around".into(),
            _ => "Take one down and pass it around".into(),
        }
    }

    fn bottle_text(n: u32) -> String {
        let s = match n {
            0 => "no more bottles".to_string(),
            1 => "1 bottle".to_string(),
            _ => format!("{} bottles", n),
        };
        s + " of beer"
    }

    fn remaining_bottle_text(&self) -> String {
        let n = if self.n == 0 { 99 } else { self.n - 1 };
        Self::bottle_text(n)
    }

    fn to_first_capital_letter(src: &str) -> String {
        let mut s = src.to_string();
        let first_letter = s.remove(0).to_ascii_uppercase();
        s.insert(0, first_letter);
        s
    }
}

pub fn verse(n: u32) -> String {
    let beer = Beer::new(n);
    beer.sentences()
}

pub fn sing(start: u32, end: u32) -> String {
    (end..=start)
        .rev()
        .map(|n| verse(n))
        .collect::<Vec<String>>()
        .join("\n")
}
