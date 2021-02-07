pub fn encode(source: &str) -> String {
    let mut num = 0;
    let mut prev = '_';
    let mut s = String::new();
    for c in source.chars() {
        if c == prev {
            num += 1;
        } else {
            if 1 < num {
                s.push_str(&num.to_string());
            }
            if num != 0 {
                s.push(prev);
            }
            num = 1;
            prev = c;
        }
    }
    if 1 < num {
        s.push_str(&num.to_string());
    }
    s.push(prev);
    s
}

pub fn decode(source: &str) -> String {
    let mut num = 0;
    let mut s = String::new();
    for c in source.chars() {
        if c.is_numeric() {
            num = num * 10 + c.to_digit(10).unwrap() as usize;
        } else {
            s.push_str(
                &std::iter::repeat(c)
                    .take(if num == 0 { 1 } else { num })
                    .collect::<String>(),
            );
            num = 0;
        }
    }
    s
}
