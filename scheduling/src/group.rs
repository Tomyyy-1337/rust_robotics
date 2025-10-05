use std::time::Duration;
use derive_more::{Deref, DerefMut};
use crate::{Module, ModuleBuilder, ThreadContainer};
use crate::spawn_mode::SpawnMode;

pub trait Group {
    fn init(&mut self, group: &mut GroupBuilder);
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

#[derive(Deref, DerefMut)]
pub struct GroupConnector<G: Group> {
    #[deref] #[deref_mut]
    inner: G,

    builder: GroupBuilder,
}

impl GroupBuilder {
    pub fn empty() -> Self {
        Self {
            spawn_mode: SpawnMode::NewThread,
            children: GroupChildren {
                modules: Vec::new(),
                groups: Vec::new(),
            }
        }
    }

    pub fn new<G: Group>(mut group: G, spawn_mode: SpawnMode) -> GroupConnector<G> {
        let mut builder = Self {
            spawn_mode,
            children: GroupChildren {
                modules: Vec::new(),
                groups: Vec::new(),
            }
        };
        group.init(&mut builder);
        GroupConnector {
            inner: group,
            builder,
        }
        
    }

    pub fn add_module<M: Module + Send + 'static>(&mut self, builder: ModuleBuilder<M>) {
        self.children.modules.push(ModuleData {
            module: Box::new(builder.inner),
            cycle_time: builder.cycle_time,
            spawn_mode: builder.spawn_mode,
        });
    }

    pub fn add_group<G>(&mut self, group: G) 
    where 
        G: Into<GroupBuilder>
    {
        let group = group.into();
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

impl<G: Group> GroupConnector<G> {
    pub fn add_module<M: Module + Send + 'static>(&mut self, builder: ModuleBuilder<M>) {
        self.builder.add_module(builder);
    }

    pub fn add_group(&mut self, group: GroupBuilder) {
        self.builder.add_group(group);
    }

    pub fn spawn(self) {
        self.builder.spawn()
    }
}

impl<G: Group> Into<GroupBuilder> for GroupConnector<G> {
    fn into(self) -> GroupBuilder {
        self.builder
    }
}