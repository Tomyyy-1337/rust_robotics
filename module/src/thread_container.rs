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

    pub fn add_module<M>(&mut self, module: M, cycle_time: Duration)
    where
        M: Module,
    {
        self.modules.push(ModuleData::new(module, cycle_time));
        self.task_queue.push(
            QueueElement {
                next_run: Instant::now(),
                module_index: self.modules.len() - 1,
            }
        )
    }

    pub fn run(mut self)
    where {
        std::thread::spawn(move || {
            if self.modules.is_empty() {
                return;
            }
            loop {
                let next_run = self.task_queue.peek().unwrap().next_run;
                let now = Instant::now();
                if next_run > now {
                    sleep(next_run - now);
                } else {
                    let task = self.task_queue.pop().unwrap();
                    let module_data = &mut self.modules[task.module_index];
                    module_data.module.update();
                    self.task_queue.push(QueueElement::new(now + module_data.cycle_time, task.module_index));
                }
            }
        });
    }
}

impl QueueElement {
    fn new(next_run: Instant, module_index: usize) -> Self {
        Self { next_run, module_index }
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

impl ModuleData
{
    fn new<M>(module: M, cycle_time: Duration) -> Self
    where
        M: Module,
    {
        Self {
            module: Box::new(module),
            cycle_time,
        }
    }
}
