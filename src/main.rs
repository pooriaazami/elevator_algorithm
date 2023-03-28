use disk::disk::disk::{Disk, DiskMetadata};

pub mod disk;

fn main() {
    let metadata = DiskMetadata::new(4000, 5000);
    let disk =  Disk::new(metadata);

    disk.show();
}
