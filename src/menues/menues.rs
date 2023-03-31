pub mod menues {
    use std::{
        io::{stdin, stdout, Read, Write},
        num::ParseIntError,
    };

    use rand::Rng;

    use crate::disk::{
        disk::disk::{Disk, DiskMetadata},
        driver::driver::{Driver, SimpleDriver, Task},
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

    fn flush() {
        stdout()
            .flush()
            .expect("There was an error while writing to the standard output");
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

        flush();
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

    fn generate_random_request(task_id: u32) -> Task {
        let track = rand::thread_rng().gen_range(1..=10000) as u32;
        let angle = rand::thread_rng().gen_range(0..=359) as u32;

        Task::new(task_id, track, angle)
    }

    fn run_naive_approach_simulation(requests: u32) {
        // clear();
        println!("Here is the disk:");
        let disk = build_disk();
        disk.show();

        let mut driver = SimpleDriver::new(disk);

        let mut remaining_tasks = 0;

        let mut added_tasks = 0;
        let threshould = requests as f32 / 10000000.0;
        let mut tasks: Vec<Task> = Vec::new();
        let mut time = 0;

        for i in 1..=requests {
            let task = generate_random_request(i);
            tasks.push(task);
        }

        while added_tasks != requests || remaining_tasks != 0 {
            let prob: f32 = rand::thread_rng().gen();

            if prob < threshould && added_tasks != requests {
                let task = &tasks[added_tasks as usize];
                driver.add_new_task(task);

                added_tasks += 1;
                remaining_tasks += 1;
                println!(
                    "New task with task_id: {} added, time: {}",
                    added_tasks, time
                );
            }

            time += 1;
            let result = driver.step();

            if result != 0 {
                remaining_tasks -= 1;
                println!(
                    "\t\t\t\t\ttask with task_id: {} is done, time: {}",
                    result, time
                );
            }
        }
    }

    fn read_number_of_steps() -> u32 {
        let mut user_input = read_user_input();
        loop {
            match user_input {
                Ok(value) => return value,
                Err(_) => {
                    print_error_message();
                    user_input = read_user_input();
                }
            }
        }
    }

    fn build_disk() -> Disk {
        let metadata = DiskMetadata::new(1000000, 600000000);
        let disk = Disk::new(metadata);
        disk
    }

    fn naivea_approach() {
        clear();
        println!("Enter the number of requests you want to simulate:");
        flush();

        let steps = read_number_of_steps();
        run_naive_approach_simulation(steps);
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
                    // run_naive_approach_simulation();
                    naivea_approach();
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
