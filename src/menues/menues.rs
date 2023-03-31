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
        LOG,
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

        println!(
            "1- Simulate Naive Approach\n2- Simulate Elevator Algorithm\n3- Log\n4- Info\n5- Exit"
        );
        print!(">> ");

        flush();
    }

    fn read_raw_input() -> String {
        let mut user_input = String::new();
        std::io::stdin()
            .read_line(&mut user_input)
            .expect("Invalid input");

        user_input
    }

    fn read_user_input() -> Result<u32, ParseIntError> {
        let user_input = read_raw_input();
        user_input.trim().parse::<u32>()
    }

    fn safe_read_int_value() -> u32 {
        loop {
            let value = read_user_input();
            match value {
                Ok(v) => return v,
                Err(_) => print_error_message(),
            }
        }
    }

    fn read_main_manu_option() -> MainMenuOptions {
        let user_input = read_user_input();

        match user_input {
            Ok(1) => MainMenuOptions::NAIVE,
            Ok(2) => MainMenuOptions::ELEVATOR,
            Ok(3) => MainMenuOptions::LOG,
            Ok(4) => MainMenuOptions::INFO,
            Ok(5) => MainMenuOptions::EXIT,
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

    fn generate_random_request(task_id: u32, max_track: u32) -> Task {
        let track = rand::thread_rng().gen_range(1..=max_track) as u32;
        let angle = rand::thread_rng().gen_range(0..=359) as u32;

        Task::new(task_id, track, angle)
    }

    fn run_simulation(
        algorithm: Algorithms,
        metadata: DiskMetadata,
        max_track: u32,
        requests: u32,
    ) -> Vec<u32> {
        println!("Here is the disk:");
        let disk = build_disk(metadata);
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
            let task = generate_random_request(i, max_track);
            tasks.push(task);
        }

        while added_tasks != requests || remaining_tasks != 0 {
            let prob: f32 = rand::thread_rng().gen();

            if prob < threshould && added_tasks != requests {
                let task = &tasks[added_tasks as usize];

                driver.add_new_task(task);
                insertion_times.insert(*task.get_id(), time);

                added_tasks += 1;
                remaining_tasks += 1;
            }

            time += 1;
            let result = driver.step();
            // println!("{}", result);
            if result != 0 {
                remaining_tasks -= 1;
                let response_length = time - insertion_times[&result];
                insertion_times.remove(&result);

                response_times.push(response_length);

                if (added_tasks - remaining_tasks) % 10 == 0 {
                    println!(
                        "{}/{} more responses are done.",
                        added_tasks - remaining_tasks,
                        requests
                    );
                }
            }
        }

        response_times
    }

    fn read_hard_metadata() -> (DiskMetadata, u32) {
        println!("Do you want to config the hard drive?(Y/N)");
        let mut user_input = read_raw_input().trim().to_lowercase();

        loop {
            if user_input == "y" {
                println!("Enter forward-backward speed:");
                let fd_speed = safe_read_int_value();

                println!("Enter spin speed:");
                let spin_speed = safe_read_int_value();

                println!("Enter number of the tracks:");
                let max_tracks = safe_read_int_value();

                return (DiskMetadata::from_config(fd_speed, spin_speed), max_tracks);
            } else if user_input == "n" {
                return (DiskMetadata::default(), 10000);
            }
            print_error_message();
            user_input = read_raw_input().trim().to_lowercase();
        }
    }

    fn build_disk(metadata: DiskMetadata) -> Disk {
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

    fn simulation_menu(algorithm: Algorithms) {
        clear();
        let (metadata, max_track) = read_hard_metadata();
        println!("Enter the number of requests you want to simulate:");
        // flush();

        let steps = safe_read_int_value();
        let response_times = run_simulation(algorithm, metadata, max_track, steps);

        show_stats(response_times);
        pause();
    }

    pub fn main_menu() {
        let mut user_input = MainMenuOptions::INVALID;
        let mut details = true;

        while user_input != MainMenuOptions::EXIT {
            print_main_menu(details, details);
            user_input = read_main_manu_option();

            match user_input {
                MainMenuOptions::NAIVE => {
                    simulation_menu(Algorithms::NAIVE);
                    details = true;
                }
                MainMenuOptions::ELEVATOR => {
                    simulation_menu(Algorithms::ELEVATOR);
                    details = true;
                }
                MainMenuOptions::LOG => {}
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
