use std::cmp::min;
use std::ops::{Deref, DerefMut};
use std::time::Duration;
use module::{ModuleBuilder, Module};
use ports::prelude::{ports, ReceivePort, SendPort, PortMethods};
use crate::meta_signals::MetaSignal;

pub trait BehaviorModuleTrait: PortMethods + Default + Send + 'static {
    fn init() -> Self where Self: Sized {
        Self::default()
    }
    fn transfer(&mut self);
    fn target_rating(&self) -> MetaSignal;

    fn new(cycle_time: Duration) -> ModuleBuilder<BehaviorModule<Self>> where Self: Sized {
        ModuleBuilder::new(BehaviorModule::new(Self::init()), cycle_time)
    }
}

#[ports]
pub struct BehaviorModule<M: BehaviorModuleTrait> {
    inner: M,
    pub stimulation: ReceivePort<MetaSignal>,
    pub inhibition: ReceivePort<MetaSignal>,
    pub activity: SendPort<MetaSignal>,
    pub target_rating: SendPort<MetaSignal>,
}

impl<M: BehaviorModuleTrait> Module for BehaviorModule<M> {
    fn update(&mut self) {
        self.inner.update_ports();
        self.update_ports();
        self.inner.transfer();
        let target = self.inner.target_rating();
        let stimulation = *self.stimulation.get_data();
        let inhibition = *self.inhibition.get_data();

        let potential = min(stimulation, MetaSignal::HIGH - inhibition);
        let activity = min(potential, target);
        self.activity.send(activity);
        self.target_rating.send(target);
    }
}

impl<M: BehaviorModuleTrait> BehaviorModule<M> {
    pub fn new(inner: M) -> Self {
        BehaviorModule {
            inner,
            stimulation: ReceivePort::new(MetaSignal::HIGH),
            inhibition: ReceivePort::new(MetaSignal::LOW),
            activity: SendPort::new(MetaSignal::LOW),
            target_rating: SendPort::new(MetaSignal::LOW),
        }
    }
}

impl<M: BehaviorModuleTrait> Deref for BehaviorModule<M> {
    type Target = M;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<M: BehaviorModuleTrait> DerefMut for BehaviorModule<M> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}