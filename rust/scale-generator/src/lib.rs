// You should change this.
//
// Depending on your implementation, there are a variety of potential errors
// which might occur. They aren't checked by the test suite in order to
// allow the greatest freedom of implementation, but real libraries should
// provide useful, descriptive errors so that downstream code can react
// appropriately.
//
// One common idiom is to define an Error enum which wraps all potential
// errors. Another common idiom is to use a helper type such as failure::Error
// which does more or less the same thing but automatically.
pub type Error = ();

pub struct Scale {
    tonic: String,
    intervals: Vec<usize>,
}

impl Scale {
    const SHARPS: [&'static str; 12] = [
        "A", "A#", "B", "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#",
    ];
    const FLATS: [&'static str; 12] = [
        "A", "Bb", "B", "C", "Db", "D", "Eb", "E", "F", "Gb", "G", "Ab",
    ];

    pub fn new(tonic: &str, intervals: &str) -> Result<Scale, Error> {
        Ok(Self {
            tonic: tonic.to_string(),
            intervals: intervals
                .chars()
                .map(|c| match c {
                    'm' => 1,
                    'M' => 2,
                    _ => 3,
                })
                .collect(),
        })
    }

    pub fn chromatic(tonic: &str) -> Result<Scale, Error> {
        Ok(Self {
            tonic: tonic.to_string(),
            intervals: vec![1; 11],
        })
    }

    pub fn enumerate(&self) -> Vec<String> {
        let scale = if Self::is_sharp(self.tonic.as_str()) {
            Self::SHARPS
        } else {
            Self::FLATS
        };

        let key = self
            .tonic
            .chars()
            .enumerate()
            .map(|(i, c)| match (i, c.is_ascii_alphabetic()) {
                (0, true) => c.to_ascii_uppercase(),
                (1, true) => c.to_ascii_lowercase(),
                _ => c,
            })
            .collect::<String>();

        let mut result = vec![];
        if let Some(i) = (0..Self::SHARPS.len()).find(|i| scale[*i] == key) {
            result.push(scale[i].to_string());
            let mut i = i;
            for j in &self.intervals {
                i = (i + j) % scale.len();
                if result[0] != scale[i] {
                    result.push(scale[i].to_string());
                }
            }
        }
        result
    }

    fn is_sharp(tonic: &str) -> bool {
        [
            "C", "G", "D", "A", "a", "E", "B", "F#", "e", "b", "f#", "c#", "g#", "d#",
        ]
        .contains(&tonic)
    }
}
