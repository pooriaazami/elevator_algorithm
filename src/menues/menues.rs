pub mod menues {
    use std::{
        collections::HashMap,
        fs::File,
        io::{stdin, stdout, Read, Write},
        num::ParseIntError,
        time::SystemTime,
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

    #[derive(Copy, Clone)]
    enum Algorithms {
        NAIVE,
        ELEVATOR,
    }

    struct LogHeader {
        metadata: DiskMetadata,
        max_tracks: u32,
        steps: u32,
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

    fn open_log_file() -> File {
        let log_file_path = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_millis()
            .to_string();
        let log_file_path = format!("{}.txt", log_file_path);

        let data_file =
            File::create(log_file_path).expect("There was a problem creating the log file");

        data_file
    }

    fn log_data_to_file(
        data_file: &mut File,
        algortihm: Algorithms,
        log_header: LogHeader,
        times: Vec<u32>,
    ) {
        let algorithm = match algortihm {
            Algorithms::NAIVE => "Naive",
            Algorithms::ELEVATOR => "Elevator",
        };

        let header = format!(
            "algorithm: {}, forward_speed: {}, spin_speed: {}, max_track: {}, steps: {}\n",
            algorithm,
            log_header.metadata.get_forward_speed(),
            log_header.metadata.get_spin_speed(),
            log_header.max_tracks,
            log_header.steps,
        );

        data_file
            .write(header.as_bytes())
            .expect("There was an error while write header to the log file");

        for time in times.iter() {
            data_file
                .write(time.to_string().as_bytes())
                .expect("There was an error while write data to the log file");
            data_file
                .write(",".as_bytes())
                .expect("There was an error while write data to the log file");
        }
        data_file
            .write("\n".as_bytes())
            .expect("There was an error while write data to the log file");
    }

    fn simulation_menu(algorithm: Algorithms) {
        clear();
        let (metadata, max_track) = read_hard_metadata();
        println!("Enter the number of requests you want to simulate:");

        let steps = safe_read_int_value();
        let log_header = LogHeader {
            metadata: metadata.clone(),
            max_tracks: max_track,
            steps: steps,
        };
        let response_times = run_simulation(algorithm, metadata, max_track, steps);

        let mut log_file = open_log_file();
        // show_stats(response_times);
        log_data_to_file(&mut log_file, algorithm, log_header, response_times);
        pause();
    }

    fn generate_experience(
        log_file: &mut File,
        forward_speed: u32,
        spin_speed: u32,
        steps: u32,
        max_tracks: u32,
        algorithm: Algorithms,
    ) {
        let metadata = DiskMetadata::from_config(forward_speed, spin_speed);
        let log_header = LogHeader {
            metadata: metadata.clone(),
            max_tracks: max_tracks,
            steps: steps,
        };

        let response_times = run_simulation(algorithm, metadata, max_tracks, steps);
        log_data_to_file(log_file, algorithm, log_header, response_times);
    }

    fn log_all_configs() {
        let mut log_file = open_log_file();

        for algorithm in [Algorithms::NAIVE, Algorithms::ELEVATOR] {
            for forward_speed in [1, 5, 10, 15, 20, 25] {
                for spin_speed in [25, 50, 100, 250, 500] {
                    for max_tracks in [1000, 5000, 10000, 50000] {
                        for steps in [100, 500, 1000, 5000] {
                            for _ in 0..10 {
                                generate_experience(
                                    &mut log_file,
                                    forward_speed,
                                    spin_speed,
                                    steps,
                                    max_tracks,
                                    algorithm,
                                );
                            }
                        }
                    }
                }
            }
        }
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
                MainMenuOptions::LOG => {
                    log_all_configs();
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
