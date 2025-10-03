use std::time::Duration;
use derive_more::{Deref, DerefMut};
use scheduling::Module;
use ports::prelude::PortMethods;

/// A basic scheduling, the update method will be called periodically.
pub trait BasicModuleTrait: PortMethods + Default {
    /// Initialize the basic scheduling (optional).
    fn init() -> Self where Self: Sized {
        Self::default()
    }

    /// Called periodically. Use this to update internal state
    /// and read from or write to ports.
    fn update(module: &mut BasicModule<Self>);

    /// Create a new basic module. Should be wrapped in a [`scheduling::ModuleBuilder::new`] to be added to a ThreadContainer.
    fn new() -> BasicModule<Self> where Self: Sized {
        BasicModule::new(Self::init())
    }
}

/// Inner structure of a basic scheduling.
/// Used by [`BasicModuleTrait`] to create a basic scheduling.
#[derive(Deref, DerefMut)]
pub struct BasicModule<M: BasicModuleTrait> {
    inner: M,
}

impl<M: BasicModuleTrait> Module for BasicModule<M> {
    fn update(&mut self) {
        self.inner.update_ports();
        M::update(self);
    }
}

impl<M: BasicModuleTrait> BasicModule<M> {
    fn new(inner: M) -> Self {
        BasicModule {
            inner,
        }
    }
}