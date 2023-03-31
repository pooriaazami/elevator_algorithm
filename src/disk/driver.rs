pub mod driver {
    use std::collections::HashMap;

    use crate::disk::{
        disk::disk::Disk,
        hardware_manager::hardware_manager::{DiskState, MoveDirection},
    };

    pub struct Task {
        task_id: u32,
        track: u32,
        angle: u32,
    }

    pub enum CacheState<'a> {
        EMPTY,
        ACTIVE(&'a Task),
    }

    pub trait Driver<'a> {
        fn add_new_task(&mut self, task: &'a Task);

        fn step(&mut self) -> u32;
    }

    pub struct SimpleDriver<'a> {
        disk: Disk,
        cache: CacheState<'a>,
        task_list: Vec<&'a Task>,
    }

    pub struct ElevetorDriver<'a> {
        disk: Disk,
        cache: CacheState<'a>,
        same_direction_list: HashMap<u32, Vec<&'a Task>>,
        opposite_direction_list: HashMap<u32, Vec<&'a Task>>,
    }

    impl Task {
        pub fn new(task_id: u32, track: u32, angle: u32) -> Task {
            Task {
                task_id: task_id,
                track: track,
                angle: angle,
            }
        }

        pub fn get_track(&self) -> &u32 {
            &self.track
        }
    }

    impl<'a> SimpleDriver<'a> {
        pub fn new(disk: Disk) -> SimpleDriver<'a> {
            SimpleDriver {
                disk: disk,
                cache: CacheState::EMPTY,
                task_list: Vec::new(),
            }
        }
    }

    impl<'a> ElevetorDriver<'a> {
        pub fn new(disk: Disk) -> ElevetorDriver<'a> {
            ElevetorDriver {
                disk: disk,
                cache: CacheState::EMPTY,
                same_direction_list: HashMap::new(),
                opposite_direction_list: HashMap::new(),
            }
        }

        fn add_to_same_direction_list(&mut self, task: &'a Task) {
            let vector = self.same_direction_list.get_mut(&task.track);
            match vector {
                Some(v) => {
                    v.push(task);
                }
                None => {
                    self.same_direction_list.insert(task.track, vec![task]);
                }
            }
        }

        fn add_to_opposite_direction_list(&mut self, task: &'a Task) {
            let vector = self.opposite_direction_list.get_mut(&task.track);
            match vector {
                Some(v) => {
                    v.push(task);
                }
                None => {
                    self.opposite_direction_list.insert(task.track, vec![task]);
                }
            }
        }

        fn fetch_same_direction_task(&mut self) -> Option<&'a Task> {
            let keys: Vec<_> = { self.same_direction_list.keys().take(1).cloned().collect() };
            for key in keys {
                let mut tasks = self.same_direction_list.remove(&key).unwrap();
                let task = tasks.pop().unwrap();

                if tasks.len() != 0 {
                    self.same_direction_list.insert(key, tasks);
                }

                // println!("returning task with id: {}", task.task_id);
                return Some(task);
            }

            None
        }

        fn fetch_a_task_for_current_track(&mut self) -> &'a Task {
            let mut tasks = self
                .same_direction_list
                .remove(&self.disk.get_current_track())
                .unwrap();
            let task = tasks.pop().unwrap();

            if tasks.len() != 0 {
                self.same_direction_list
                    .insert(self.disk.get_current_track(), tasks);
            }

            task
        }
    }

    impl<'a> Driver<'a> for SimpleDriver<'a> {
        fn add_new_task(&mut self, task: &'a Task) {
            self.task_list.insert(0, task);
        }

        fn step(&mut self) -> u32 {
            match self.cache {
                CacheState::EMPTY => {
                    if !self.task_list.is_empty() {
                        let peak = self.task_list.pop().unwrap();
                        self.disk.add_move_task(peak.track);
                        self.cache = CacheState::ACTIVE(peak);
                    }
                }
                CacheState::ACTIVE(f) => {
                    if self.disk.get_current_track() == f.track {
                        if self.disk.get_current_angle() == f.angle {
                            self.cache = CacheState::EMPTY;
                            return f.task_id;
                        } else {
                            if self.disk.is_rotating() {
                                self.disk.step()
                            } else {
                                self.disk.add_reading_task(f.angle);
                            }
                        }
                    } else {
                        self.disk.step();
                    }
                }
            }

            0
        }
    }

    impl<'a> Driver<'a> for ElevetorDriver<'a> {
        fn add_new_task(&mut self, task: &'a Task) {
            match self.disk.get_state() {
                DiskState::STOP => {
                    self.add_to_same_direction_list(task);
                }
                DiskState::READ(_) => {
                    if self.disk.get_current_track() == task.track {
                        self.add_to_same_direction_list(task);
                    }
                }
                DiskState::MOVE(state) => {
                    if self.disk.get_current_track() == task.track {
                        self.opposite_direction_list.insert(task.track, vec![task]);
                    } else {
                        match self.disk.calculate_moving_direction(task) {
                            MoveDirection::FORWARD => {
                                if state.direction == MoveDirection::FORWARD {
                                    self.add_to_same_direction_list(task);
                                } else {
                                    self.add_to_opposite_direction_list(task);
                                }
                            }
                            MoveDirection::BACKWARD => {
                                if state.direction == MoveDirection::BACKWARD {
                                    self.add_to_same_direction_list(task);
                                } else {
                                    self.add_to_opposite_direction_list(task);
                                }
                            }
                        }
                    }
                }
            }
        }

        fn step(&mut self) -> u32 {
            match self.cache {
                CacheState::EMPTY => {
                    let tasks = self
                        .same_direction_list
                        .get_mut(&self.disk.get_current_track());
                    match tasks {
                        Some(tasks) => {
                            let task = tasks.pop().unwrap();
                            if tasks.len() == 0 {
                                self.same_direction_list
                                    .remove(&self.disk.get_current_track());
                            }

                            self.cache = CacheState::ACTIVE(task);
                        }
                        None => {
                            if !self.same_direction_list.is_empty() {
                                let task = self.fetch_same_direction_task().unwrap();
                                self.cache = CacheState::ACTIVE(task);
                                self.disk.add_move_task(task.track);
                            } else {
                                let temp = self.same_direction_list.to_owned();
                                self.same_direction_list = self.opposite_direction_list.to_owned();
                                self.opposite_direction_list = temp;
                            }
                        }
                    }
                }
                CacheState::ACTIVE(f) => {
                    if self.disk.get_current_track() == f.track {
                        // println!("1");
                        if self.disk.get_current_angle() == f.angle {
                            self.cache = CacheState::EMPTY;
                            return f.task_id;
                        } else {
                            if self.disk.is_rotating() {
                                self.disk.step()
                            } else {
                                self.disk.add_reading_task(f.angle);
                            }
                        }
                    } else if self
                        .same_direction_list
                        .contains_key(&self.disk.get_current_track())
                    {
                        let new_task = self.fetch_a_task_for_current_track();
                        // println!("2");
                        match self.disk.get_state() {
                            DiskState::MOVE(_) => {
                                self.disk.detach_current_state();
                                self.add_to_same_direction_list(f);
                                self.cache = CacheState::ACTIVE(new_task);
                            }
                            _ => {}
                        }
                    } else {
                        // println!("{}", self.disk.get_current_track());
                        self.disk.step();
                    }
                }
            }

            0
        }
    }
}
