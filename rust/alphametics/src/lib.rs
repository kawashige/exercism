use std::collections::HashMap;

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let (lhs, rhs) = {
        let mut s = input.split(" == ");
        (
            s.next()
                .unwrap()
                .split(" + ")
                .fold(Vec::new(), |mut acc, x| {
                    for (i, c) in x.chars().rev().enumerate() {
                        if acc.len() <= i {
                            acc.push(vec![c]);
                        } else {
                            acc[i].push(c);
                        }
                    }
                    acc
                }),
            s.next().unwrap().chars().rev().collect::<Vec<char>>(),
        )
    };

    let mut char_map: HashMap<char, u8> = input
        .chars()
        .filter(|c| c.is_ascii_alphabetic())
        .map(|c| (c, 10))
        .collect();
    let chars = lhs
        .iter()
        .chain(std::iter::once(&rhs))
        .fold(Vec::new(), |mut acc, v| {
            for c in v {
                if !acc.contains(c) {
                    acc.push(*c);
                }
            }
            acc
        });
    let mut num_used = [false; 10];

    if lhs.len() < rhs.len() {
        *char_map.get_mut(&rhs[rhs.len() - 1]).unwrap() = 1;
        num_used[1] = true;
    }

    if dfs(&mut char_map, &mut num_used, &chars, &lhs, &rhs) {
        Some(char_map)
    } else {
        None
    }
}

fn dfs(
    char_map: &mut HashMap<char, u8>,
    num_used: &mut [bool; 10],
    chars: &Vec<char>,
    lhs: &Vec<Vec<char>>,
    rhs: &Vec<char>,
) -> bool {
    match check(char_map, lhs, rhs) {
        0 => return true,
        1 => return false,
        _ => {}
    }

    for k in chars {
        if char_map[&k] != 10 {
            continue;
        }
        for j in 0..10 {
            if num_used[j] {
                continue;
            }
            num_used[j] = true;
            *char_map.get_mut(&k).unwrap() = j as u8;
            if dfs(char_map, num_used, chars, lhs, rhs) {
                return true;
            }
            num_used[j] = false;
            *char_map.get_mut(&k).unwrap() = 10;
        }
    }
    false
}

fn check(char_map: &HashMap<char, u8>, lhs: &Vec<Vec<char>>, rhs: &Vec<char>) -> usize {
    if lhs[lhs.len() - 1]
        .iter()
        .chain(std::iter::once(&rhs[rhs.len() - 1]))
        .any(|c| char_map[&c] == 0)
    {
        return 1;
    }

    let mut c = 0;
    for i in 0..rhs.len() {
        if (i < lhs.len() && lhs[i].iter().any(|c| char_map[c] == 10)) || char_map[&rhs[i]] == 10 {
            return 2;
        }
        let sum: u64 = if lhs.len() - 1 < i {
            0
        } else {
            lhs[i].iter().map(|c| char_map[c] as u64).sum()
        } + c;
        if sum % 10 != char_map[&rhs[i]] as u64 {
            return 1;
        }
        c = sum / 10;
    }
    0
}
