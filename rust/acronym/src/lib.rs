pub fn abbreviate(phrase: &str) -> String {
    let mut result = String::new();
    let mut prev_c = None;
    for c in phrase.chars() {
        if !" -_".contains(c)
            && (prev_c.is_none()
                || prev_c == Some(' ')
                || prev_c == Some('_')
                || prev_c == Some('-')
                || (prev_c.as_ref().unwrap().is_ascii_lowercase() && c.is_ascii_uppercase()))
        {
            result.push(c.to_ascii_uppercase());
        }
        prev_c = Some(c);
    }
    result
}
