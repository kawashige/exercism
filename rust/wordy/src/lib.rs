use regex::Regex;

pub struct WordProblem;

pub fn answer(command: &str) -> Option<i32> {
    let r1 = Regex::new(r"What is (?:(-?\d+)(.*))\?").unwrap();
    let r2 = Regex::new(r"^ (plus|minus|multiplied by|divided by) (-?\d+)(.*)$").unwrap();

    if let Some(caps) = r1.captures(command) {
        let mut num = caps
            .get(1)
            .map_or(0, |m| m.as_str().parse::<i32>().unwrap());
        let mut rest = caps.get(2).map_or("", |m| m.as_str());

        while !rest.is_empty() {
            if let Some(caps) = r2.captures(rest) {
                let num2 = caps
                    .get(2)
                    .map_or(0, |m| m.as_str().parse::<i32>().unwrap());
                match caps.get(1).map_or("", |m| m.as_str()) {
                    "plus" => num += num2,
                    "minus" => num -= num2,
                    "multiplied by" => num *= num2,
                    "divided by" => num /= num2,
                    _ => return None,
                }
                rest = caps.get(3).map_or("", |m| m.as_str());
            } else {
                return None;
            }
        }
        Some(num)
    } else {
        None
    }
}
