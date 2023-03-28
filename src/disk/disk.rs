pub mod disk {
    use crate::disk::hardware_manager::hardware_manager::DiskState;

    pub struct DiskHead {
        current_track: u32,
        current_angle: u32,
        state: DiskState,
    }

    pub struct DiskMetadata {
        forward_speed: u32,
        base_spin_speed: u32,
    }

    pub struct Disk {
        head: DiskHead,
        metadata: DiskMetadata,
    }

    impl DiskHead {
        pub fn default() -> DiskHead {
            DiskHead {
                current_track: 0,
                current_angle: 0,
                state: DiskState::STOP,
            }
        }

        pub fn show_head(&self) {
            let state = match &self.state {
                DiskState::STOP => "STOP".to_owned(),
                DiskState::READ(angle) => format!("READ({})", angle),
                DiskState::MOVE(m) => {
                    format!("MOVE({}, {})", m.destination, match m.direction{
                    crate::disk::hardware_manager::hardware_manager::MoveDirection::FORWARD => "FORWARD".to_owned(),
                    crate::disk::hardware_manager::hardware_manager::MoveDirection::BACKWARD => "BACWARD".to_owned(),
                })
                }
            };

            println!(
                "DiskHead<current track: {}, current angle: {}, state: {}>",
                self.current_track, self.current_angle, state
            );
        }
    }

    impl DiskMetadata {
        pub fn new(forward_speed: u32, base_spin_speed: u32) -> DiskMetadata {
            let forward_speed = ((1.0 / forward_speed as f32) * 1000.0 * 1000.0) as u32;
            let base_spin_speed =
                ((1.0 / base_spin_speed as f32) * 60.0 * 1000.0 * 1000.0 * 1000.0) as u32;

            DiskMetadata {
                forward_speed: forward_speed,
                base_spin_speed: base_spin_speed,
            }
        }
    }

    impl Disk {
        pub fn new(metadata: DiskMetadata) -> Disk {
            Disk {
                head: DiskHead::default(),
                metadata: metadata,
            }
        }

        fn get_state(&self) -> String {
            match &self.head.state {
                DiskState::STOP => "STOP".to_owned(),
                DiskState::READ(angle) => format!("READ({})", angle),
                DiskState::MOVE(m) => {
                    format!("MOVE({}, {})", m.destination, match m.direction{
                    crate::disk::hardware_manager::hardware_manager::MoveDirection::FORWARD => "FORWARD".to_owned(),
                    crate::disk::hardware_manager::hardware_manager::MoveDirection::BACKWARD => "BACWARD".to_owned(),
                })
                }
            }
        }

        pub fn show(&self) {
            println!(
                "Disk <\n\tHead <current track: {}, current angle: {}, state: {}>\n\tMetadata <forward speed: {}, base spin speed: {}>\n>",
                self.head.current_track,
                self.head.current_angle,
                self.get_state(),
                self.metadata.forward_speed,
                self.metadata.base_spin_speed
            );
        }
    }
}
