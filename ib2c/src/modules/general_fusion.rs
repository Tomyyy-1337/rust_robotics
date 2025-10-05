use std::time::Duration;
use derive_more::{Deref, DerefMut};
use scheduling::{Module, ModuleBuilder, SpawnMode};
use ports::prelude::*;
use meta_signals::MetaSignal;
use crate::ib2c_meta_signals::IB2CMetaSignals;
use crate::modules::behavior_module::{BehaviorModule, BehaviorModuleTrait};

/// A general fusion scheduling that can fuse multiple data inputs based on their activity levels.
/// The fusion strategy is defined by implementing this trait.
pub trait GeneralFusionTrait<D: Default>: PortMethods + Default {
    /// Initialize the fusion scheduling (optional).
    fn init() -> Self where Self: Sized {
        Self::default()
    }

    /// Fuse the inputs by connection a data port to the output port
    /// or by publishing a values to the output port.
    /// Return the target rating of the fusion.
    fn fuse(module: &mut GeneralFusion<Self, D>) -> MetaSignal;

    /// Create a new general fusion module. Should be wrapped in a [`scheduling::ModuleBuilder::new`] to be added to a ThreadContainer.
    fn new() -> GeneralFusion<Self, D>
    where
        Self: Sized,
    {
        GeneralFusion::new(Self::init())
    }
}

/// Inner structure of a general fusion scheduling.
/// Used by the [`GeneralFusionTrait`] to create a fusion scheduling.
#[derive(PortMethods, Deref, DerefMut)]
pub struct GeneralFusion<M, D>
where
    M: GeneralFusionTrait<D>,
    D: Default,
{
    #[deref] #[deref_mut]
    inner: M,

    pub stimulation: ReceivePort<MetaSignal>,
    pub inhibition: ReceivePort<MetaSignal>,
    pub activity: SendPort<MetaSignal>,
    pub target_rating: SendPort<MetaSignal>,

    pub data_ports: Vec<ReceivePort<D>>,
    pub activity_ports: Vec<ReceivePort<MetaSignal>>,
    pub output_port: SendPort<D>,
}

impl<M, D> Module for GeneralFusion<M, D>
where
    M: GeneralFusionTrait<D>,
    D: Default,
{
    fn update(&mut self) {
        self.update_ports();
        self.inner.update_ports();
        for port in &mut self.data_ports {
            port.update();
        }
        for port in &mut self.activity_ports {
            port.update();
        }

        let target = M::fuse(self);
        let stimulation = *self.stimulation.get_data();
        let inhibition = *self.inhibition.get_data();
        let potential = std::cmp::min(stimulation, MetaSignal::HIGH - inhibition);
        let activity = std::cmp::min(potential, target);
        self.activity.send(activity);
        self.target_rating.send(target);
    }
}

impl<M,D> GeneralFusion<M,D>
where
    M: GeneralFusionTrait<D>,
    D: Default,
{
    fn new(inner: M) -> Self {
        GeneralFusion {
            inner,
            stimulation: ReceivePort::new(MetaSignal::HIGH),
            inhibition: ReceivePort::new(MetaSignal::LOW),
            activity: SendPort::new(MetaSignal::LOW),
            target_rating: SendPort::new(MetaSignal::LOW),
            data_ports: Vec::new(),
            activity_ports: Vec::new(),
            output_port: SendPort::new(D::default()),
        }
    }

    /// Add a new scheduling to the fusion.
    pub fn add_module(&mut self, data_port: &InnerPort<D>, activity_port: &InnerPort<MetaSignal>) {
        let data_receive_port = ReceivePort::default();
        data_receive_port.connect_to_source(data_port);
        self.data_ports.push(data_receive_port);
        let activity_receive_port = ReceivePort::default();
        activity_receive_port.connect_to_source(activity_port);
        self.activity_ports.push(activity_receive_port);
    }
}

impl<M: GeneralFusionTrait<D>, D: Default> IB2CMetaSignals for GeneralFusion<M, D> {
    fn stimulation(&mut self) -> &mut ReceivePort<MetaSignal> {
        &mut self.stimulation
    }
    fn inhibition(&mut self) -> &mut ReceivePort<MetaSignal> {
        &mut self.inhibition
    }
    fn activity(&mut self) -> &mut SendPort<MetaSignal> {
        &mut self.activity
    }
    fn target_rating(&mut self) -> &mut SendPort<MetaSignal> {
        &mut self.target_rating
    }
}