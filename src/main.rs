mod enigma;

fn is_word(s: &String) -> bool {
    for i in 0..s.len() {
        if s.chars().nth(i).unwrap().is_alphanumeric() {
            return false;
        }
    }
    return true;
}

fn main() {
    let mut rotors = vec![2, 3, 4];
    let mut key = "YES".to_string();
    let mut ask_count = 0;
    let mut choice = "".to_string();

    while choice != "Y" && choice != "N" {
        choice = "".to_string();
        println!("Set key? [y/n]");
        std::io::stdin().read_line(&mut choice).unwrap();
        choice = choice.trim_end().to_ascii_uppercase();
        ask_count += 1;

        if ask_count > 10 {
            println!("Program Timeout.");
            return;
        }
    }

    ask_count = 0;

    if choice == "Y" {
        println!("Enter 3/4 character key for M3 or M4 machine: ");
        std::io::stdin().read_line(&mut key).unwrap();

        let key_len = key.len();

        while (key_len != 3 && key_len != 4) || !is_word(&key) {
            println!("ey must be of size 3 or 4 and alpha values only: ");
            std::io::stdin().read_line(&mut key).unwrap();
        }
    }

    choice = "".to_string();

    while choice != "Y" && choice != "N" {
        choice = "".to_string();
        println!("Set Rotors? [y/n]");
        std::io::stdin().read_line(&mut choice).unwrap();
        choice = choice.trim_end().to_ascii_uppercase();
        ask_count += 1;

        if ask_count > 10 {
            println!("Program Timeout.");
            return;
        }
    }

    ask_count = 0;

    if choice == "Y" {
        let mut _temp = -1;
        rotors = vec![];
        println!("Enter {} rotor types invidually: ", key.len().to_string());

        for i in 0..key.len() {
            println!("Rotor {}: ", (i+1).to_string());
            std::io::stdin().read_line(&mut choice).unwrap();
            _temp = choice.trim().parse().expect("Input must be integer");
            while _temp < 1 || _temp > 8 || rotors.iter().any(|&x| x == _temp) {
                println!("Invalid Rotor Choice! Rotor type must be 1-8 and be unique: ");
                std::io::stdin().read_line(&mut choice).unwrap();
                _temp = choice.trim().parse().expect("Input must be integer");
            }
            rotors.push(_temp);
        }
    }

    let mut rsg_settings = vec![];

    choice = "".to_string();

    while choice != "Y" && choice != "N" {
        choice = "".to_string();
        println!("Ringstellung? [y/n] ");
        std::io::stdin().read_line(&mut choice).unwrap();
        choice = choice.trim_end().to_ascii_uppercase();
        ask_count += 1;

        if ask_count > 10 {
            println!("Program Timeout.");
            return;
        }
    }

    ask_count = 0;

    if choice == "Y" {
        let mut _temp = -1;
        println!("Set Number of Internal Wiring Rotation Increments for Each of the {} Rotors: ", rotors.len().to_string());

        for i in 0..rotors.len() {
            while _temp < 0 && !choice.chars().nth(0).unwrap().is_digit(10) {
                println!("Rotation increments must be a positive integer: ");
                std::io::stdin().read_line(&mut choice).unwrap();
                _temp = choice.trim().parse().expect("Input must be integer");
                ask_count += 1;
                if ask_count > 10 {
                    println!("Program Timeout.");
                    return;
                }
            }
            ask_count = 0;
            rsg_settings.push(_temp);
        }
    }
    else {
        for i in 0..rotors.len() {
            rsg_settings.push(0);
        }
    }

    choice = "".to_string();

    while choice != "quit" {
        println!("INPUT: ");
        choice = "".to_string();
        std::io::stdin().read_line(&mut choice).unwrap();
        if choice != "quit" {
            let mut enigma = enigma::Enigma::new(rotors.clone(), 'B', if key.clone().len() == 3 {"M3".to_string()} else {"M4".to_string()});
            enigma = enigma.set_key(key.clone());
            enigma = enigma.ringstellung(rsg_settings.clone());
            let output = enigma.type_phrase(choice);
            println!("OUTPUT: {}", output);
            choice = "".to_string();
        }
    }
}
