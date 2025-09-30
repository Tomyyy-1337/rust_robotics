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
    const SPIN_TIME: Duration = Duration::ZERO;

    pub fn run(mut self)
    where {
        std::thread::spawn(move || {
            if self.modules.is_empty() {
                return;
            }
            loop {
                let next_run = self.task_queue.peek().unwrap().next_run;
                let start = Instant::now();
                if next_run > start {
                    let wait_time = (next_run - start).saturating_sub(Self::SPIN_TIME);
                    sleep(wait_time);
                } else {
                    let task_index = self.task_queue.pop().unwrap().module_index;
                    let module_data = &mut self.modules[task_index];
                    module_data.module.update();

                    let mut next = start + module_data.cycle_time;
                    let now = Instant::now();
                    if next < now {
                        eprintln!("Warning: Module {} is running behind schedule!", task_index);
                        next = now;
                    }
                    self.task_queue.push(QueueElement::new(next, task_index));
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
