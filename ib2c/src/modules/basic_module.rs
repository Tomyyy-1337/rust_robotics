use std::time::Duration;
use derived_deref::{Deref, DerefMut};
use module::{ModuleBuilder, Module};
use ports::prelude::PortMethods;

pub trait BasicModuleTrait: PortMethods + Default {
    fn init() -> Self where Self: Sized {
        Self::default()
    }

    fn update(module: &mut BasicModule<Self>);

    fn new(
        cycle_time: Duration,
    ) -> ModuleBuilder<BasicModule<Self>> where Self: Sized {
        ModuleBuilder::new(BasicModule::new(Self::init()), cycle_time)
    }
}

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
    pub fn new(inner: M) -> Self {
        BasicModule {
            inner,
        }
    }
}