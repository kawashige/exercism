pub fn build_proverb(list: &[&str]) -> String {
    if list.is_empty() {
        String::new()
    } else {
        (0..(list.len() - 1))
            .map(|i| format!("For want of a {} the {} was lost.", list[i], list[i + 1]))
            .chain(std::iter::once(format!(
                "And all for the want of a {}.",
                list[0]
            )))
            .collect::<Vec<String>>()
            .join("\n")
    }
}
