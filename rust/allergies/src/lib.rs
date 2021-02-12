use strum::IntoEnumIterator;
use strum_macros::EnumIter;

pub struct Allergies {
    score: u32,
}

#[derive(EnumIter, Debug, PartialEq, Clone)]
pub enum Allergen {
    Eggs,
    Peanuts,
    Shellfish,
    Strawberries,
    Tomatoes,
    Chocolate,
    Pollen,
    Cats,
}

impl Allergies {
    pub fn new(score: u32) -> Self {
        Self { score }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        0 < self.score & (1 << allergen.clone() as u32)
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        Allergen::iter()
            .filter(|allergen| 0 < self.score & (1 << allergen.clone() as u32))
            .collect()
    }
}
