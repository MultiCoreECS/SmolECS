use crate::component::{Component, ComponentStorage};
use crate::entity::EntityCommon;
use crate::join::{Joinable, JoinIter};
use super::{WorldCommon, Resource, DepVec, AccessType};

use std::ops::{Deref, DerefMut};
use std::marker::PhantomData;
use std::cell::{RefCell, Ref, RefMut};

use bit_vec::BitVec;
use parking_lot::{RwLock, RwLockReadGuard, RwLockWriteGuard, MappedRwLockReadGuard, MappedRwLockWriteGuard};

use SmolCommonMacros::{impl_system_data, impl_system_data_multi};

pub trait Scheduler<'w, W: WorldCommon>{
    fn add<S: System<'w>>(&mut self, name: String, depend: Vec<String>);

    fn run(&mut self, world: &'w W);
}

pub trait System<'d>{
    type SystemData: SystemData<'d> + Sized;

    fn run(resources: Self::SystemData);

    fn get_system_data<'w: 'd, W: WorldCommon>(world: &'w W) -> Self::SystemData{
        Self::SystemData::get_data(world)
    }

    fn get_system_dependencies<'w: 'd, W: WorldCommon>(world: &'w W) -> DepVec{
        Self::SystemData::get_dep_vec(world)
    }
}

pub trait SystemData<'d>{
    fn get_data<'w: 'd, W: WorldCommon>(world: &'w W) -> Self;
    fn get_dep_vec<'w: 'd, W: WorldCommon>(world: &'w W) -> DepVec;
}

pub struct ReadComp<'d, T: 'static + Component>{
    comp: MappedRwLockReadGuard<'d, ComponentStorage<T>>
}

impl<'d, T: Component> ReadComp<'d, T>{
    pub fn get(&'d self, entity: usize) -> Option<&'d T>{
        self.comp.get(&entity)
    }
}

impl<'d, T> SystemData<'d> for ReadComp<'d, T>
    where T: Component + 'static{
    fn get_data<'w: 'd, W: WorldCommon>(world: &'w W) -> Self{
        Self{
            comp: world.get_comp::<T>()
        }
    }

    fn get_dep_vec<'w: 'd, W: WorldCommon>(world: &'w W) -> DepVec{
        world.get_dep_vec_comp::<T>(AccessType::Read)
    }
}

impl<'j, 'd, T> Joinable<'j> for &'j ReadComp<'d, T>
    where T: Component + 'j{
    type Target = &'j T;

    fn join(self) -> JoinIter<'j, Self::Target>{
        JoinIter{
            items: Box::new(self.comp.iter()),
        }
    }
}

pub struct WriteComp<'d, T: Component>{
    comp: MappedRwLockWriteGuard<'d, ComponentStorage<T>>
}

impl<'d, T: Component> WriteComp<'d, T>{
    pub fn get(&'d self, entity: &usize) -> Option<&'d T>{
        self.comp.get(entity)
    }

    pub fn get_mut(&'d mut self, entity: usize) -> Option<&'d mut T>{
        self.comp.get_mut(&entity)
    }

    pub fn set(&mut self, entity: usize, comp: T){
        self.comp.set(&entity, comp)
    }

    pub fn delete(&mut self, entity: usize){
        self.comp.delete(&entity);
    }
}

impl<'d, T> SystemData<'d> for WriteComp<'d, T>
    where T: Component + 'static{
    fn get_data<'w: 'd, W: WorldCommon>(world: &'w W) -> Self{
        Self{
            comp: world.get_comp_mut::<T>()
        }
    }

    fn get_dep_vec<'w: 'd, W: WorldCommon>(world: &'w W) -> DepVec{
        world.get_dep_vec_comp::<T>(AccessType::Write)
    }
}

impl<'j, 'd: 'j, T> Joinable<'j> for &'j mut WriteComp<'d, T>
    where T: Component + 'j{
    type Target = &'j mut T;

    fn join(self) -> JoinIter<'j, Self::Target>{
        JoinIter{
            items: Box::new(self.comp.iter_mut()),
        }
    }
}

pub struct Read<'d, T: 'static + Resource>{
    comp: MappedRwLockReadGuard<'d, T>
}

impl<'d, T: Resource> Deref for Read<'d, T>{
    type Target = T;

    fn deref(&self) -> &Self::Target{
        self.comp.deref()
    }
}

impl<'d, T> SystemData<'d> for Read<'d, T>
    where T: Resource + 'static{
    fn get_data<'w: 'd, W: WorldCommon>(world: &'w W) -> Self{
        Self{
            comp: world.get::<T>()
        }
    }

    fn get_dep_vec<'w: 'd, W: WorldCommon>(world: &'w W) -> DepVec{
        world.get_dep_vec_res::<T>(AccessType::Read)
    }
}

pub struct Write<'d, T: 'static + Resource>{
    comp: MappedRwLockWriteGuard<'d, T>
}

impl<'d, T: Resource> Deref for Write<'d, T>{
    type Target = T;

    fn deref(&self) -> &Self::Target{
        self.comp.deref()
    }
}

impl<'d, T: Resource> DerefMut for Write<'d, T>{
    fn deref_mut(&mut self) -> &mut Self::Target{
        self.comp.deref_mut()
    }
}

impl<'d, T> SystemData<'d> for Write<'d, T>
    where T: Resource + 'static{
    fn get_data<'w: 'd, W: WorldCommon>(world: &'w W) -> Self{
        Self{
            comp: world.get_mut::<T>()
        }
    }

    fn get_dep_vec<'w: 'd, W: WorldCommon>(world: &'w W) -> DepVec{
        world.get_dep_vec_res::<T>(AccessType::Write)
    }
}

impl_system_data_multi!{16}