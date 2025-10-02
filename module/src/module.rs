use std::ops::{Deref, DerefMut};
use std::time::Duration;
use crate::ThreadContainer;

/// A module that can be added to a `ThreadContainer`.
/// The module must implement the `update` method, which will be called
/// periodically based on the specified cycle time.
pub trait Module {
    /// Update the module's internal state.
    fn update(&mut self);
}

pub struct ModuleBuilder<M: Module> {
    pub inner: M,
    pub cycle_time: Duration,
}

impl<M: Module> ModuleBuilder<M> {
    pub fn new(
        inner: M,
        cycle_time: Duration,
    ) -> Self {
        Self { inner, cycle_time }
    }

    pub fn add_to_container(self, container: &mut ThreadContainer)
    where
        M: Send + 'static
    {
        container.add_module(self.inner, self.cycle_time);
    }
}

impl<M: Module> Deref for ModuleBuilder<M> {
    type Target = M;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<M: Module> DerefMut for ModuleBuilder<M> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}