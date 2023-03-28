use disk::disk::disk::{Disk, DiskMetadata};

pub mod disk;

fn main() {
    let metadata = DiskMetadata::new(1000000, 3000000000);
    let mut disk = Disk::new(metadata);

    disk.show();

    disk.add_move_task(100);

    disk.show();

    let time = 1 * 1000 * 1000;

    for _ in 0..time {
        disk.step();
    }
    disk.show();

    disk.add_move_task(50);

    disk.show();

    for _ in 0..time {
        disk.step();
    }
    disk.show();

    disk.add_reading_task(150);
    disk.show();

    for _ in 0..100000000 {
        disk.step();
        // disk.show();
    }

    disk.show();

}
