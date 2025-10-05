use meta_signals::MetaSignal;
use ports::prelude::{ReceivePort, SendPort};

pub trait IB2CMetaSignals<> {
    fn stimulation(&mut self) -> &mut ReceivePort<MetaSignal>;
    fn inhibition(&mut self) -> &mut ReceivePort<MetaSignal>;
    fn activity(&mut self) -> &mut SendPort<MetaSignal>;
    fn target_rating(&mut self) -> &mut SendPort<MetaSignal>;
}