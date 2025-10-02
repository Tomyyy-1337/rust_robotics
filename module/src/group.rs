use std::time::Duration;
use crate::{Module, ModuleBuilder, ThreadContainer};

struct ModuleData {
    module: Box<dyn Module + Send>,
    cycle_time: Duration,
    add_to_group_thread: bool,
}

struct GroupData {
    group: GroupChildren,
    add_to_group_thread: bool,
}

pub struct GroupChildren {
    modules: Vec<ModuleData>,
    groups: Vec<GroupData>,
}

pub struct GroupBuilder {
    run_on_group_thread: bool,
    children: GroupChildren,
}

impl GroupBuilder {
    pub fn new(run_on_group_thread: bool) -> Self {
        Self {
            run_on_group_thread,
            children: GroupChildren {
                modules: Vec::new(),
                groups: Vec::new(),
            },
        }
    }

    pub fn add_module_builder<M: Module + Send + 'static>(&mut self, builder: ModuleBuilder<M>) {
        self.children.modules.push(ModuleData {
            module: Box::new(builder.inner),
            cycle_time: builder.cycle_time,
            add_to_group_thread: builder.run_on_group_thread,
        });
    }

    pub fn add_group(&mut self, group: GroupBuilder) {
        self.children.groups.push(GroupData {
            group: group.children,
            add_to_group_thread: group.run_on_group_thread,
        });
    }

    pub fn spawn(self) {
        let mut main_container = ThreadContainer::new();
        Self::spawn_on_thread(self.children, &mut main_container);
        main_container.run();
    }

    fn spawn_on_thread(group_children: GroupChildren, container: &mut ThreadContainer) {
        for child_module in group_children.modules {
            if child_module.add_to_group_thread {
                container.add_dyn_module(child_module.module, child_module.cycle_time);
            } else {
                let mut new_container = ThreadContainer::new();
                new_container.add_dyn_module(child_module.module, child_module.cycle_time);
                new_container.run();
            }
        }
        for child_group in group_children.groups {
            if child_group.add_to_group_thread {
                Self::spawn_on_thread(child_group.group, container);
            } else {
                let mut new_container = ThreadContainer::new();
                Self::spawn_on_thread(child_group.group, &mut new_container);
                new_container.run();
            }
        }
    }
}