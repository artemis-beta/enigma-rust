use std::collections::HashMap;

pub const ALPHA: [char; 26] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I',
    'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R',
    'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z'
];

pub fn alpha_index(letter: char) -> usize {
    let index = ALPHA.iter().position(|&x| x == letter);

    match index {
        Some(x) => x,
        None => panic!("Cannot get index of invalid alpha character {}", letter)
    }
}

pub struct Rotor {
    notches: Vec<char>,
    wiring: HashMap<i32, i32>,
    face: char,
    name: String
}

impl PartialEq for Rotor {
    fn eq(&self, other: &Self) -> bool {
        (self.notches == other.notches) &&
        (self.wiring == other.wiring) &&
        (self.face == other.face) &&
        (self.name == other.name)
    }
}

impl Rotor {
    pub fn rotate(&mut self, other: Option<Rotor>) {
        let mut pos: usize = alpha_index(self.face);
        pos = if pos > 24 {0} else {pos+1};
        self.face = ALPHA[pos];

        match other {
            Some(mut other) => other.rotate(None),
            None => ()
        }
    }

    pub fn get_face_letter(&self) -> char {
        self.face.clone()
    }

    pub fn get_notches(&self) -> Vec<char> {
        self.notches.clone()
    }

    // RINGSTELLUNG
    pub fn rotate_inner_ring(&mut self) {
        let x: i32 = self.wiring[&0];
        for i in 0..self.wiring.keys().len() {
            if i == 25 {
                *self.wiring.get_mut(&(i as i32)).unwrap() = x;
            }
            else {
                *self.wiring.get_mut(&(i as i32)).unwrap() = self.wiring[&(i as i32+1)];
            }
        }
    }

    pub fn get_output_terminal(self, letter: char) -> i32 {
        let pos: usize = alpha_index(letter);
        self.wiring[&(pos as i32)]
    }

    pub fn get_input_terminal(self, letter: char) -> i32 {
        let pos: usize = alpha_index(letter);
        for i in 0..self.wiring.keys().len() {
            if i == pos {
                return i as i32;
            }
        }
        panic!("Could not find Input terminal for letter {}", letter);
    }

    pub fn convert(&self, letter: char) -> char {
        let pos: usize = alpha_index(letter);
        ALPHA[self.wiring[&(pos as i32)] as usize]
    }

    pub fn convert_inv(&self, letter: char) -> char {
        let pos: usize = alpha_index(letter);

        for i in 0..self.wiring.keys().len() {
            if pos as i32 == self.wiring[&(i as i32)] {
                return ALPHA[i as usize];
            }
        }

        panic!("Could not find the inverse of character '{}'", letter);
    }

    pub fn Rotor_1() -> Self {
        let mut notches = Vec::new();
        notches.push('R');
        let wiring: HashMap<i32, i32> = [
            (0, 4), (1, 10), (2, 12),
			(3, 5), (4, 11), (5, 6),
			(6, 3), (7, 16), (8, 21),
			(9, 25), (10, 13), (11, 19),
			(12, 14), (13, 22), (14, 24),
			(15, 7), (16, 23), (17, 20),
			(18, 18), (19, 15), (20, 0),
			(21, 8), (22, 1), (23, 17),
			(24, 2), (25, 9)
        ].iter().cloned().collect();
        
        Self {
            name: "I".to_string(),
            notches: notches,
            face: 'A',
            wiring: wiring
        }
    }

    pub fn Rotor_2() -> Self {
        let mut notches = Vec::new();
        notches.push('F');
        let wiring: HashMap<i32, i32> = [
            (0, 0), (1, 9), (2, 3),
			(3, 10), (4, 18), (5, 8),
			(6, 17), (7, 20), (8, 23),
			(9, 1), (10, 11), (11, 7),
			(12, 22), (13, 19), (14, 12),
			(15, 2), (16, 16), (17, 6),
			(18, 25), (19, 13), (20, 15),
			(21, 24), (22, 5), (23, 21),
			(24, 14), (25, 4)
        ].iter().cloned().collect();
        
        Self {
            name: "II".to_string(),
            notches: notches,
            face: 'A',
            wiring: wiring
        }
    }

