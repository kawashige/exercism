pub fn encrypt(input: &str) -> String {
    let chars = input
        .to_ascii_lowercase()
        .chars()
        .filter(|c| c.is_alphanumeric())
        .collect::<Vec<char>>();

    let l = chars.len();
    let (r, c) = (0..l)
        .find_map(|i| {
            if l <= i * i {
                Some((i, i))
            } else if l <= i * (i + 1) {
                Some((i, i + 1))
            } else {
                None
            }
        })
        .unwrap();

    chars
        .chunks(c)
        .fold(vec![vec![]; c], |mut acc, chunk| {
            for i in 0..c {
                acc[i].push(chunk.get(i).unwrap_or(&' ').clone());
            }
            acc
        })
        .into_iter()
        .flatten()
        .collect::<Vec<char>>()
        .chunks(r)
        .map(|chunk| chunk.iter().collect())
        .collect::<Vec<String>>()
        .join(" ")
}
