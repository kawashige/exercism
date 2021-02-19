pub struct RailFence {
    rails: usize,
}

impl RailFence {
    pub fn new(rails: u32) -> RailFence {
        Self {
            rails: rails as usize,
        }
    }

    pub fn encode(&self, text: &str) -> String {
        let mut v = vec![vec![]; self.rails];
        for (c, i) in text
            .chars()
            .zip((0..self.rails).chain((1..(self.rails - 1)).rev()).cycle())
        {
            v[i].push(c);
        }
        v.into_iter()
            .map(|vv| vv.into_iter().collect::<String>())
            .collect()
    }

    pub fn decode(&self, cipher: &str) -> String {
        let mut v = vec![vec![' '; cipher.len()]; self.rails];
        let mut pos = (0..self.rails)
            .chain((1..(self.rails - 1)).rev())
            .cycle()
            .take(cipher.len())
            .enumerate()
            .collect::<Vec<(usize, usize)>>();
        pos.sort_unstable_by(|a, b| a.1.cmp(&b.1).then(a.0.cmp(&b.0)));

        for (c, (j, i)) in cipher.chars().zip(pos.into_iter()) {
            v[i][j] = c;
        }

        (0..cipher.len())
            .map(|i| {
                (0..self.rails)
                    .find_map(|j| {
                        if v[j][i].is_whitespace() {
                            None
                        } else {
                            Some(v[j][i])
                        }
                    })
                    .unwrap()
            })
            .collect()
    }
}
