use std::ops::{Deref, DerefMut};
use std::time::Duration;
use module::{ModuleBuilder, Module};
use ports::prelude::PortMethods;

pub trait BasicModuleTrait: PortMethods + Default + Send + 'static {
    fn init() -> Self where Self: Sized {
        Self::default()
    }

    fn update(module: &mut BasicModule<Self>);

    fn new(cycle_time: Duration) -> ModuleBuilder<BasicModule<Self>> where Self: Sized {
        ModuleBuilder::new(BasicModule::new(Self::init()), cycle_time)
    }
}



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

impl<M: BasicModuleTrait> Deref for BasicModule<M> {
    type Target = M;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<M: BasicModuleTrait> DerefMut for BasicModule<M> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}