use std::collections::HashMap;

pub struct CodonsInfo<'a> {
    map: HashMap<&'a str, &'a str>,
}

impl<'a> CodonsInfo<'a> {
    pub fn name_for(&self, codon: &str) -> Option<&'a str> {
        self.map.get(codon).cloned()
    }

    pub fn of_rna(&self, rna: &str) -> Option<Vec<&'a str>> {
        let mut result = Vec::new();
        for chars in rna.chars().collect::<Vec<char>>().chunks(3) {
            let s = chars.into_iter().collect::<String>();
            if let Some(v) = self.map.get(s.as_str()).cloned() {
                if v == "stop codon" {
                    break;
                }
                result.push(v);
            } else {
                return None;
            }
        }
        Some(result)
    }
}

pub fn parse<'a>(pairs: Vec<(&'a str, &'a str)>) -> CodonsInfo<'a> {
    let map = pairs.into_iter().collect::<HashMap<_, _>>();
    CodonsInfo { map }
}
