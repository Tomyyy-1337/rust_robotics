use ports::prelude::PortMethods;
use meta_signals::MetaSignal;
use crate::modules::general_fusion::{GeneralFusion, GeneralFusionTrait};

#[derive(Default, PortMethods)]
pub struct MaximumFusion { }

/// A fusion scheduling that connects the output port to the data port with the highest activity.
/// The target rating is the activity of the selected data port.
/// If no data ports are available, the output port is not connected and the target rating is LOW.
impl<D: Default> GeneralFusionTrait<D> for MaximumFusion {
    fn fuse(module: &mut GeneralFusion<Self, D>) -> MetaSignal {
        // find the data port with the highest activity
        let max = module.data_ports.iter()
            .zip(module.activity_ports.iter())
            .rev()
            .map(|(d, a)| (d, a.get_data()))
            .max_by_key(|(_, activity)| *activity);

        // connect the output port to the data port with the highest activity
        if let Some((data_port, max_activity)) = max {
            module.output_port.connect_to_source(data_port);
            return *max_activity
        }
        MetaSignal::LOW
    }
}