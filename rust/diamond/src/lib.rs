pub fn get_diamond(c: char) -> Vec<String> {
    let max = c as usize - 0x41;
    let l = max * 2 + 1;
    let mut result = vec![vec![' '; l]; l];

    for i in 0..=max {
        let ch = (i as u8 + 0x41) as char;
        result[i][max + i] = ch;
        result[i][max - i] = ch;
        result[l - 1 - i][max + i] = ch;
        result[l - 1 - i][max - i] = ch;
    }

    result
        .into_iter()
        .map(|chars| chars.into_iter().collect::<String>())
        .collect()
}
