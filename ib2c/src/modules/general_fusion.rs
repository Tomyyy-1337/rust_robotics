use std::time::Duration;
use derived_deref::{Deref, DerefMut};
use module::{Module, ModuleBuilder};
use ports::prelude::*;
use crate::meta_signals::MetaSignal;

pub trait GeneralFusionTrait<D: Default>: PortMethods + Default {
    fn init() -> Self where Self: Sized {
        Self::default()
    }

    fn fuse(module: &mut GeneralFusion<Self, D>);

    fn new(cycle: Duration) -> ModuleBuilder<GeneralFusion<Self, D>>
    where
        Self: Sized,
    {
        ModuleBuilder::new(GeneralFusion::new(Self::init()), cycle)
    }
}

#[derive(PortMethods, Deref, DerefMut)]
pub struct GeneralFusion<M, D>
where
    M: GeneralFusionTrait<D>,
    D: Default,
{
    #[deref]
    inner: M,

    pub stimulation: ReceivePort<MetaSignal>,
    pub inhibition: ReceivePort<MetaSignal>,
    pub activity: SendPort<MetaSignal>,
    pub target_rating: SendPort<MetaSignal>,

    pub data_ports: Vec<InnerPort<D>>,
    pub activity_ports: Vec<InnerPort<MetaSignal>>,
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
        M::fuse(self);
    }
}

impl<M,D> GeneralFusion<M,D>
where
    M: GeneralFusionTrait<D>,
    D: Default,
{
    pub fn new(inner: M) -> Self {
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

    pub fn add_module(&mut self, data_port: &InnerPort<D>, activity_port: &InnerPort<MetaSignal>) {
        self.data_ports.push(data_port.clone());
        self.activity_ports.push(activity_port.clone());
    }
}