use std::collections::BTreeSet;

#[derive(Debug)]
pub struct HighScores<'a> {
    scores: &'a [u32],
}

impl<'a> HighScores<'a> {
    pub fn new(scores: &'a [u32]) -> Self {
        Self { scores }
    }

    pub fn scores(&self) -> &[u32] {
        self.scores
    }

    pub fn latest(&self) -> Option<u32> {
        if self.scores.is_empty() {
            None
        } else {
            self.scores[self.scores.len() - 1].into()
        }
    }

    pub fn personal_best(&self) -> Option<u32> {
        if self.scores.is_empty() {
            None
        } else {
            (*self.scores.iter().max().unwrap()).into()
        }
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let mut top_three = Vec::new();
        for s in self.scores {
            top_three.push(*s);
            top_three.sort_by_key(|t| -(*t as i32));
            while 3 < top_three.len() {
                top_three.pop();
            }
        }

        top_three
    }
}
