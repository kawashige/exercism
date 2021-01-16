pub fn is_armstrong_number(num: u32) -> bool {
    let s = num.to_string();
    let p = s.len() as u32;
    s.chars()
        .map(|c| c.to_digit(10).unwrap().pow(p))
        .sum::<u32>()
        == num
}
