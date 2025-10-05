use derive_more::{Deref, DerefMut};
use ib2c_macros::IB2CMetaSignals;
use meta_signals::MetaSignal;
use ports::prelude::{ReceivePort, SendPort};
use scheduling::{Group, GroupBuilder};
use crate::ib2c_meta_signals::IB2CMetaSignals;

pub trait BehaviorGroupTrait: Default {
    fn init(group: &mut BehaviorGroup<Self>, builder: &mut GroupBuilder);

    fn new() -> BehaviorGroup<Self> {
        BehaviorGroup::new(Self::default())
    }
}

#[derive(Deref, DerefMut, IB2CMetaSignals)]
pub struct BehaviorGroup<G: BehaviorGroupTrait> {
    #[deref] #[deref_mut]
    inner: G,
    pub stimulation: ReceivePort<MetaSignal>,
    pub inhibition: ReceivePort<MetaSignal>,
    pub activity: SendPort<MetaSignal>,
    pub target_rating: SendPort<MetaSignal>,
}

impl<G: BehaviorGroupTrait> Group for BehaviorGroup<G> {
    fn init(&mut self, builder: &mut GroupBuilder) {
        println!("Initializing BasicGroup");
        G::init(self, builder);
    }
}

impl<G: BehaviorGroupTrait> BehaviorGroup<G> {
    pub fn new(inner: G) -> Self {
        BehaviorGroup {
            inner,
            stimulation: ReceivePort::new(MetaSignal::HIGH),
            inhibition: ReceivePort::new(MetaSignal::LOW),
            activity: SendPort::new(MetaSignal::LOW),
            target_rating: SendPort::new(MetaSignal::LOW),
        }
    }

    pub fn set_characteristic_module<M: IB2CMetaSignals>(&mut self, module: &mut M) {
        module.stimulation().connect_to_source(&self.stimulation);
        module.inhibition().connect_to_source(&self.inhibition);
        self.activity.connect_to_source(module.activity());
        self.target_rating.connect_to_source(module.target_rating());
    }
}

// impl<G: BehaviorGroupTrait> IB2CMetaSignals for BehaviorGroup<G> {
//     fn stimulation(&mut self) -> &mut ReceivePort<MetaSignal> {
//         &mut self.stimulation
//     }
//     fn inhibition(&mut self) -> &mut ReceivePort<MetaSignal> {
//         &mut self.inhibition
//     }
//     fn activity(&mut self) -> &mut SendPort<MetaSignal> {
//         &mut self.activity
//     }
//     fn target_rating(&mut self) -> &mut SendPort<MetaSignal> {
//         &mut self.target_rating
//     }
// }
