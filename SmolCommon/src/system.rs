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

pub trait Scheduler<'d, 'w: 'd, W: WorldCommon>{
    fn add<S:'w + System<'d, 'w, W>>(&mut self, system: S, name: &str, depend: Vec<&str>);

    fn run(&self, world: &'w W);
}

pub trait System<'d, 'w: 'd, W: WorldCommon>{
    type SystemData: SystemData<'d> + Sized;


    fn run(&self, resources: Self::SystemData);

    fn get_system_data(world: &'w W) -> Self::SystemData{
        Self::SystemData::get_data(world)
    }

    fn get_system_dependencies(&self, world: &W) -> DepVec{
        Self::SystemData::get_dep_vec(world)
    }
}

pub trait SystemRunner<'d, 'w: 'd, W: WorldCommon>{
    fn get_and_run(&self, world: &'w W);
    fn get_system_dependencies(&self, world: &W) -> DepVec;
}

impl<'d, 'w: 'd, W: WorldCommon, T, Q> SystemRunner<'d, 'w, W> for T
    where T: System<'d, 'w, W, SystemData = Q>,
          Q: SystemData<'d> + Sized{

    fn get_and_run(&self, world: &'w W){
        println!("got");
        self.run(T::SystemData::get_data(world));
        println!("ran");
    }

    fn get_system_dependencies(&self, world: &W) -> DepVec {
        T::SystemData::get_dep_vec(world)
    }
}

pub trait SystemData<'d>{
    fn get_data<'w: 'd, W: WorldCommon>(world: &'w W) -> Self;
    fn get_dep_vec<'w: 'd, W: WorldCommon>(world: &W) -> DepVec;
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

    fn get_dep_vec<'w: 'd, W: WorldCommon>(world: &W) -> DepVec{
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

    fn get_dep_vec<'w: 'd, W: WorldCommon>(world: &W) -> DepVec{
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

impl<'j, 'd: 'j, T> Joinable<'j> for &'j WriteComp<'d, T>
    where T: Component + 'j{
    type Target = &'j T;

    fn join(self) -> JoinIter<'j, Self::Target>{
        JoinIter{
            items: Box::new(self.comp.iter()),
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

    fn get_dep_vec<'w: 'd, W: WorldCommon>(world: &W) -> DepVec{
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

    fn get_dep_vec<'w: 'd, W: WorldCommon>(world: &W) -> DepVec{
        world.get_dep_vec_res::<T>(AccessType::Write)
    }
}

impl_system_data_multi!{16}