use disk::{
    disk::disk::{Disk, DiskMetadata},
    driver::driver::{Driver, ElevetorDriver, Task},
};

pub mod disk;

fn main() {
    let metadata = DiskMetadata::new(1000000, 3000000000);
    let disk = Disk::new(metadata);
    disk.show();

    let mut driver = ElevetorDriver::new(disk);
    let tasks = vec![
        Task::new(1, 1, 0),
        Task::new(2, 100, 50),
        Task::new(3, 200, 180),
        Task::new(4, 100, 50),
        Task::new(5, 150, 50),
        Task::new(6, 1, 20),
        Task::new(7, 1, 0),
        Task::new(8, 1, 0),
    ];

    for task in &tasks {
        driver.add_new_task(task);
    }

    for i in 0..100000 {
        let result = driver.step();

        if result != 0 {
            println!("task with task_id {result} has done in time {i}");
        }
    }

    let tasks = vec![
        Task::new(9, 1, 0),
        Task::new(10, 100, 50),
        Task::new(11, 100, 50),
        Task::new(12, 150, 50),
        Task::new(13, 1, 20),
        Task::new(14, 1, 0),
        Task::new(15, 1, 0),
    ];

    for task in &tasks {
        driver.add_new_task(task);
    }

    for i in 0..100000 {
        let result = driver.step();

        if result != 0 {
            println!("task with task_id {result} has done in time {i}");
        }
    }
}
