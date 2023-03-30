pub mod driver {
    use crate::disk::{disk::disk::Disk};

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

    impl Task {
        pub fn new(task_id: u32, track: u32, angle: u32) -> Task {
            Task {
                task_id: task_id,
                track: track,
                angle: angle,
            }
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
}
