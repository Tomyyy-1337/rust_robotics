use std::time::Duration;
use crate::{Module, ModuleBuilder, ThreadContainer};

#[derive(Copy, Clone)]
pub enum SpawnMode {
    GroupThread,
    NewThread
}

struct ModuleData {
    module: Box<dyn Module + Send>,
    cycle_time: Duration,
    spawn_mode: SpawnMode,
}

struct GroupData {
    group: GroupChildren,
    spawn_mode: SpawnMode,
}

pub struct GroupChildren {
    modules: Vec<ModuleData>,
    groups: Vec<GroupData>,
}

pub struct GroupBuilder {
    spawn_mode: SpawnMode,
    children: GroupChildren,
}

impl GroupBuilder {
    pub fn new(spawn_mode: SpawnMode) -> Self {
        Self {
            spawn_mode,
            children: GroupChildren {
                modules: Vec::new(),
                groups: Vec::new(),
            },
        }
    }

    pub fn add_module<M: Module + Send + 'static>(&mut self, builder: ModuleBuilder<M>) {
        self.children.modules.push(ModuleData {
            module: Box::new(builder.inner),
            cycle_time: builder.cycle_time,
            spawn_mode: builder.spawn_mode,
        });
    }

    pub fn add_group(&mut self, group: GroupBuilder) {
        self.children.groups.push(GroupData {
            group: group.children,
            spawn_mode: group.spawn_mode
        });
    }

    pub fn spawn(self) {
        let mut main_container = ThreadContainer::new();
        Self::spawn_on_thread(self.children, &mut main_container);
        main_container.run();
    }

    fn spawn_on_thread(group_children: GroupChildren, container: &mut ThreadContainer) {
        for child_module in group_children.modules {
            match child_module.spawn_mode {
                SpawnMode::GroupThread => container.add_dyn_module(child_module.module, child_module.cycle_time),
                SpawnMode::NewThread => {
                    let mut new_container = ThreadContainer::new();
                    new_container.add_dyn_module(child_module.module, child_module.cycle_time);
                    new_container.run();
                }
            }
        }
        for child_group in group_children.groups {
            match child_group.spawn_mode {
                SpawnMode::GroupThread => Self::spawn_on_thread(child_group.group, container),
                SpawnMode::NewThread => {
                    let mut new_container = ThreadContainer::new();
                    Self::spawn_on_thread(child_group.group, &mut new_container);
                    new_container.run();
                }
            }
        }
    }
}