pub mod hardware_manager {
    pub enum MoveDirection {
        FORWARD,
        BACKWARD,
    }

    pub struct MoveState {
        pub destination: u32,
        pub direction: MoveDirection,
    }

    pub enum DiskState {
        STOP,
        READ(u32),
        MOVE(MoveState),
    }


}
