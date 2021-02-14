use regex::Regex;

pub fn translate(input: &str) -> String {
    let rules = [
        Regex::new(r"(^(?:[aeiuo]|xr|yt).*)").unwrap(),
        Regex::new(r"^([^aeiuo]*qu)(.*)").unwrap(),
        Regex::new(r"^([^aeiuo]+)(y.*)").unwrap(),
        Regex::new(r"^([^aeiuo]+)(.*)").unwrap(),
    ];

    input
        .split_whitespace()
        .map(|s| {
            for rule in rules.iter() {
                if let Some(caps) = rule.captures(s) {
                    return format!(
                        "{}{}ay",
                        caps.get(2).map_or("", |m| m.as_str()),
                        caps.get(1).map_or("", |m| m.as_str())
                    );
                }
            }
            s.to_string()
        })
        .collect::<Vec<String>>()
        .join(" ")
}
