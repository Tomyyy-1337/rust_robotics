use std::cmp::min;
use std::time::Duration;
use module::{Module, ModuleBuilder};
use ports::prelude::*;
use crate::meta_signals::MetaSignal;

pub struct MaximumFusion<D: Send> {
    pub stimulation: ReceivePort<MetaSignal>,
    pub inhibition: ReceivePort<MetaSignal>,
    pub activity: SendPort<MetaSignal>,
    pub target_rating: SendPort<MetaSignal>,

    data_ports: Vec<InnerPort<D>>,
    activity_ports: Vec<InnerPort<MetaSignal>>,
    pub output_port: SendPort<D>,
}

impl<D> MaximumFusion<D>
where
    Self: Send,
    D: Default + Send + 'static
{
    pub fn init() -> Self {
        MaximumFusion {
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

    pub fn new(
        cycle_time: Duration
    ) -> ModuleBuilder<MaximumFusion<D>> {
        ModuleBuilder::new(MaximumFusion::init(), cycle_time)
    }
}

impl<D> Module for MaximumFusion<D>
where
    Self: Send,
    D: Default + Send + 'static
{
    fn update(&mut self) {
        let max = self.data_ports.iter()
            .zip(self.activity_ports.iter())
            .map(|(d, a)| (d, a.get_data_blocking()))
            .max_by_key(|(_, activity)| *activity);
        if let Some((data_port, activity)) = max {
            self.output_port.connect_to_source(data_port);
            let stimulation = *self.stimulation.get_data();
            let inhibition = *self.inhibition.get_data();
            let potential = min(stimulation, MetaSignal::HIGH - inhibition);
            let activity = min(potential, activity);
            self.activity.send(activity);
            self.target_rating.send(activity);
        } else {
            self.output_port.send(D::default());
            self.activity.send(MetaSignal::LOW);
            self.target_rating.send(MetaSignal::LOW);
        }
    }
}