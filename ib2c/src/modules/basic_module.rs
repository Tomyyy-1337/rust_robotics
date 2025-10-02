use std::time::Duration;
use derive_more::{Deref, DerefMut};
use module::{ModuleBuilder, Module};
use ports::prelude::PortMethods;

/// A basic module, the update method will be called periodically.
pub trait BasicModuleTrait: PortMethods + Default {
    /// Initialize the basic module (optional).
    fn init() -> Self where Self: Sized {
        Self::default()
    }

    /// Called periodically. Use this to update internal state
    /// and read from or write to ports.
    fn update(module: &mut BasicModule<Self>);

    /// Create a new basic module with the specified cycle time.
    fn new(
        cycle_time: Duration,
    ) -> ModuleBuilder<BasicModule<Self>> where Self: Sized {
        ModuleBuilder::new(BasicModule::new(Self::init()), cycle_time)
    }
}

/// Inner structure of a basic module.
/// Used by the [`BasicModuleTrait`] to create a basic module.
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