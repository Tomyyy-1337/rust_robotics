use std::cmp::min;
use ports::prelude::PortMethods;
use crate::meta_signals::MetaSignal;
use crate::modules::general_fusion::{GeneralFusion, GeneralFusionTrait};

#[derive(Default, PortMethods)]
pub struct MaximumFusion { }

impl<D> GeneralFusionTrait<D> for MaximumFusion
where
    D: Default
{
    fn fuse(module: &mut GeneralFusion<Self, D>) {
        let max = module.data_ports.iter()
            .zip(module.activity_ports.iter())
            .map(|(d, a)| (d, a.get_data_blocking()))
            .max_by_key(|(_, activity)| *activity);
        if let Some((data_port, activity)) = max {
            module.output_port.connect_to_source(data_port);
            let stimulation = *module.stimulation.get_data();
            let inhibition = *module.inhibition.get_data();
            let potential = min(stimulation, MetaSignal::HIGH - inhibition);
            let activity = min(potential, activity);
            module.activity.send(activity);
            module.target_rating.send(activity);
        } else {
            module.output_port.send(D::default());
            module.activity.send(MetaSignal::LOW);
            module.target_rating.send(MetaSignal::LOW);
        }
    }
}