#[derive(Debug, PartialEq)]
pub struct Dna {
    nucleotides: Vec<char>,
}

#[derive(Debug, PartialEq)]
pub struct Rna {
    nucleotides: Vec<char>,
}

impl Dna {
    pub fn new(dna: &str) -> Result<Dna, usize> {
        let mut nucleotides = Vec::with_capacity(dna.len());
        for (i, c) in dna.chars().enumerate() {
            if !['A', 'C', 'G', 'T'].contains(&c) {
                return Err(i);
            }
            nucleotides.push(c)
        }
        Ok(Self { nucleotides })
    }

    pub fn into_rna(self) -> Rna {
        Rna {
            nucleotides: self
                .nucleotides
                .iter()
                .map(|n| match n {
                    'G' => 'C',
                    'C' => 'G',
                    'T' => 'A',
                    _ => 'U',
                })
                .collect(),
        }
    }
}

impl Rna {
    pub fn new(rna: &str) -> Result<Rna, usize> {
        let mut nucleotides = Vec::with_capacity(rna.len());
        for (i, c) in rna.chars().enumerate() {
            if !['A', 'C', 'G', 'U'].contains(&c) {
                return Err(i);
            }
            nucleotides.push(c)
        }
        Ok(Self { nucleotides })
    }
}
