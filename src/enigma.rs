#[path = "rotor.rs"]
mod rotor;

#[path = "reflector.rs"]
mod reflector;

#[path = "plugboard.rs"]
mod plugboard;

use std::collections::VecDeque;
use std::collections::HashMap;
use std::iter::FromIterator;

pub struct Enigma {
    enigma_type: String,
    rotor_ids: Vec::<i32>,
    rotors: HashMap<String, rotor::Rotor>,
    rotor_labels: Vec::<String>,
    reflector: reflector::Reflector,
    plugboard: plugboard::Plugboard
}

impl PartialEq for Enigma {
    fn eq(&self, other: &Self) -> bool {
        (self.enigma_type == other.enigma_type) &&
        (self.rotor_ids == other.rotor_ids)
    }
}

impl Enigma {
    pub fn new(rotor_list: Vec::<i32>, reflector: char, enigma_type: String) -> Self {
        let mut rotors = HashMap::<String, rotor::Rotor>::new();
        let mut rotor_labels = Vec::<String>::new();

        if enigma_type == "M3" {
            if rotor_list.len() != 3 {
                panic!("Three rotor types only must be provided for Enigma machine 'M3'");
            }
    
            rotors.insert("left".to_string(), rotor::Rotors(rotor_list[0]));
            rotors.insert("middle".to_string(), rotor::Rotors(rotor_list[1]));
            rotors.insert("right".to_string(), rotor::Rotors(rotor_list[2]));

            rotor_labels.push("left".to_string());
            rotor_labels.push("middle".to_string());
            rotor_labels.push("right".to_string());
        }
        else if enigma_type == "M4" {
            if rotor_list.len() != 4 {
                panic!("Three rotor types only must be provided for Enigma machine 'M4'");
            }

            rotors.insert("left".to_string(), rotor::Rotors(rotor_list[0]));
            rotors.insert("middle left".to_string(), rotor::Rotors(rotor_list[1]));
            rotors.insert("middle right".to_string(), rotor::Rotors(rotor_list[2]));
            rotors.insert("right".to_string(), rotor::Rotors(rotor_list[3]));

            rotor_labels.push("left".to_string());
            rotor_labels.push("middle left".to_string());
            rotor_labels.push("middle right".to_string());
            rotor_labels.push("right".to_string());
        }
        else {
            panic!("Unrecognised Enigma type '{}'", enigma_type);
        }
        
        Self {
            rotor_ids: rotor_list,
            rotors: rotors,
            reflector: reflector::Reflectors(reflector),
            rotor_labels: rotor_labels,
            enigma_type: enigma_type,
            plugboard: plugboard::Plugboard::new(),
        }

    }

    pub fn get_rotor_labels(&self) -> Vec<String> {
        self.rotor_labels.clone()
    }

    fn rotor_index(&self, label: &String) -> usize {
        match self.rotor_labels.iter().enumerate().find(|&x| x.1 == label) {
            Some(x) => x.0,
            None => panic!("Could not find Rotor, '{}' in rotor list", label)
        }
    }

    fn _move_rotor(mut self, rotor: &String, amount: i32) -> Self {
        for _i in 0..amount {
            self.rotors.get_mut(rotor).unwrap().rotate(None);
        }
        return self
    }

    fn _set_rotor(mut self, rotor: &String, letter: char) -> Self {
        let mut face = self.rotors.get(rotor).unwrap().get_face_letter();
        let curr_index = rotor::alpha_index(face);
        let dest_index = rotor::alpha_index(letter);
        let n_rotations = if dest_index > curr_index {dest_index-curr_index} else {26 - (curr_index-dest_index)};

        self._move_rotor(rotor, n_rotations as i32)
    }

    pub fn rotor_conv(&self, rotor: &String, letter: char) -> char {
        self.rotors.get(rotor).unwrap().convert(letter)
    }

    pub fn rotor_conv_inv(&self, rotor: &String, letter: char) -> char {
        self.rotors.get(rotor).unwrap().convert_inv(letter)
    }

    pub fn reflector_conv(&self, letter: char) -> char {
        self.reflector.convert(letter)
    }

    pub fn plugboard_conv(&self, letter: char) -> char {
        self.plugboard.convert(letter)
    }

    pub fn plugboard_conv_inv(&self, letter: char) -> char {
        self.plugboard.convert_inv(letter)
    }

    pub fn inter_rotor_conv(&self, rotor_1: &String, rotor_2: &String, letter: char) -> char {
        let terminal = rotor::alpha_index(letter);
        let zero_point_1 = rotor::alpha_index(self.rotors.get(rotor_1).unwrap().get_face_letter());
        let zero_point_2 = rotor::alpha_index(self.rotors.get(rotor_2).unwrap().get_face_letter());
        let interval = zero_point_2 as i32 - zero_point_1 as i32;

        let mut n = 0;

        if zero_point_2 > zero_point_1 {
            const i: [i32; 26] = [0; 26];
            n = i[(terminal + interval as usize) % i.len()];
        }
        else {
            const i: [i32; 26] = [0; 26];
            let index: i32 = 26 + terminal as i32 + interval as i32;
            n = i[(index as usize) % i.len()];
        }

        rotor::ALPHA[n as usize]   
    }

