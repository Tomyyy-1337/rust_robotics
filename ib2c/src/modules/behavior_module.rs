use std::cmp::min;
use std::time::Duration;
use derived_deref::{Deref, DerefMut};
use module::{ModuleBuilder, Module};
use ports::prelude::*;
use crate::meta_signals::MetaSignal;

pub trait BehaviorModuleTrait: PortMethods + Default {
    fn init() -> Self where Self: Sized {
        Self::default()
    }
    
    fn transfer(module: &mut BehaviorModule<Self>);
    fn target_rating(module: &BehaviorModule<Self>) -> MetaSignal;

    fn new(
        cycle_time: Duration
    ) -> ModuleBuilder<BehaviorModule<Self>> where Self: Sized {
        ModuleBuilder::new(BehaviorModule::new(Self::init()), cycle_time)
    }
}

#[derive(PortMethods, Default, Deref, DerefMut)]
pub struct BehaviorModule<M: BehaviorModuleTrait> {
    #[deref]
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

        M::transfer(self);
        let target = M::target_rating(self);
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