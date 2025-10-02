use std::collections::BinaryHeap;
use std::thread::sleep;
use std::time::{Duration, Instant};
use crate::module::Module;

/// A Task, representing a module and its next scheduled start time
/// The ordering is reversed to make BinaryHeap a min-heap based on next_run time
struct Task {
    scheduled_start: Instant,
    module_index: usize,
}

/// Generic dyn Module and its associated cycle time
struct ModuleData {
    module: Box<dyn Module + Send>,
    cycle_time: Duration,
}

/// A container that manages and runs multiple modules in a separate thread
/// Each module is scheduled to run based on its specified cycle time
/// Modules never run more frequently than their cycle time, but may run less frequently
pub struct ThreadContainer {
    modules: Vec<ModuleData>,
    task_queue: BinaryHeap<Task>,
}

impl ThreadContainer {
    /// Creates a new, empty ThreadContainer
    /// The container can be used to add modules and then run them in a separate thread
    /// with each module being called based on its cycle time.
    pub fn new() -> Self {
        Self {
            modules: Vec::new(),
            task_queue: BinaryHeap::new(),
        }
    }

    /// Adds a module to the container with the specified cycle time
    /// The module will be called every `cycle_time` duration in the working thread
    pub fn add_module<M: Module + Send + 'static>(&mut self, module: M, cycle_time: Duration) {
        self.add_dyn_module(Box::new(module), cycle_time)
    }

    /// Adds a boxed module to the container with the specified cycle time
    /// The module will be called every `cycle_time` duration in the working thread
    pub fn add_dyn_module(&mut self, module: Box<dyn Module + Send>, cycle_time: Duration){
        self.modules.push(ModuleData { module, cycle_time });
        self.task_queue.push(
            Task {
                scheduled_start: Instant::now(),
                module_index: self.modules.len() - 1,
            }
        )
    }

    /// Starts the working thread that schedules modules based on their cycle times
    /// calling their `update` method when it's time
    pub fn run(mut self) {
        println!("Running threads");
        std::thread::spawn(move || {
            while let Some(mut task) = self.task_queue.pop() {
                self.wait_for_module(task.scheduled_start);

                let ModuleData { module, cycle_time } = &mut self.modules[task.module_index];
                module.update();

                task.scheduled_start = Self::next_start(task.scheduled_start, *cycle_time);
                self.task_queue.push(task);
            }
            println!("Threads stopped");
        });
    }

    /// Sleeps until the next module is scheduled to run
    fn wait_for_module(&self, scheduled_start: Instant) {
        let now = Instant::now();
        if scheduled_start > now {
            let sleep_duration = scheduled_start - now;
            sleep(sleep_duration);
        }
    }

    /// Calculates the next start time for a module, ensuring it is not in the past
    fn next_start(last_run: Instant, cycle_time: Duration) -> Instant {
        let next = last_run + cycle_time;
        let now = Instant::now();
        if next < now {
            return now;
        }
        next
    }
}

impl Ord for Task {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.scheduled_start.cmp(&self.scheduled_start)
    }
}
impl PartialOrd for Task {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(other.scheduled_start.cmp(&self.scheduled_start))
    }
}
impl Eq for Task {}
impl PartialEq for Task {
    fn eq(&self, other: &Self) -> bool {
        self.scheduled_start == other.scheduled_start
    }
}

#[cfg(test)]
mod tests {
    use crate::Module;

    struct TestModule {
        count: usize,
        sleep_time: std::time::Duration,
        channel: std::sync::mpsc::Sender<usize>,
    }
    impl Module for TestModule {
        fn update(&mut self) {
            std::thread::sleep(self.sleep_time);
            self.count += 1;
            self.channel.send(self.count).unwrap();
        }
    }

    #[test]
    fn it_works() {
        let (result_tx, result_rx) = std::sync::mpsc::channel();
        let module_1 = TestModule {
            count: 0,
            channel: result_tx,
            sleep_time: std::time::Duration::ZERO
        };

        let mut container = super::ThreadContainer::new();
        container.add_module(module_1, std::time::Duration::from_millis(10));
        container.run();

        for i in 1..10 {
            let received = result_rx.recv().unwrap();
            assert_eq!(received, i);
        }
    }

    #[test]
    fn two_modules() {
        let (result_tx_1, result_rx_1) = std::sync::mpsc::channel();
        let (result_tx_2, result_rx_2) = std::sync::mpsc::channel();
        let module_1 = TestModule { count: 0, sleep_time: std::time::Duration::from_millis(5), channel: result_tx_1 };
        let module_2 = TestModule { count: 0, sleep_time: std::time::Duration::from_millis(5), channel: result_tx_2 };

        let mut container = super::ThreadContainer::new();
        container.add_module(module_1, std::time::Duration::from_millis(10));
        container.add_module(module_2, std::time::Duration::from_millis(20));
        container.run();

        for i in 1..10 {
            let received_1 = result_rx_1.recv().unwrap();
            assert_eq!(received_1, i);
        }

        let mut had_to_wait = false;
        for i in 1..10 {
            match result_rx_2.try_recv() {
                Ok(v) => {
                    assert_eq!(v, i);
                    continue;
                }
                Err(std::sync::mpsc::TryRecvError::Empty) => {
                    had_to_wait = true;
                    let received_2 = result_rx_2.recv().unwrap();
                    assert_eq!(received_2, i);
                },
                Err(e) => panic!("Unexpected error: {:?}", e),
            }
        }
        assert!(had_to_wait);

    }
}