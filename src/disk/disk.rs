pub mod disk {
    use crate::disk::hardware_manager::hardware_manager::DiskState;

    pub struct DiskHead {
        current_track: u32,
        current_angle: u32,
        state: DiskState,
    }

    impl DiskHead {
        pub fn default() -> DiskHead {
            DiskHead {
                current_track: 0,
                current_angle: 0,
                state: DiskState::STOP,
            }
        }
    }
}
