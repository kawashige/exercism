use std::cmp::Ordering;
use std::collections::HashMap;

/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.
#[derive(PartialEq)]
struct Hand<'a> {
    ordered_nums: [usize; 5],
    is_flash: bool,
    src: &'a str,
}

impl<'a> From<&'a str> for Hand<'a> {
    fn from(src: &'a str) -> Self {
        let mut v: Vec<(usize, char)> = src
            .split_whitespace()
            .take(5)
            .map(|src| {
                let (num, mark) = src.split_at(src.len() - 1);
                (
                    match num {
                        "A" => 14,
                        "J" => 11,
                        "Q" => 12,
                        "K" => 13,
                        _ => num.parse().unwrap(),
                    },
                    mark.chars().next().unwrap(),
                )
            })
            .collect();

        let counts = v.iter().fold(HashMap::new(), |mut map, c| {
            *map.entry(c.0).or_insert(0) += 1;
            map
        });
        v.sort_by_key(|c| (-counts[&c.0], -(c.0 as i32)));

        let is_flash = (1..v.len()).all(|i| v[0].1 == v[i].1);

        Self {
            ordered_nums: [v[0].0, v[1].0, v[2].0, v[3].0, v[4].0],
            is_flash,
            src,
        }
    }
}

impl<'a> PartialOrd for Hand<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(
            self.hand()
                .cmp(&other.hand())
                .then(other.ordered_nums.cmp(&self.ordered_nums)),
        )
    }
}

impl<'a> Hand<'a> {
    fn hand(&'a self) -> usize {
        match (self.ordered_nums, self.is_flash) {
            // Royal Straight Flash
            (v, true) if Self::is_straight(v) && v[0] == 14 => 1,
            // Straight Falsh
            (v, true) if Self::is_straight(v) => 2,
            // Four Cards
            (v, _) if v[0] == v[1] && v[0] == v[2] && v[0] == v[3] => 3,
            // Full House
            (v, _) if v[0] == v[1] && v[0] == v[2] && v[3] == v[4] => 4,
            // Flash
            (_, true) => 5,
            // 5-high Straight
            (v, _) if Self::is_straight(v) && v[0] == 14 && v[1] == 5 => 7,
            // Straight
            (v, _) if Self::is_straight(v) => 6,
            // Three Cards
            (v, _) if v[0] == v[1] && v[0] == v[2] => 8,
            // Two Pair
            (v, _) if v[0] == v[1] && v[2] == v[3] => 9,
            // One Pair
            (v, _) if v[0] == v[1] => 10,
            // No hand
            _ => 11,
        }
    }

    fn is_straight(nums: [usize; 5]) -> bool {
        nums == [14, 5, 4, 3, 2] || (1..5).all(|i| nums[0] == nums[i] + i)
    }
}

pub fn winning_hands<'a>(hands: &[&'a str]) -> Option<Vec<&'a str>> {
    if hands.is_empty() {
        return None;
    }

    let mut h: Vec<Hand> = hands.iter().map(|h| Hand::from(*h)).collect();
    h.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal));

    let mut results = vec![h[0].src];
    for i in 0..h.len() {
        if h[0].partial_cmp(&h[i]).unwrap_or(Ordering::Equal) == Ordering::Equal {
            results.push(h[i].src)
        }
    }
    Some(results)
}