    pub fn Rotor_3() -> Self {
        let mut notches = Vec::new();
        notches.push('W');
        let wiring: HashMap<i32, i32> = [
            (0, 1), (1, 3), (2, 5),
			(3, 7), (4, 9), (5, 11),
			(6, 2), (7, 15), (8, 17),
			(9, 19), (10, 23), (11, 21),
			(12, 25), (13, 13), (14, 24),
			(15, 4), (16, 8), (17, 22),
			(18, 6), (19, 0), (20, 10),
			(21, 12), (22, 20), (23, 18),
			(24, 16), (25, 14)
        ].iter().cloned().collect();
        
        Self {
            name: "III".to_string(),
            notches: notches,
            face: 'A',
            wiring: wiring
        }
    }

    pub fn Rotor_4() -> Self {
        let mut notches = Vec::new();
        notches.push('K');
        let wiring: HashMap<i32, i32> = [
            (0, 4), (1, 18), (2, 14),
			(3, 21), (4, 15), (5, 25),
			(6, 9), (7, 0), (8, 24),
			(9, 16), (10, 20), (11, 8),
			(12, 17), (13, 7), (14, 23),
			(15, 11), (16, 13), (17, 5),
			(18, 19), (19, 6), (20, 10),
			(21, 3), (22, 2), (23, 12),
			(24, 22), (25, 1)
        ].iter().cloned().collect();
        
        Self {
            name: "IV".to_string(),
            notches: notches,
            face: 'A',
            wiring: wiring
        }
    }

    pub fn Rotor_5() -> Self {
        let mut notches = Vec::new();
        notches.push('A');
        let wiring: HashMap<i32, i32> = [
            (0, 21), (1, 25), (2, 1),
            (3, 17), (4, 6), (5, 8),
            (6, 19), (7, 24), (8, 20),
            (9, 15), (10, 18), (11, 3),
            (12, 13), (13, 7), (14, 11),
            (15, 23), (16, 0), (17, 22),
            (18, 12), (19, 9), (20, 16),
            (21, 14), (22, 5), (23, 4),
            (24, 2), (25, 10)
        ].iter().cloned().collect();
        
        Self {
            name: "V".to_string(),
            notches: notches,
            face: 'A',
            wiring: wiring
        }
    }

    pub fn Rotor_6() -> Self {
        let mut notches = Vec::new();
        notches.push('A');
        notches.push('N');
        let wiring: HashMap<i32, i32> = [
            (0, 9), (1, 15), (2, 6),
            (3, 21), (4, 14), (5, 20),
            (6, 12), (7, 5), (8, 24),
            (9, 16), (10, 1), (11, 4),
            (12, 13), (13, 7), (14, 25),
            (15, 17), (16, 3), (17, 10),
            (18, 0), (19, 18), (20, 23),
            (21, 11), (22, 8), (23, 2),
            (24, 19), (25, 22)
        ].iter().cloned().collect();
        
        Self {
            name: "VI".to_string(),
            notches: notches,
            face: 'A',
            wiring: wiring
        }
    }

    pub fn Rotor_7() -> Self {
        let mut notches = Vec::new();
        notches.push('A');
        notches.push('N');
        let wiring: HashMap<i32, i32> = [
            (0, 13), (1, 25), (2, 9),
            (3, 7), (4, 6), (5, 17),
            (6, 2), (7, 23), (8, 12),
            (9, 24), (10, 18), (11, 22),
            (12, 1), (13, 14), (14, 20),
            (15, 5), (16, 0), (17, 8),
            (18, 21), (19, 11), (20, 15),
            (21, 4), (22, 10), (23, 16),
            (24, 3), (25, 19)
        ].iter().cloned().collect();
        
        Self {
            name: "VII".to_string(),
            notches: notches,
            face: 'A',
            wiring: wiring
        }
    }

