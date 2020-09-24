use crate::component::{Component, ComponentStorage};
use crate::entity::EntityCommon;
use crate::join::{Joinable, JoinIter};
use super::{WorldCommon, Resource};

use std::ops::{Deref, DerefMut};
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

impl<'w, T> Joinable<'w> for &'w ReadComp<'w, T>
    where T: Component + 'w{
    type Target = &'w T;

    fn join(self) -> JoinIter<'w, Self::Target>{
        JoinIter{
            items: Box::new(self.comp.iter()),
        }
    }
}

pub struct WriteComp<'w, T: Component>{
    comp: RefMut<'w, ComponentStorage<T>>
}

impl<'w, T: Component> WriteComp<'w, T>{
    pub fn get(&'w self, entity: &usize) -> Option<&'w T>{
        self.comp.get(entity)
    }

    pub fn get_mut(&'w mut self, entity: &usize) -> Option<&'w mut T>{
        self.comp.get_mut(entity)
    }

    pub fn set(&'w mut self, entity: &usize, comp: T){
        self.comp.set(entity, comp)
    }
}

impl<'w, T> SystemData<'w> for WriteComp<'w, T>
    where T: Component + 'static{
    fn get_data<W: WorldCommon>(world: &'w mut W) -> Self{
        Self{
            comp: world.get_comp_mut::<T>()
        }
    }
}

impl<'w, T> Joinable<'w> for &'w mut WriteComp<'w, T>
    where T: Component + 'w{
    type Target = &'w mut T;

    fn join(self) -> JoinIter<'w, Self::Target>{
        JoinIter{
            items: Box::new(self.comp.iter_mut()),
        }
    }
}

pub struct Read<'w, T: Resource>{
    comp: Ref<'w, T>
}

impl<'w, T: Resource> Deref for Read<'w, T>{
    type Target = T;

    fn deref(&self) -> &Self::Target{
        self.comp.deref()
    }
}

impl<'w, T> SystemData<'w> for Read<'w, T>
    where T: Resource + 'static{
    fn get_data<W: WorldCommon>(world: &'w mut W) -> Self{
        Self{
            comp: world.get::<T>()
        }
    }
}

pub struct Write<'w, T: Resource>{
    comp: RefMut<'w, T>
}

impl<'w, T: Resource> Deref for Write<'w, T>{
    type Target = T;

    fn deref(&self) -> &Self::Target{
        self.comp.deref()
    }
}

impl<'w, T: Resource> DerefMut for Write<'w, T>{
    fn deref_mut(&mut self) -> &mut Self::Target{
        self.comp.deref_mut()
    }
}

impl<'w, T> SystemData<'w> for Write<'w, T>
    where T: Resource + 'static{
    fn get_data<W: WorldCommon>(world: &'w mut W) -> Self{
        Self{
            comp: world.get_mut::<T>()
        }
    }
}