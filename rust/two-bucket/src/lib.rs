use std::collections::HashSet;

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub enum Bucket {
    One,
    Two,
}

/// A struct to hold your results in.
#[derive(PartialEq, Eq, Debug)]
pub struct BucketStats {
    /// The total number of "moves" it should take to reach the desired number of liters, including
    /// the first fill.
    pub moves: u8,
    /// Which bucket should end up with the desired number of liters? (Either "one" or "two")
    pub goal_bucket: Bucket,
    /// How many liters are left in the other bucket?
    pub other_bucket: u8,
}

/// Solve the bucket problem
pub fn solve(
    capacity_1: u8,
    capacity_2: u8,
    goal: u8,
    start_bucket: &Bucket,
) -> Option<BucketStats> {
    let mut moves = 1;
    let mut next = if start_bucket == &Bucket::One {
        vec![(capacity_1, 0)]
    } else {
        vec![(0, capacity_2)]
    };
    let mut seen = HashSet::new();

    while !next.is_empty() {
        let mut new_next = Vec::new();
        for n in next {
            if seen.contains(&n) {
                continue;
            }

            if n.0 == goal || n.1 == goal {
                return Some(BucketStats {
                    moves,
                    goal_bucket: if n.0 == goal {
                        Bucket::One
                    } else {
                        Bucket::Two
                    },
                    other_bucket: if n.0 == goal { n.1 } else { n.0 },
                });
            }

            seen.insert(n);

            if 0 < n.0 && n.1 < capacity_2 {
                let moved = (
                    n.0 - std::cmp::min(n.0, capacity_2 - n.1),
                    n.1 + std::cmp::min(n.0, capacity_2 - n.1),
                );
                new_next.push(moved);
            }
            if 0 < n.1 && n.0 < capacity_1 {
                let moved = (
                    n.0 + std::cmp::min(n.1, capacity_1 - n.0),
                    n.1 - std::cmp::min(n.1, capacity_1 - n.0),
                );
                new_next.push(moved);
            }
            if 0 < n.1 && 0 < n.0 && n.0 != capacity_1 {
                new_next.push((n.0, 0));
            }
            if 0 < n.1 && 0 < n.0 && n.1 != capacity_2 {
                new_next.push((0, n.1));
            }
            if n.0 == 0 {
                new_next.push((capacity_1, n.1));
            }
            if n.1 == 0 {
                new_next.push((n.0, capacity_2));
            }
        }
        next = new_next;
        moves += 1;
    }

    None
}
