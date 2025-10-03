use std::time::Duration;
use derive_more::with_trait::{Deref, DerefMut};
use crate::{SpawnMode, ThreadContainer};

/// A scheduling that can be added to a `ThreadContainer`.
/// The scheduling must implement the `update` method, which will be called
/// periodically based on the specified cycle time.
pub trait Module {
    /// Update the scheduling's internal state.
    fn update(&mut self);
}

#[derive(Deref, DerefMut)]
pub struct ModuleBuilder<M: Module> {
    #[deref] #[deref_mut]
    pub inner: M,
    pub cycle_time: Duration,
    pub spawn_mode: SpawnMode
}

impl<M: Module> ModuleBuilder<M> {
    /// create a new module builder. Wrapper for all modules.
    pub fn new(
        inner: M,
        cycle_time: Duration,
        spawn_mode: SpawnMode
    ) -> Self {
        Self { inner, cycle_time , spawn_mode }
    }
}