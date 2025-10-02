use std::cmp::min;
use std::time::Duration;
use derive_more::{Deref, DerefMut};
use module::{ModuleBuilder, Module};
use ports::prelude::*;
use meta_signals::MetaSignal;

/// An IB2C behavior module with stimulation, inhibition, activity and target_rating ports.
/// The transfer and target_rating functions are called periodically. Transfer will always be called before target_rating.
/// transfer is used to update the internal state while target_rating expresses how much the behavior wants to be active.
/// The activity of the behavior is calculated using stimulation, inhibition and target_rating.
/// The activity is the minimum op potential and target_rating. Where potential is the minimum of stimulation
/// and (HIGH - inhibition).
pub trait BehaviorModuleTrait: PortMethods + Default {
    /// Initialize the behavior module (optional).
    fn init() -> Self where Self: Sized {
        Self::default()
    }

    /// Called periodically. Use this to update internal state
    /// and read from or write to ports of your module.
    fn transfer(module: &mut BehaviorModule<Self>);

    /// Return the target rating of the behavior module used to calculate the activity.
    fn target_rating(module: &BehaviorModule<Self>) -> MetaSignal;

    /// Create a new behavior module with the specified cycle time.
    fn new(
        cycle_time: Duration
    ) -> ModuleBuilder<BehaviorModule<Self>> where Self: Sized {
        ModuleBuilder::new(BehaviorModule::new(Self::init()), cycle_time)
    }
}

/// Inner structure of a behavior module.
/// Used by the [`BehaviorModuleTrait`] to create a behavior module.
#[derive(PortMethods, Default, Deref, DerefMut)]
pub struct BehaviorModule<M: BehaviorModuleTrait> {
    #[deref] #[deref_mut]
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
    fn new(inner: M) -> Self {
        BehaviorModule {
            inner,
            stimulation: ReceivePort::new(MetaSignal::HIGH),
            inhibition: ReceivePort::new(MetaSignal::LOW),
            activity: SendPort::new(MetaSignal::LOW),
            target_rating: SendPort::new(MetaSignal::LOW),
        }
    }
}