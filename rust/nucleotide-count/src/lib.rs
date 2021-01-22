use std::collections::HashMap;

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    let valid_bytes = "ACGT".as_bytes();

    if !valid_bytes.contains(&(nucleotide as u8)) {
        return Err(nucleotide);
    }

    let mut count = 0;
    for b in dna.as_bytes() {
        if !valid_bytes.contains(b) {
            return Err((*b).into());
        }
        if b == &(nucleotide as u8) {
            count += 1;
        }
    }
    Ok(count)
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let mut counts: HashMap<char, usize> = vec![('A', 0), ('C', 0), ('G', 0), ('T', 0)]
        .into_iter()
        .collect();
    let valid_bytes = "ACGT".as_bytes();
    for b in dna.as_bytes() {
        if !valid_bytes.contains(b) {
            return Err((*b).into());
        }
        *counts.get_mut(&(*b as char)).unwrap() += 1;
    }
    Ok(counts)
}
