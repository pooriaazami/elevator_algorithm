pub mod hardware_manager {
    pub enum MoveDirection {
        FORWARD,
        BACKWARD,
    }

    pub struct MoveState {
        distination: u32,
        direction: MoveDirection,
    }

    pub enum DiskState {
        STOP,
        READ,
        MOVE(MoveState),
    }
}
