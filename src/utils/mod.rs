use super::generate_membership_id;
use super::std::{
    io::{self, Write},
    thread,
    time::Duration,
};

pub fn print_dots(delay: i32, text: String) {
    let spinner = ['|', '/', '-', '\\']; // Characters for the spinner

    for _ in 0..delay {
        for &s in &spinner {
            print!("{}..{}\r", text, s); // Print the spinner character at the beginning of the line
            io::stdout().flush().unwrap(); // Ensure it prints immediately
            thread::sleep(Duration::from_millis(50)); // Sleep to control speed of rotation
        }
        print!("{}...\r", text);
    }
    println!();
}

pub fn prompt_user(vector: &mut Vec<String>) {
    let mut country_code = String::new();
    let mut state_code = String::new();

    println!("Welcome to the Membership ID Generator! We have over {} IDs generated! Enter your country code to generate yours", vector.len());

    let parsed_country_code: u32 = loop {
        io::stdin()
            .read_line(&mut country_code)
            .expect("Failed to read line");

        if country_code[..1] != "+".to_string() {
            println!("Country code must always begin with +!",);
            country_code.clear();
            continue;
        } else {
            country_code = country_code.replace("+", "")
        }

        if country_code.trim().len() > 3 {
            println!("Aw snap! Country code cannot exceed 3 digits! Kindly enter a valid 3 digit country code");
            country_code.clear();
            continue;
        }

        match country_code.trim().parse() {
            Ok(num) => break num,
            Err(_) => {
                println!("Please enter a valid country code!");
                state_code.clear();
                continue;
            }
        };
    };

    println!("Great!, Kindly input your state's code to continue");

    let parsed_state_code: u32 = loop {
        io::stdin()
            .read_line(&mut state_code)
            .expect("Failed to read line");

        if state_code.trim().len() > 2 {
            println!("State code must not exceed 2 digits!");
            state_code.clear();
            continue;
        }

        match state_code.trim().parse() {
            Ok(num) => break num,
            Err(_) => {
                println!(
                    "Please enter a valid state code. Valid state codes contain numbers only!"
                );
                state_code.clear();
                continue;
            }
        };
    };

    let newly_generated_id =
        generate_membership_id(&parsed_country_code, &parsed_state_code, &vector);

    vector.push(newly_generated_id);

    generate_new_id(prompt_user, vector);
}

fn generate_new_id(callback: fn(&mut Vec<String>), vector: &mut Vec<String>) {
    let mut res: String = String::new();
    println!(
        "Thanks for using our Membership ID generator 😁🙌! Would you like to generate another Membership ID? (Y/N)"
    );

    io::stdin()
        .read_line(&mut res)
        .expect("Failed to read line");

    match res.trim().to_uppercase().as_str() {
        "Y" => callback(vector),
        "N" => {
            println!(
        "Thanks for using our Membership ID generator! We can't wait to have you come back 😁😊"
    );
        }
        _ => {
            println!("Uhh 🤔, that didn't seem right..");
            res.clear();
            generate_new_id(callback, vector)
        }
    }
}
