pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let mut fileds = minefield
        .iter()
        .map(|s| {
            s.chars()
                .map(|c| if c == '*' { -1 } else { 0 })
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();

    let r = fileds.len();
    let c = fileds[0].len();
    for i in 0..r {
        for j in 0..c {
            if fileds[i][j] == -1 {
                for (x, y) in surronding_spaces(i, j, r, c) {
                    if fileds[x][y] != -1 {
                        fileds[x][y] += 1;
                    }
                }
            }
        }
    }

    fileds
        .into_iter()
        .map(|row| {
            row.into_iter()
                .map(|r| match r {
                    -1 => "*".to_string(),
                    0 => " ".to_string(),
                    _ => r.to_string(),
                })
                .collect::<String>()
        })
        .collect()
}

fn surronding_spaces(i: usize, j: usize, r: usize, c: usize) -> Vec<(usize, usize)> {
    vec![
        (-1_i32, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ]
    .into_iter()
    .map(|(x, y)| (x + i as i32, y + j as i32))
    .filter(|(x, y)| &0 <= x && x < &(r as i32) && &0 <= y && y < &(c as i32))
    .map(|(x, y)| (x as usize, y as usize))
    .collect()
}
