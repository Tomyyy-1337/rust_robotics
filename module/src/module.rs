use std::time::Duration;
use derive_more::with_trait::{Deref, DerefMut};
use crate::ThreadContainer;

/// A module that can be added to a `ThreadContainer`.
/// The module must implement the `update` method, which will be called
/// periodically based on the specified cycle time.
pub trait Module {
    /// Update the module's internal state.
    fn update(&mut self);
}

#[derive(Deref, DerefMut)]
pub struct ModuleBuilder<M: Module> {
    #[deref] #[deref_mut]
    pub inner: M,
    pub cycle_time: Duration,
    pub run_on_group_thread: bool,
}

impl<M: Module> ModuleBuilder<M> {
    pub fn new(
        inner: M,
        cycle_time: Duration,
        run_on_group_thread: bool,
    ) -> Self {
        Self { inner, cycle_time , run_on_group_thread }
    }

    pub fn add_to_container(self, container: &mut ThreadContainer)
    where
        M: Send + 'static
    {
        container.add_module(self.inner, self.cycle_time);
    }
}