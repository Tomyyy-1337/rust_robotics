use scheduling::{GroupBuilder, SpawnMode};

pub trait BasicGroupTrait {
    fn init(&mut self, builder: &mut GroupBuilder) where Self: Sized;

    fn new(
        spawn_mode: SpawnMode
    ) -> GroupBuilder where Self: Sized + Default {
        let mut builder = GroupBuilder::new(spawn_mode);
        let mut group = Self::default();
        Self::init(&mut group, &mut builder);
        builder
    }
}