use std::collections::HashMap;

pub struct Triangle {
    sides: HashMap<u64, usize>,
}

impl Triangle {
    pub fn build(sides: [u64; 3]) -> Option<Triangle> {
        let sum: u64 = sides.iter().sum();
        if sides.iter().any(|s| sum - s < *s) {
            return None;
        }
        sides
            .iter()
            .try_fold(HashMap::new(), |mut map, s| {
                if s == &0 {
                    None
                } else {
                    *map.entry(*s).or_insert(0) += 1;
                    Some(map)
                }
            })
            .map(|sides| Self { sides })
    }

    pub fn is_equilateral(&self) -> bool {
        self.sides.len() == 1
    }

    pub fn is_scalene(&self) -> bool {
        self.sides.len() == 3
    }

    pub fn is_isosceles(&self) -> bool {
        self.sides.len() < 3
    }
}
