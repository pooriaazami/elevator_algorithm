pub mod menues {
    use std::{
        io::{stdin, stdout, Read, Write},
        num::ParseIntError,
    };

    #[derive(Eq, PartialEq)]
    enum MainMenuOptions {
        NAIVE,
        ELEVATOR,
        INFO,
        EXIT,

        INVALID,
    }

    fn clear() {
        clearscreen::clear().expect("There was an error while clearing the screen");
    }

    fn print_main_menu(header: bool, clear_screen: bool) {
        if clear_screen {
            clear();
        }

        if header {
            println!("Welcome to the Disk Simulation app.\nPlease enter your command:");
        }

        println!("1- Simulate Naive Approach\n2- Simulate elevator algorithm\n3- Info\n4- Exit");
        print!(">> ");

        stdout()
            .flush()
            .expect("There was an error while writing to the standard output");
    }

    fn read_user_input() -> Result<u32, ParseIntError> {
        let mut user_input = String::new();

        std::io::stdin()
            .read_line(&mut user_input)
            .expect("Invalid input");

        user_input.trim().parse::<u32>()
    }

    fn read_main_manu_option() -> MainMenuOptions {
        let user_input = read_user_input();

        match user_input {
            Ok(1) => MainMenuOptions::NAIVE,
            Ok(2) => MainMenuOptions::ELEVATOR,
            Ok(3) => MainMenuOptions::INFO,
            Ok(4) => MainMenuOptions::EXIT,
            _ => MainMenuOptions::INVALID,
        }
    }

    fn print_error_message() {
        println!("Invalid input!\nPlease check your input and try again.")
    }

    fn pause() {
        let _ = stdin().read(&mut [0u8]).unwrap();
        let _ = stdin().read(&mut [0u8]).unwrap();
    }

    fn print_info() {
        clear();
        println!("pooriaazami@gmail.com\nimohsen2002@gmail.com");
        pause();
    }

    pub fn main_menu() {
        // print_main_menu(true);
        let mut user_input = MainMenuOptions::INVALID;
        let mut details = true;

        while user_input != MainMenuOptions::EXIT {
            print_main_menu(details, details);
            user_input = read_main_manu_option();

            match user_input {
                MainMenuOptions::NAIVE => {
                    details = true;
                }
                MainMenuOptions::ELEVATOR => {
                    details = true;
                }
                MainMenuOptions::INFO => {
                    print_info();
                    details = true;
                }
                MainMenuOptions::INVALID => {
                    print_error_message();
                    details = false;
                }
                MainMenuOptions::EXIT => {}
            }
        }
    }
}
