use derive_more::{Deref, DerefMut};
use scheduling::{Group, GroupBuilder};

pub trait BasicGroupTrait: Default {
    fn init(&mut self, builder: &mut GroupBuilder) where Self: Sized;
    
    fn new() -> BasicGroup<Self> where Self: Sized {
        BasicGroup {
            inner: Self::default()
        }
    }
}

#[derive(Deref, DerefMut)]
pub struct BasicGroup<G: BasicGroupTrait> {
    inner: G,
}

impl<G: BasicGroupTrait> Group for BasicGroup<G> {
    fn init(&mut self, builder: &mut GroupBuilder) {
        println!("Initializing BasicGroup");
        self.inner.init(builder);
    }
}
