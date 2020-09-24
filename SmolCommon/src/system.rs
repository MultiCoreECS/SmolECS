use crate::component::{Component, ComponentStorage};
use crate::entity::EntityCommon;
use super::{WorldCommon, Resource};

use std::ops::Deref;
use std::marker::PhantomData;
use std::cell::{RefCell, Ref, RefMut};

pub trait Scheduler<'w>{
    fn new() -> Self;

    fn add<S: System<'w>>(&mut self, system: S);

    fn schedule(&self) -> dyn FnOnce() -> ();
}

pub trait System<'w>{
    type SystemData: SystemData<'w> + Sized;

    fn init(resources: Self::SystemData);

    fn run(resources: Self::SystemData);

    fn get_system_data<W: WorldCommon>(world: &'w mut W) -> Self::SystemData{
        Self::SystemData::get_data(world)
    }
}

pub trait SystemData<'w>{
    fn get_data<W: WorldCommon>(world: &'w mut W) -> Self;
}

pub struct ReadComp<'w, T: Component>{
    comp: Ref<'w, ComponentStorage<T>>
}

impl<'w, T: Component> ReadComp<'w, T>{
    pub fn get(&'w self, entity: &usize) -> Option<&'w T>{
        self.comp.get(entity)
    }
}

impl<'w, T> SystemData<'w> for ReadComp<'w, T>
    where T: Component + 'static{
    fn get_data<W: WorldCommon>(world: &'w mut W) -> Self{
        Self{
            comp: world.get_comp::<T>()
        }
    }
}
