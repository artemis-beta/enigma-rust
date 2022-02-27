use std::collections::HashMap;

pub struct Plugboard {
    conversions: HashMap<char, char>
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
        for value in self.conversions.values() {
            let output = &letter;
            if value == output {
                return *output;
            }
        }

        panic!("Could find inverse conversion of character '{}'", letter);
    }

    pub fn swap_letter_wiring(mut self, letter_1: char, letter_2: char) {
        let init_1: char = self.convert(letter_1);
        let init_2: char = self.convert(letter_2);
        *self.conversions.get_mut(&letter_1).unwrap() = letter_2;
        *self.conversions.get_mut(&letter_2).unwrap() = letter_1;
        *self.conversions.get_mut(&init_2).unwrap() = init_1;
        *self.conversions.get_mut(&init_1).unwrap() = init_2;
    }
}
