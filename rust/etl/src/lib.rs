use std::collections::BTreeMap;

pub fn transform(h: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    h.into_iter()
        .map(|(k, v)| {
            v.iter()
                .map(|c| (c.to_ascii_lowercase(), *k))
                .collect::<Vec<(char, i32)>>()
        })
        .flatten()
        .collect()
}
