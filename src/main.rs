use disk::{
    disk::disk::{Disk, DiskMetadata},
    driver::driver::{Driver, SimpleDriver, Task},
};

pub mod disk;

fn main() {
    let metadata = DiskMetadata::new(1000000, 3000000000);
    let disk = Disk::new(metadata);
    disk.show();

    let mut driver = SimpleDriver::new(disk);
    let tasks = vec![
        Task::new(1, 0, 0),
        Task::new(2, 100, 50),
        Task::new(3, 100, 180),
        Task::new(4, 200, 50),
        Task::new(5, 100, 50),
        Task::new(6, 0, 0),
        Task::new(7, 0, 0),
        Task::new(8, 0, 0),
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
