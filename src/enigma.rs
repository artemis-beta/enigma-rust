#[path = "rotor.rs"]
mod rotor;

#[path = "reflector.rs"]
mod reflector;

#[path = "plugboard.rs"]
mod plugboard;

use std::collections::HashMap;
use rand::Rng;
use log::{debug};

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

impl Clone for Enigma {
    fn clone(&self) -> Enigma {
        Self {
            enigma_type: self.enigma_type.clone(),
            rotor_ids: self.rotor_ids.clone(),
            rotors: self.rotors.clone(),
            rotor_labels: self.rotor_labels.clone(),
            reflector: self.reflector.clone(),
            plugboard: self.plugboard.clone()
        }
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

    fn _get_rotor_at_position(&self, label: &String) -> &rotor::Rotor {
        match self.rotors.get(label) {
            Some(r) => return r,
            None => panic!("Failed to retrieve rotor at position '{}' from rotor list", label)
        }
    }

    fn _get_rotor_at_position_mut(&mut self, label: &String) -> &mut rotor::Rotor {
        match self.rotors.get_mut(label) {
            Some(r) => return r,
            None => panic!("Failed to retrieve mutable reference to rotor at position '{}' from rotor list", label)
        }
    }

    fn rotor_index(&self, label: &String) -> usize {
        match self.rotor_labels.iter().enumerate().find(|&x| x.1 == label) {
            Some(x) => x.0,
            None => panic!("Could not find Rotor, '{}' in rotor list", label)
        }
    }

    fn _move_rotor(&mut self, rotor: &String, amount: i32) {
        for i in 0..amount {
            debug!("[{}] Rotating rotor {} by {}", i, rotor, amount);

            match self.rotors.get_mut(rotor) {
                Some(r) => r.rotate(None),
                None => panic!("Failed to retrieve rotor at position '{}' from rotor list", rotor)
            }
        }
    }

    fn _set_rotor(&mut self, rotor: &String, letter: char) {
        debug!("Setting rotor {} to {}", rotor, letter);
        let mut face = self._get_rotor_at_position(rotor).get_face_letter();

        while face  != letter {
            self._move_rotor(rotor, 1);
            face = self._get_rotor_at_position(rotor).get_face_letter();
        }
    }

    pub fn rotor_conv(&self, rotor: &String, letter: char) -> char {
        self._get_rotor_at_position(rotor).convert(letter)
    }

    pub fn rotor_conv_inv(&self, rotor: &String, letter: char) -> char {
        self._get_rotor_at_position(rotor).convert_inv(letter)
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
        let zero_point_1 = rotor::alpha_index(self._get_rotor_at_position(rotor_1).get_face_letter());
        let zero_point_2 = rotor::alpha_index(self._get_rotor_at_position(rotor_2).get_face_letter());
        let interval = zero_point_2 as i32 - zero_point_1 as i32;

        let n;

        if zero_point_2 > zero_point_1 {
            let i: Vec<i32> = (0..26).collect();
            n = i[(terminal + interval as usize) % i.len()];
        }
        else {
            let i: Vec<i32> = (0..26).collect();
            let index: i32 = 26 + terminal as i32 + interval as i32;
            n = i[(index as usize) % i.len()];
        }
        println!("N: {}, {}", n, rotor::ALPHA[n as usize]);

        rotor::ALPHA[n as usize]
    }

    fn ringstellung_rotor_(&mut self, rotor: &String, amount: i32) {
        for _i in 0..amount {
            self._get_rotor_at_position_mut(rotor).rotate_inner_ring();
        }
    }

    pub fn ringstellung(&mut self, rsg_vec: Vec<i32>) {
        let rotor_labels = self.rotor_labels.clone();
        for i in 0..rsg_vec.len() {
            self.ringstellung_rotor_(&rotor_labels[i], rsg_vec[i]);
        }
    }

    pub fn type_letter(&mut self, letter: char) -> char {
        let upper_l = letter.to_ascii_uppercase();

        let mut cipher_af = self.plugboard.convert(upper_l);
        debug!("Plugboard: {} -> {}", upper_l, cipher_af);

        let rotor_labels = self.rotor_labels.clone();

        let (_, offset_1) = rotor_labels.split_at(1);
        let (offset_2, _) = rotor_labels.split_at(self.rotor_labels.len()-1);

        let mut notch_dict = HashMap::<String, Vec<char>>::new();
        let mut face_letters = HashMap::<String, char>::new();

        for rotor_label in self.rotor_labels.clone() {
            notch_dict.insert(rotor_label.clone(), self._get_rotor_at_position(&rotor_label).get_notches());
            face_letters.insert(rotor_label.clone(), self._get_rotor_at_position(&rotor_label).get_face_letter());
        }

        for rotors in offset_1.iter().rev().zip(offset_2.iter().rev()) {
            let (rotor_1, rotor_2) = rotors;
            let notches;

            match notch_dict.get(&rotor_1.clone()) {
                Some(n) => notches = n,
                None => panic!("Could not retrieve notch locations for rotor at position '{}'", rotor_1)
            }

            for notch in notches {
                if face_letters[&rotor_1.clone()] == *notch {
                    self._move_rotor(&rotor_2.clone(), 1);
                }
            }
        }

        self._move_rotor(&self.rotor_labels.clone()[&self.rotor_labels.len()-1], 1);

        let mut cipher = cipher_af;

        for rotor in self.rotor_labels.iter().rev() {
            cipher = cipher_af.clone();
            cipher_af = self.rotor_conv(rotor, cipher);
            debug!("Rotor {} conversion: {} -> {}", &rotor, cipher, cipher_af);
            cipher = cipher_af.clone();
            let adj_rotor_index = self.rotor_index(&rotor) as i32 -1;
            if adj_rotor_index < 0 {
                break;
            }

            let adjacent_rotor = &self.rotor_labels[adj_rotor_index as usize];

            cipher_af = self.inter_rotor_conv(&rotor, adjacent_rotor, cipher);
            debug!("Inter-Rotor {} to {} conversion: {} -> {}", &rotor, adjacent_rotor, cipher, cipher_af);
            cipher = cipher_af.clone();
        }

        cipher_af = self.reflector_conv(cipher);
        debug!("Reflector conversion: {} -> {}", cipher, cipher_af);
        cipher = cipher_af.clone();

        for rotor in self.rotor_labels.clone() {
            cipher_af = self.rotor_conv_inv(&rotor, cipher);
            debug!("Rotor {} conversion: {} -> {}", &rotor, cipher, cipher_af);
            cipher = cipher_af.clone();
            let adj_rotor_index = self.rotor_index(&rotor) as i32 + 1;
            if adj_rotor_index >= self.rotor_labels.len() as i32 {
                break;
            }
            let adjacent_rotor = &self.rotor_labels[adj_rotor_index as usize];
            cipher_af = self.inter_rotor_conv(&rotor, adjacent_rotor, cipher);
            debug!("Inter-Rotor {} to {} conversion: {} -> {}", &rotor, adjacent_rotor, cipher, cipher_af);
            cipher = cipher_af.clone();
        }

        cipher_af = self.plugboard_conv_inv(cipher);
        debug!("Plugboard: {} -> {}", cipher, cipher_af);
        debug!("--------------------");
        cipher_af
    }

    pub fn type_phrase(&mut self, phrase: String) -> String {
        let mut rng = rand::thread_rng();
        let mut temp = phrase.clone();
        temp.retain(|x| !x.is_whitespace());

        let remainder = if temp.len() % 5 != 0 {5 - temp.len() % 5} else {0};

        for _i in 0..remainder {
            temp.push(rotor::ALPHA[rng.gen_range(0..25) as usize]);
        }

        let mut out_str: String = "".to_string();

        for i in 0..temp.len() {
            let letter;

            match temp.chars().nth(i) {
                Some(l) => letter = l,
                None => panic!(
                    "Could not retrieve letter at position '{}' of modified phrase string '{}'", i, temp)
            }

            out_str += &self.type_letter(letter).to_string();

            if i + 1 % 5 == 0 {
                out_str += " ";
            }
        }
        out_str
    }

    pub fn set_key(&mut self, user_key: String) {
        let labels = self.rotor_labels.clone();
        if labels.len() != user_key.len() {
            panic!("Key length must match no. of rotors.");
        }

        let upper_k = user_key.to_ascii_uppercase();

        for (rotor_dict_key, letter) in self.rotor_labels.clone().iter().zip(upper_k.chars()) {
            let key = rotor_dict_key.clone();
            self._set_rotor(&key.clone(), letter.clone());
        }
    }

    pub fn rewire_plugboard(mut self, letter_1: char, letter_2: char) {
        self.plugboard.swap_letter_wiring(letter_1, letter_2);
    }
}

#[cfg(test)]
mod tests {
    #[test_log::test]
    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }
    #[test]
    fn test_type_letter() {
        let rotor_list = vec![1, 2, 3, 4];
        let enigma_type = "M4".to_string();
        let reflector = 'B';
        let letter = 'K';
        let before = letter.clone();
        let mut machine = super::Enigma::new(rotor_list.clone(), reflector, enigma_type.clone());
        assert!(machine.type_letter(letter) != before);
    }

