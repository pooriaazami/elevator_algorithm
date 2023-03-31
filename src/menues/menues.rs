pub mod menues {
    use std::{
        collections::HashMap,
        io::{stdin, stdout, Read, Write},
        num::ParseIntError,
    };

    use rand::Rng;

    use crate::disk::{
        disk::disk::{Disk, DiskMetadata},
        driver::driver::{Driver, ElevetorDriver, SimpleDriver, Task},
    };

    #[derive(Eq, PartialEq)]
    enum MainMenuOptions {
        NAIVE,
        ELEVATOR,
        INFO,
        EXIT,

        INVALID,
    }

    enum Algorithms {
        NAIVE,
        ELEVATOR,
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

    fn run_simulation(algorithm: Algorithms, requests: u32) -> Vec<u32> {
        // clear();
        println!("Here is the disk:");
        let disk = build_disk();
        disk.show();

        let mut tasks: Vec<Task> = Vec::new();
        let mut insertion_times: HashMap<u32, u32> = HashMap::new();
        let mut response_times: Vec<u32> = Vec::new();

        let mut driver: Box<dyn Driver> = match algorithm {
            Algorithms::NAIVE => Box::new(SimpleDriver::new(disk)),
            Algorithms::ELEVATOR => Box::new(ElevetorDriver::new(disk)),
        };

        let mut remaining_tasks = 0;

        let mut added_tasks = 0;
        let threshould = requests as f32 / 10000000.0;
        let mut time = 0;

        for i in 1..=requests {
            let task = generate_random_request(i);
            tasks.push(task);
        }

        while added_tasks != requests || remaining_tasks != 0 {
            let prob: f32 = rand::thread_rng().gen();

            if prob < threshould && added_tasks != requests {
                let task = &tasks[added_tasks as usize];
                // task.show_task();
                driver.add_new_task(task);
                insertion_times.insert(*task.get_id(), time);

                added_tasks += 1;
                remaining_tasks += 1;
            }

            time += 1;
            let result = driver.step();

            if result != 0 {
                remaining_tasks -= 1;
                let response_length = time - insertion_times[&result];
                insertion_times.remove(&result);

                response_times.push(response_length);

                if (added_tasks - remaining_tasks) % 10 == 0{
                    println!("{}/{} more responses are done.", added_tasks - remaining_tasks, requests);
                }
            }
        }

        response_times
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

    fn show_stats(times: Vec<u32>) {
        let times = times.iter().map(|x| *x as f32);
        let length = times.len() as f32;
        let total: f32 = times.sum();
        let mean = total / length;

        println!("mean response time: {}", mean);
    }

    fn naivea_algorithm() {
        clear();
        println!("Enter the number of requests you want to simulate:");
        flush();

        let steps = read_number_of_steps();
        let response_times = run_simulation(Algorithms::NAIVE, steps);
        show_stats(response_times);
        pause();
    }

    fn elevator_algorithm() {
        clear();
        println!("Enter the number of requests you want to simulate:");
        flush();

        let steps = read_number_of_steps();
        let response_times = run_simulation(Algorithms::ELEVATOR, steps);
        show_stats(response_times);
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
                    naivea_algorithm();
                    details = true;
                }
                MainMenuOptions::ELEVATOR => {
                    elevator_algorithm();
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
