pub fn spiral_matrix(size: u32) -> Vec<Vec<u32>> {
    let mut matrix = vec![vec![0; size as usize]; size as usize];
    let moves = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    let mut d = 0;
    let mut pos = (0, -1);
    for i in 1..=(size * size) {
        while (pos.0 + moves[d].0 < 0)
            || (size as i32 - 1 < pos.0 + moves[d].0)
            || (pos.1 + moves[d].1 < 0)
            || (size as i32 - 1 < pos.1 + moves[d].1)
            || matrix[(pos.0 + moves[d].0) as usize][(pos.1 + moves[d].1) as usize] != 0
        {
            d = (d + 1) % 4;
        }
        pos = (pos.0 + moves[d].0, pos.1 + moves[d].1);
        matrix[pos.0 as usize][pos.1 as usize] = i;
    }
    matrix
}
