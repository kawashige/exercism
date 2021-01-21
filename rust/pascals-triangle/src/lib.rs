use std::slice::Windows;

pub struct PascalsTriangle {
    rows: Vec<Vec<u32>>,
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        let row_count = row_count as usize;
        let mut rows: Vec<Vec<u32>> = Vec::with_capacity(row_count);
        for i in 0..row_count {
            rows.push(match i {
                0 => vec![1],
                1 => vec![1, 1],
                _ => std::iter::once(1)
                    .chain(rows[i - 1].windows(2).map(|w| w[0] + w[1]))
                    .chain(std::iter::once(1))
                    .collect(),
            });
        }
        Self { rows }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        self.rows.clone()
    }
}