    fn ringstellung_rotor_(mut self, rotor: &String, amount: i32) -> Self {
        for _i in 0..amount {
            self.rotors.get_mut(rotor).unwrap().rotate_inner_ring();
        }
        self
    }

    pub fn ringstellung(mut self, rsg_vec: Vec<i32>) -> Self {
        let rotor_labels = self.rotor_labels.clone();
        for i in 0..rsg_vec.len() {
            self = self.ringstellung_rotor_(&rotor_labels[i], rsg_vec[i]);
        }
        self
    }

    pub fn type_letter(mut self, mut letter: char) -> char {
        let upper_l = letter.to_ascii_uppercase();
        let rotor_labels: Vec<String> = self.rotors.keys().cloned().collect();
        let mut reversed = VecDeque::from_iter(&rotor_labels);
        let mut reversed_1 = VecDeque::from_iter(&rotor_labels);
        let mut reversed_2 = VecDeque::from_iter(&rotor_labels);

        reversed_2.pop_back();
        reversed_1.pop_front();

        let mut cipher = self.plugboard.convert(upper_l);

        let mut notch_dict = HashMap::<String, Vec<char>>::new();
        let mut face_letters = HashMap::<String, char>::new();

        for i in 0..rotor_labels.len() {
            notch_dict.insert(rotor_labels[i].clone(), self.rotors.get(&rotor_labels[i]).unwrap().get_notches());
            face_letters.insert(rotor_labels[i].clone(), self.rotors.get(&rotor_labels[i]).unwrap().get_face_letter());
        }

        for i in 0..reversed_1.len() {
            let notches = notch_dict.get(&rotor_labels[i]).unwrap();
            for notch in notches {
                if &face_letters[reversed_1[i]] == notch {
                    self = self._move_rotor(reversed_2[i], 1);
                }
            }
        }

        self = self._move_rotor(&rotor_labels[rotor_labels.len()-1], 1);

        for i in 0..reversed.len() {
            cipher = self.rotor_conv(reversed[i], cipher);
            let adj_rotor_index = self.rotor_index(reversed[i]) as i32 -1;
            if adj_rotor_index < 0 {
                break;
            }

            let adjacent_rotor = &rotor_labels[adj_rotor_index as usize];

            cipher = self.inter_rotor_conv(reversed[i], adjacent_rotor, cipher);
        }

        cipher = self.reflector_conv(cipher);

        for key in self.rotor_labels.clone() {
            cipher = self.rotor_conv_inv(&key, cipher);
        }

        self.plugboard_conv_inv(cipher)
    }

    pub fn type_phrase(mut self, phrase: String) -> String {
        let mut temp = phrase.clone();
        temp.retain(|x| !x.is_whitespace());

        let remainder = if temp.len() % 5 != 0 {5 - temp.len() % 5} else {0};

        let mut out_str: String = "".to_string();
        
        for i in 0..remainder {
            let letter = temp.chars().nth(i).unwrap();
            self.type_letter(letter);
            out_str += &letter.to_string();
            
            if i + 1 % 5 == 0 {
                out_str += " ";
            }
        }

        out_str
    }

    pub fn set_key(mut self, user_key: String) -> Self {
        let labels = self.rotor_labels.clone();
        if labels.len() != user_key.len() {
            panic!("Key length must match no. of rotors.");
        }

        let upper_k = user_key.to_ascii_uppercase();

        for i in 0..upper_k.len() {
            let letter = upper_k.chars().nth(i).unwrap();
            self = self._set_rotor(&labels[i], letter);
        }
        self
    }

    pub fn rewire_plugboard(mut self, letter_1: char, letter_2: char) {
        self.plugboard.swap_letter_wiring(letter_1, letter_2);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_type_letter() {
        let rotor_list = vec![1, 2, 3, 4];
        let enigma_type = "M4".to_string();
        let reflector = 'B';
        let mut letter = 'K';
        let before = letter.clone();
        let message = "This is a test".to_string().replace(" ", "").to_ascii_uppercase();
        let rotor_labels = vec!["left", "middle left", "middle right", "right"];
        let mut machine_1 = super::Enigma::new(rotor_list.clone(), reflector, enigma_type.clone());
        machine_1.type_letter(letter);
        assert!(letter != before);
    }

    #[test]
    fn test_ringstellung() {
        let rotor_list = vec![1, 2, 3, 4];
        let enigma_type = "M4".to_string();
        let reflector = 'B';
        let key = "TEST".to_string();
        let message = "This is a test".to_string().replace(" ", "").to_ascii_uppercase();
        let rotor_labels = vec!["left", "middle left", "middle right", "right"];
        let mut machine_1 = super::Enigma::new(rotor_list.clone(), reflector, enigma_type.clone());
        let mut machine_2 = super::Enigma::new(rotor_list.clone(), reflector, enigma_type.clone());
        
        machine_1 = machine_1.set_key(key.clone());
        machine_2 = machine_2.set_key(key.clone());

        machine_1 = machine_1.ringstellung(rotor_list.clone());
        machine_2 = machine_2.ringstellung(rotor_list.clone());

        let output = machine_1.type_phrase(message.clone()).replace(" ", "");
        let output_2 = machine_2.type_phrase(message.clone());


        println!("{}", output);
        assert!(&output[..output.len()-4] == "AIJYSDOZODD");
    }
}