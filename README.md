# Membership ID Generator (Rust)

Welcome to my first Rust project! 🎉 This is a console-based Membership ID Generator written in Rust.

## Overview

This program generates unique membership IDs based on user inputs such as country code and state code. The application prompts the user to input their country and state codes, generates a unique ID, and handles multiple retries in case of invalid inputs. It also includes a typewriter-like spinning animation to simulate the "processing" stage while generating IDs.

## Features

- Generates unique membership IDs.
- Validates input for country code and state code.
- Simulates a typewriter-like spinner animation during ID generation.
- Allows the user to generate a new ID or exit the program.

## Requirements

- [Rust](https://www.rust-lang.org/) installed on your machine. You can install Rust by following the instructions [here](https://www.rust-lang.org/tools/install).

## How to Run

1. Clone the repository:
   ```bash
   git clone https://github.com/yourusername/membership-id-generator.git
   ```
2. Navigate to the project folder:
   ```
   cd membership-id-generator
   ```
3. Build and run the project using Cargo:
   ```
   cargo run
   ```
4. Follow the prompts to input your country and state codes and generate a membership ID.

## Code Explanation

- Country Code Validation: The program ensures that the country code starts with a "+" and has a maximum of 3 digits.
- State Code Validation: The program checks that the state code is no longer than 2 digits.
- Spinner Animation: A simple rotating cursor is shown while the ID is being generated.
- ID Generation: The ID is formatted as DL{country_code}{state_code}{unique_number}.

# Example

Welcome to the Membership ID Generator! We have over 4 IDs generated! Enter your country code to generate yours <br/>
`+1` <br/>
Great!, Kindly input your state's code to continue <br/>
`41` <br/>
Please kindly wait while we generate a unique Membership ID for you 🥳! <br />
Generating.. <br />
Yohoo! You're the first of your kind, welcome 🥳🥳🥳! <br/>
Kindly give us some time while we carefully craft your membership ID! <br />
Almost done... <br/>
Congrats! Your newly generated Membership ID is `DL00101001` <br/>

# Contributing

Feel free to fork this project, create issues, and submit pull requests! Contributions are welcome to improve the project.
