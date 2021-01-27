use std::cmp::Ordering;
use std::collections::HashMap;

/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.
#[derive(PartialEq)]
struct Hand<'a> {
    score: Score,
    src: &'a str,
}
#[derive(Ord, Eq, PartialEq, PartialOrd, Debug)]
enum Score {
    RoyalStraightFlush,
    StraightFlush(u8),
    FourCard(u8, u8),
    FullHouse(u8, u8),
    Flash(u8, u8, u8, u8, u8),
    Straight(u8),
    FiveHighStraight,
    ThreeCards(u8, u8, u8),
    TwoPairs(u8, u8, u8),
    OnePair(u8, u8, u8, u8),
    NoHand(u8, u8, u8, u8, u8),
}

impl<'a> From<&'a str> for Hand<'a> {
    fn from(src: &'a str) -> Self {
        let mut v: Vec<(u8, char)> = src
            .split_whitespace()
            .take(5)
            .map(|src| {
                let (num, mark) = src.split_at(src.len() - 1);
                (
                    match num {
                        "A" => 0,
                        "K" => 1,
                        "Q" => 2,
                        "J" => 3,
                        _ => 14 - num.parse::<u8>().unwrap(),
                    } as u8,
                    mark.chars().next().unwrap(),
                )
            })
            .collect();

        let counts = v.iter().fold(HashMap::new(), |mut map, c| {
            *map.entry(c.0).or_insert(0) += 1;
            map
        });
        v.sort_by_key(|c| (-counts[&c.0], c.0));

        let is_flush = (1..v.len()).all(|i| v[0].1 == v[i].1);

        Self {
            score: Self::score([v[0].0, v[1].0, v[2].0, v[3].0, v[4].0], is_flush),
            src,
        }
    }
}

impl<'a> PartialOrd for Hand<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.score.cmp(&other.score))
    }
}

impl<'a> Hand<'a> {
    pub fn score(nums: [u8; 5], is_flush: bool) -> Score {
        match (nums, is_flush) {
            // Royal Straight Flush
            (v, true) if Self::is_straight(v) && v[0] == 14 => Score::RoyalStraightFlush,
            // Straight Flush
            (v, true) if Self::is_straight(v) => Score::StraightFlush(v[0]),
            // Four Cards
            (v, _) if v[0] == v[1] && v[0] == v[2] && v[0] == v[3] => Score::FourCard(v[0], v[4]),
            // Full House
            (v, _) if v[0] == v[1] && v[0] == v[2] && v[3] == v[4] => Score::FullHouse(v[0], v[3]),
            // Flash
            (v, true) => Score::Flash(v[0], v[1], v[2], v[3], v[4]),
            // 5-high Straight
            (v, _) if Self::is_straight(v) && v[0] == 0 && v[1] == 9 => Score::FiveHighStraight,
            // Straight
            (v, _) if Self::is_straight(v) => Score::Straight(v[0]),
            // Three Cards
            (v, _) if v[0] == v[1] && v[0] == v[2] => Score::ThreeCards(v[0], v[3], v[4]),
            // Two Pair
            (v, _) if v[0] == v[1] && v[2] == v[3] => Score::TwoPairs(v[0], v[2], v[4]),
            // One Pair
            (v, _) if v[0] == v[1] => Score::OnePair(v[0], v[2], v[3], v[4]),
            // No hand
            _ => Score::NoHand(nums[0], nums[1], nums[2], nums[3], nums[4]),
        }
    }

    fn is_straight(nums: [u8; 5]) -> bool {
        nums == [0, 9, 10, 11, 12] || (1..5).all(|i| nums[i] == nums[0] + i as u8)
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