    #[test]
    fn test_m3_encoding() {
        let rotor_list = vec![4, 3, 2];
        let enigma_type = "M3".to_string();
        let reflector = 'C';
        let key = "OUY".to_string();
        let message = "NOBODYEXPECTSTHESPANISHINQUISITION";
        let mut machine = super::Enigma::new(rotor_list.clone(), reflector, enigma_type.clone());

        machine.set_key(key.clone());

        let result = machine.type_phrase(message.to_string());

        machine = super::Enigma::new(rotor_list.clone(), reflector, enigma_type.clone());
        machine.set_key(key.clone());
        let out = machine.type_phrase(result);

        assert!(&out[..out.len()-1] == message);
    }

    #[test]
    fn test_m4_encoding() {
        let rotor_list = vec![4, 3, 2, 1];
        let enigma_type = "M4".to_string();
        let reflector = 'B';
        let key = "MOAN".to_string();
        let message = "SOTHATSCAPRICORNISIT";
        let mut machine = super::Enigma::new(rotor_list.clone(), reflector, enigma_type.clone());

        machine.set_key(key.clone());

        let result = machine.type_phrase(message.to_string());

        machine = super::Enigma::new(rotor_list.clone(), reflector, enigma_type.clone());
        machine.set_key(key.clone());
        let out = machine.type_phrase(result);

        assert!(out == message);
    }

    #[test]
    fn test_ringstellung() {
        let rotor_list = vec![1, 2, 3, 4];
        let enigma_type = "M4".to_string();
        let reflector = 'B';
        let key = "TEST".to_string();
        let message = "This is a test".to_string().replace(" ", "").to_ascii_uppercase();
        let mut machine = super::Enigma::new(rotor_list.clone(), reflector, enigma_type.clone());

        machine.set_key(key.clone());
        machine.ringstellung(rotor_list.clone());
        let result = machine.type_phrase(message.clone()).replace(" ", "");
        machine = super::Enigma::new(rotor_list.clone(), reflector, enigma_type.clone());
        machine.set_key(key.clone());
        machine.ringstellung(rotor_list.clone());
        let orig = machine.type_phrase(result.clone()).replace(" ", "");

        assert!(&orig[..orig.len()-4] == message);
    }
}