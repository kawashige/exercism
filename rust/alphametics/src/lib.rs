use std::collections::HashMap;

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let ((lhs, mut not_zero), rhs) = {
        let mut s = input.split(" == ");
        (
            s.next().unwrap().split(" + ").fold(
                (Vec::new(), [false; 26]),
                |(mut acc, mut not_zero), x| {
                    for (i, c) in x.chars().rev().enumerate() {
                        if i == 0 && 1 < x.len() {
                            not_zero[c as usize - 0x41] = true;
                        }
                        if acc.len() <= i {
                            acc.push(vec![c]);
                        } else {
                            acc[i].push(c);
                        }
                    }
                    (acc, not_zero)
                },
            ),
            s.next().unwrap().chars().rev().collect::<Vec<char>>(),
        )
    };
    if 0 < rhs.len() {
        not_zero[rhs[rhs.len() - 1] as usize - 0x41] = true;
    }

    let mut multiply = [0_i32; 26];
    for (i, v) in lhs.iter().enumerate() {
        for c in v {
            multiply[*c as usize - 0x41] += 10_i32.pow(i as u32);
        }
    }
    for (i, c) in rhs.iter().enumerate() {
        multiply[*c as usize - 0x41] -= 10_i32.pow(i as u32);
    }

    let mut num_used = [false; 10];
    let mut chars = [10; 26];
    let mut char_indices = multiply
        .iter()
        .enumerate()
        .filter(|(_, v)| *v != &0)
        .collect::<Vec<(usize, &i32)>>();
    char_indices.sort_by_key(|(_, v)| -**v);
    let indicies = char_indices
        .into_iter()
        .map(|(i, _)| i)
        .collect::<Vec<usize>>();

    if dfs(
        &mut chars,
        &mut num_used,
        &indicies,
        &not_zero,
        &multiply,
        &lhs,
        &rhs,
        0,
    ) {
        Some(
            chars
                .iter()
                .enumerate()
                .filter(|(_, n)| *n != &10)
                .map(|(i, n)| ((i as u8 + 0x41) as char, *n as u8))
                .collect(),
        )
    } else {
        None
    }
}

fn dfs(
    chars: &mut [usize; 26],
    num_used: &mut [bool; 10],
    indices: &Vec<usize>,
    not_zero: &[bool; 26],
    multiply: &[i32; 26],
    lhs: &Vec<Vec<char>>,
    rhs: &Vec<char>,
    sum: i64,
) -> bool {
    if sum < 0 {
        return false;
    }
    if (0..26).all(|i| multiply[i] == 0 || chars[i] != 10) {
        return sum == 0;
    }

    for i in indices {
        if chars[*i] != 10 || multiply[*i] == 0 {
            continue;
        }
        for j in 0..10 {
            if num_used[j] || (j == 0 && not_zero[*i]) {
                continue;
            }
            num_used[j] = true;
            chars[*i] = j;
            if dfs(
                chars,
                num_used,
                indices,
                not_zero,
                multiply,
                lhs,
                rhs,
                sum + multiply[*i] as i64 * j as i64,
            ) {
                return true;
            }
            num_used[j] = false;
            chars[*i] = 10;
        }
    }
    false
}
