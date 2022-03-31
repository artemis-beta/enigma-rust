use std::collections::HashMap;

pub struct Plugboard {
    conversions: HashMap<char, char>
}

impl Clone for Plugboard {
    fn clone(&self) -> Self {
        Self {
            conversions: self.conversions.clone()
        }
    }
}

impl Plugboard {
    pub fn new() -> Self {
        Self {
            conversions: [
                ('A', 'Z'), ('B', 'P'), ('C', 'M'),
                ('D', 'S'), ('E', 'Y'), ('F', 'U'),
                ('G', 'N'), ('H', 'V'), ('I', 'Q'),
                ('J', 'X'), ('K', 'T'), ('L', 'R'),
                ('M', 'C'), ('N', 'G'), ('O', 'W'),
                ('P', 'B'), ('Q', 'I'), ('R', 'L'),
                ('S', 'D'), ('T', 'K'), ('U', 'F'),
                ('V', 'H'), ('W', 'O'), ('X', 'J'),
                ('Y', 'E'), ('Z', 'A')
            ].iter().cloned().collect()
        }
    }

    pub fn convert(&self, letter: char) -> char {
        self.conversions[&letter]
    }

    pub fn convert_inv(&self, letter: char) -> char {
        for (key, value) in &self.conversions {
            let output = &letter;
            if value == output {
                return key.clone();
            }
        }

        panic!("Could find inverse conversion of character '{}'", letter);
    }

    fn _get_mutable_conversion_for_key(&mut self, letter: &char) -> &mut char {
        match self.conversions.get_mut(letter) {
            Some(c) => return c,
            None => panic!("Failed to retrieve conversion for letter '{}'", letter)
        }
    }

    pub fn swap_letter_wiring(mut self, letter_1: char, letter_2: char) {
        let init_1: char = self.convert(letter_1);
        let init_2: char = self.convert(letter_2);
        *self._get_mutable_conversion_for_key(&letter_1) = letter_2;
        *self._get_mutable_conversion_for_key(&letter_2) = letter_1;
        *self._get_mutable_conversion_for_key(&init_2) = init_1;
        *self._get_mutable_conversion_for_key(&init_1) = init_2;
    }
}
