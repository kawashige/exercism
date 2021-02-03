pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let row_maxs: Vec<u64> = input
        .iter()
        .map(|row| *row.iter().max().unwrap_or(&0))
        .collect();
    let column_mins: Vec<u64> = (0..input[0].len())
        .map(|i| (0..input.len()).map(|j| input[j][i]).min().unwrap_or(0))
        .collect();

    let mut result = Vec::new();
    for i in 0..input.len() {
        for j in 0..input[0].len() {
            if input[i][j] == row_maxs[i] && input[i][j] == column_mins[j] {
                result.push((i, j));
            }
        }
    }
    result
}
