use crate::component::{Component, ComponentStorage};
use crate::join::{Joinable, JoinIter};
use super::{WorldCommon, Resource, DepVec, AccessType};
use crate::entity::EntityCommon;

use std::ops::{Deref, DerefMut};

use parking_lot::{ MappedRwLockReadGuard, MappedRwLockWriteGuard};

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
        self.run(T::SystemData::get_data(world));
    }

    fn get_system_dependencies(&self, world: &W) -> DepVec {
        T::SystemData::get_dep_vec(world)
    }
}

pub trait SystemData<'d>{
    fn get_data<'w: 'd, W: WorldCommon>(world: &'w W) -> Self;
    fn get_dep_vec<'w: 'd, W: WorldCommon>(world: &W) -> DepVec;
}

pub trait WriteCompCommon<T, E: EntityCommon>{
    fn set(&mut self, entity: &E, comp: T){}

    fn delete(&mut self, entity: &E){}
}

impl_system_data_multi!{16}