use module::GroupBuilder;

pub trait BasicGroupTrait {
    fn init(&mut self, builder: &mut GroupBuilder) where Self: Sized;

    fn new(
        run_on_group_thread: bool
    ) -> GroupBuilder where Self: Sized + Default {
        let mut builder = GroupBuilder::new(run_on_group_thread);
        let mut group = Self::default();
        Self::init(&mut group, &mut builder);
        builder
    }
}