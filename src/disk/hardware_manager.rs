pub mod hardware_manager {
    #[derive(Eq, PartialEq, Copy, Clone)]
    pub enum MoveDirection {
        FORWARD,
        BACKWARD,
    }

    #[derive(Eq, PartialEq, Copy, Clone)]
    pub struct MoveState {
        pub destination: u32,
        pub direction: MoveDirection,
    }

    #[derive(Eq, PartialEq, Copy, Clone)]
    pub enum DiskState {
        STOP,
        READ(u32),
        MOVE(MoveState),
    }

    impl MoveState {
        pub fn new(destination: u32, direction: MoveDirection) -> MoveState {
            MoveState {
                destination: destination,
                direction: direction,
            }
        }
    }

}
