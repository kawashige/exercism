use regex::Regex;

pub fn number(user_number: &str) -> Option<String> {
    let rule =
        Regex::new(r"^(?:\+?1 ?)?\(?([2-9]\d{2})\)?[- \.]? *([2-9]\d{2})[- \.]? *(\d{4}) *$")
            .unwrap();
    if let Some(caps) = rule.captures(user_number) {
        println!("caps: {:?}", caps);
        Some(format!(
            "{}{}{}",
            caps.get(1).map_or("", |m| m.as_str()),
            caps.get(2).map_or("", |m| m.as_str()),
            caps.get(3).map_or("", |m| m.as_str()),
        ))
    } else {
        None
    }
}
