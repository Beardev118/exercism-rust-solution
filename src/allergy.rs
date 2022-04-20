pub struct Allergies {
    allergies: Vec<Allergen>,
}

#[derive(Debug, PartialEq, Clone, Copy)]
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
        let mut my_allergies = vec![];
        for i in 0..=7 {
            if (score >> i) & 1 == 1 {
                let item_allergy = match i {
                    0 => Allergen::Eggs,
                    1 => Allergen::Peanuts,
                    2 => Allergen::Shellfish,
                    3 => Allergen::Strawberries,
                    4 => Allergen::Tomatoes,
                    5 => Allergen::Chocolate,
                    6 => Allergen::Pollen,
                    _ => Allergen::Cats,
                };
                my_allergies.push(item_allergy);
            }
        }
        return Allergies {
            allergies: my_allergies,
        };
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        return self.allergies.contains(allergen);
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        let mut allergy_val: Vec<Allergen> = vec![];
        for item in self.allergies.iter() {
            allergy_val.push(*item);
        }
        return allergy_val;
    }
}