    pub fn Rotor_8() -> Self {
        let mut notches = Vec::new();
        notches.push('A');
        notches.push('N');
        let wiring: HashMap<i32, i32> = [
            (0, 5), (1, 10), (2, 16),
            (3, 7), (4, 19), (5, 11),
            (6, 23), (7, 14), (8, 2),
            (9, 1), (10, 9), (11, 18),
            (12, 15), (13, 3), (14, 25),
            (15, 17), (16, 0), (17, 12),
            (18, 4), (19, 22), (20, 13),
            (21, 8), (22, 20), (23, 24),
            (24, 6), (25, 21)
        ].iter().cloned().collect();
        
        Self {
            name: "VIII".to_string(),
            notches: notches,
            face: 'A',
            wiring: wiring
        }
    }
}

pub fn Rotors(rotor_type: i32) -> Rotor {
    match rotor_type {
        1 => Rotor::Rotor_1(),
        2 => Rotor::Rotor_2(),
        3 => Rotor::Rotor_3(),
        4 => Rotor::Rotor_4(),
        5 => Rotor::Rotor_5(),
        6 => Rotor::Rotor_6(),
        7 => Rotor::Rotor_7(),
        8 => Rotor::Rotor_8(),
        _ => panic!("No rotor for id {}", rotor_type)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_rotor_1_conv() {
        let rotor = super::Rotor::Rotor_1();
        let input = [
            'A', 'B', 'C', 'D', 'E', 'F', 'G',
            'H', 'I', 'J', 'K', 'L', 'M', 'N',
            'O', 'P', 'Q', 'R', 'S', 'T', 'U',
            'V', 'W', 'X', 'Y', 'Z'
        ];

        let output = [
            'E', 'K', 'M', 'F', 'L', 'G', 'D',
            'Q', 'V', 'Z', 'N', 'T', 'O', 'W',
            'Y', 'H', 'X', 'U', 'S', 'P', 'A',
            'I', 'B', 'R', 'C', 'J'
        ];

        for i in 0..input.len() {
            assert!(rotor.convert(input[i]) == output[i]);
        }
    }

    #[test]
    fn test_rotor_2_conv() {
        let rotor = super::Rotor::Rotor_2();
        let input = [
            'A', 'B', 'C', 'D', 'E', 'F', 'G',
            'H', 'I', 'J', 'K', 'L', 'M', 'N',
            'O', 'P', 'Q', 'R', 'S', 'T', 'U',
            'V', 'W', 'X', 'Y', 'Z'
        ];

        let output = [
            'A', 'J', 'D', 'K', 'S', 'I', 'R',
            'U', 'X', 'B', 'L', 'H', 'W', 'T', 
            'M', 'C', 'Q', 'G', 'Z', 'N', 'P',
            'Y', 'F', 'V', 'O', 'E'
        ];

        for i in 0..input.len() {
            assert!(rotor.convert(input[i]) == output[i]);
        }
    }

    #[test]
    fn test_rotor_3_conv() {
        let rotor = super::Rotor::Rotor_3();
        let input = [
            'A', 'B', 'C', 'D', 'E', 'F', 'G',
            'H', 'I', 'J', 'K', 'L', 'M', 'N',
            'O', 'P', 'Q', 'R', 'S', 'T', 'U',
            'V', 'W', 'X', 'Y', 'Z'
        ];

        let output = [
            'B', 'D', 'F', 'H', 'J', 'L', 'C',
            'P', 'R', 'T', 'X', 'V', 'Z', 'N',
            'Y', 'E', 'I', 'W', 'G', 'A', 'K',
            'M', 'U', 'S', 'Q', 'O'
        ];

        for i in 0..input.len() {
            assert!(rotor.convert(input[i]) == output[i]);
        }
    }

    #[test]
    fn test_rotor_4_conv() {
        let rotor = super::Rotor::Rotor_4();
        let input = [
            'A', 'B', 'C', 'D', 'E', 'F', 'G',
            'H', 'I', 'J', 'K', 'L', 'M', 'N',
            'O', 'P', 'Q', 'R', 'S', 'T', 'U',
            'V', 'W', 'X', 'Y', 'Z'
        ];

        let output = [
            'E', 'S', 'O', 'V', 'P', 'Z', 'J',
            'A', 'Y', 'Q', 'U', 'I', 'R', 'H',
            'X', 'L', 'N', 'F', 'T', 'G', 'K',
            'D', 'C', 'M', 'W', 'B'
        ];

        for i in 0..input.len() {
            assert!(rotor.convert(input[i]) == output[i]);
        }
    }

    #[test]
    fn test_rotor_5_conv() {
        let rotor = super::Rotor::Rotor_5();
        let input = [
            'A', 'B', 'C', 'D', 'E', 'F', 'G',
            'H', 'I', 'J', 'K', 'L', 'M', 'N',
            'O', 'P', 'Q', 'R', 'S', 'T', 'U',
            'V', 'W', 'X', 'Y', 'Z'
        ];

        let output = [
            'V', 'Z', 'B', 'R', 'G', 'I', 'T',
            'Y', 'U', 'P', 'S', 'D', 'N', 'H',
            'L', 'X', 'A', 'W', 'M', 'J', 'Q',
            'O', 'F', 'E', 'C', 'K'
        ];

        for i in 0..input.len() {
            assert!(rotor.convert(input[i]) == output[i]);
        }
    }

    #[test]
    fn test_rotor_6_conv() {
        let rotor = super::Rotor::Rotor_6();
        let input = [
            'A', 'B', 'C', 'D', 'E', 'F', 'G',
            'H', 'I', 'J', 'K', 'L', 'M', 'N',
            'O', 'P', 'Q', 'R', 'S', 'T', 'U',
            'V', 'W', 'X', 'Y', 'Z'
        ];

        let output = [
            'J', 'P', 'G', 'V', 'O', 'U', 'M',
            'F', 'Y', 'Q', 'B', 'E', 'N', 'H',
            'Z', 'R', 'D', 'K', 'A', 'S', 'X',
            'L', 'I', 'C', 'T', 'W'
        ];

        for i in 0..input.len() {
            assert!(rotor.convert(input[i]) == output[i]);
        }
    }

    #[test]
    fn test_rotor_7_conv() {
        let rotor = super::Rotor::Rotor_7();
        let input = [
            'A', 'B', 'C', 'D', 'E', 'F', 'G',
            'H', 'I', 'J', 'K', 'L', 'M', 'N',
            'O', 'P', 'Q', 'R', 'S', 'T', 'U',
            'V', 'W', 'X', 'Y', 'Z'
        ];

        let output = [
            'N', 'Z', 'J', 'H', 'G', 'R', 'C',
            'X', 'M', 'Y', 'S', 'W', 'B', 'O',
            'U', 'F', 'A', 'I', 'V', 'L', 'P',
            'E', 'K', 'Q', 'D', 'T'
        ];

        for i in 0..input.len() {
            assert!(rotor.convert(input[i]) == output[i]);
        }
    }

    #[test]
    fn test_rotor_8_conv() {
        let rotor = super::Rotor::Rotor_8();
        let input = [
            'A', 'B', 'C', 'D', 'E', 'F', 'G',
            'H', 'I', 'J', 'K', 'L', 'M', 'N',
            'O', 'P', 'Q', 'R', 'S', 'T', 'U',
            'V', 'W', 'X', 'Y', 'Z'
        ];

        let output = [
            'F', 'K', 'Q', 'H', 'T', 'L', 'X',
            'O', 'C', 'B', 'J', 'S', 'P', 'D',
            'Z', 'R', 'A', 'M', 'E', 'W', 'N',
            'I', 'U', 'Y', 'G', 'V'
        ];

        for i in 0..input.len() {
            assert!(rotor.convert(input[i]) == output[i]);
        }
    }
}
