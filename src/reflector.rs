use std::collections::HashMap;

pub struct Reflector {
    conversions: HashMap<char, char>,
    name: char
}

impl Reflector {
    pub fn convert(&self, letter: char) -> char {
        self.conversions[&letter]
    }

    pub fn Reflector_B() -> Self {
        let conversions: HashMap<char, char> = [
            ('A', 'Y'), ('B', 'R'), ('C', 'U'),
			('D', 'H'), ('E', 'Q'), ('F', 'S'),
			('G', 'L'), ('H', 'D'), ('K', 'N'),
			('L', 'G'), ('M', 'O'), ('N', 'K'),
			('O', 'M'), ('P', 'I'), ('Q', 'E'),
			('R', 'B'), ('S', 'F'), ('T', 'Z'),
			('U', 'C'), ('V', 'W'), ('W', 'V'),
			('X', 'J'), ('Y', 'A'), ('Z', 'T'),
			('I', 'P'), ('J', 'X')
        ].iter().cloned().collect();

        Self {
            name: 'B',
            conversions: conversions
        }
    }

    pub fn Reflector_C() -> Self {
        let conversions: HashMap<char, char> = [
            ('A', 'F'), ('B', 'V'), ('C', 'P'),
            ('D', 'J'), ('E', 'I'), ('F', 'A'),
            ('G', 'O'), ('H', 'Y'), ('K', 'R'),
            ('L', 'Z'), ('M', 'X'), ('N', 'W'),
            ('O', 'G'), ('P', 'C'), ('Q', 'T'),
            ('R', 'K'), ('S', 'U'), ('T', 'Q'),
            ('U', 'S'), ('V', 'B'), ('W', 'N'),
            ('X', 'M'), ('Y', 'H'), ('Z', 'L'),
            ('I', 'E'), ('J', 'D')
        ].iter().cloned().collect();

        Self {
            name: 'C',
            conversions: conversions
        }
    }
}

pub fn Reflectors(reflector_type: char) -> Reflector {
    match reflector_type {
        'B' => Reflector::Reflector_B(),
        'C' => Reflector::Reflector_C(),
        _ => panic!("Unrecognised reflector type '{}'", reflector_type)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_reflector_B_conv() {
        let in_char = 'X';
        let out_char = 'J';

        let reflector = super::Reflector::Reflector_B();

        let ref_out = reflector.convert(in_char);

        assert!(ref_out == out_char);
    }

    #[test]
    fn test_reflector_C_conv() {
        let in_char = 'X';
        let out_char = 'M';

        let reflector = super::Reflector::Reflector_C();

        let ref_out = reflector.convert(in_char);

        assert!(ref_out == out_char);
    }
}
