#[derive(Debug, PartialEq)]
pub enum Allergen {
    Eggs,
    Peanuts,
    Shellfish,
    Strawberries,
    Tomatoes,
    Chocolate,
    Pollen,
    Cats
}

pub struct Allergies {
    score: usize
}

impl Allergies {
    pub fn new(score: usize) -> Allergies {
        Allergies { score: score }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        let index = self.allergens().
            iter().
            position(|_allergen| allergen == _allergen).
            unwrap();

        self.score & (1 << index) != 0
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        self.allergens().
            into_iter().
            filter(|allergen| self.is_allergic_to(allergen)).
            collect()
    }

    fn allergens(&self) -> Vec<Allergen> {
        vec![
            Allergen::Eggs,
            Allergen::Peanuts,
            Allergen::Shellfish,
            Allergen::Strawberries,
            Allergen::Tomatoes,
            Allergen::Chocolate,
            Allergen::Pollen,
            Allergen::Cats
        ]
    }
}
