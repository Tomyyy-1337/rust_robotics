use std::collections::BinaryHeap;
use std::thread::sleep;
use std::time::{Duration, Instant};
use crate::module::Module;

struct QueueElement {
    next_run: Instant,
    module_index: usize,
}

struct ModuleData {
    module: Box<dyn Module>,
    cycle_time: Duration,
}

pub struct ThreadContainer {
    modules: Vec<ModuleData>,
    task_queue: BinaryHeap<QueueElement>,
}

impl ThreadContainer {
    pub fn new() -> Self {
        Self {
            modules: Vec::new(),
            task_queue: BinaryHeap::new(),
        }
    }

    pub fn add_module<M: Module>(&mut self, module: M, cycle_time: Duration) {
        self.add_dyn_module(Box::new(module), cycle_time)
    }

    pub fn add_dyn_module(&mut self, module: Box<dyn Module>, cycle_time: Duration){
        self.modules.push(ModuleData { module, cycle_time });
        self.task_queue.push(
            QueueElement {
                next_run: Instant::now(),
                module_index: self.modules.len() - 1,
            }
        )
    }

    const SPIN_TIME: Duration = Duration::ZERO;

    pub fn run(mut self) {
        if self.modules.is_empty() {
            eprintln!("No modules to run! Working thread will not be spawned.");
            return;
        }

        std::thread::spawn(move || {
            loop {
                let start = Instant::now();
                if !self.module_ready(start) {
                    continue;
                }
                let module_index = self.task_queue.pop().unwrap().module_index;
                let module_data = &mut self.modules[module_index];
                module_data.module.update();

                let next_run = Self::next_start(start, module_data.cycle_time);
                self.task_queue.push(QueueElement { next_run, module_index });
            }
        });
    }

    fn module_ready(&self, now: Instant) -> bool {
        let scheduled_start= self.task_queue.peek().unwrap().next_run;
        if scheduled_start > now {
            let wait_time = scheduled_start - now;
            if wait_time > Self::SPIN_TIME {
                sleep(wait_time - Self::SPIN_TIME);
            }
            return false;
        }
        true
    }

    fn next_start(last_run: Instant, cycle_time: Duration) -> Instant {
        let next = last_run + cycle_time;
        let now = Instant::now();
        if next < now {
            return now;
        }
        next
    }
}

impl Ord for QueueElement {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.next_run.cmp(&self.next_run)
    }
}
impl PartialOrd for QueueElement {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(other.next_run.cmp(&self.next_run))
    }
}
impl Eq for QueueElement {}
impl PartialEq for QueueElement {
    fn eq(&self, other: &Self) -> bool {
        self.next_run == other.next_run
    }
}
