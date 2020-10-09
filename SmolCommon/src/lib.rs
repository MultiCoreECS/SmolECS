pub mod entity;
pub mod component;
pub mod system;
pub mod join;
use std::any::Any;

use std::cell::{RefCell, Ref, RefMut};
use std::ops::{Deref, DerefMut};
use component::{ComponentStorage, Component};
use system::Scheduler;

use parking_lot::{RwLock, RwLockReadGuard, RwLockWriteGuard, MappedRwLockReadGuard, MappedRwLockWriteGuard};

pub use bit_vec::BitVec;

pub trait WorldCommon{
    fn get<T: Any>(&self) -> MappedRwLockReadGuard<T>;

    fn get_mut<T: Any>(&self) -> MappedRwLockWriteGuard<T>;

    fn insert<R: 'static + Any>(&mut self, resource: R);
    
    fn get_comp<T: Component + Any>(&self) -> MappedRwLockReadGuard<ComponentStorage<T>>;

    fn get_comp_mut<T: Component + Any>(&self) -> MappedRwLockWriteGuard<ComponentStorage<T>>;

    fn register_comp<T: 'static + Component + Any>(&mut self);

    fn get_dep_vec_res<T: Any>(&self, at: AccessType) -> DepVec;

    fn get_dep_vec_comp<T: Any>(&self, at: AccessType) -> DepVec;
}

#[derive(Clone)]
pub struct DepVec{
    pub res_read: BitVec,
    pub res_write: BitVec,
    pub comp_read: BitVec,
    pub comp_write: BitVec,
}

impl DepVec{

    fn len_fix(&mut self, other: &mut DepVec){
        if self.res_read.len() < other.res_read.len(){
            self.res_read.grow(other.res_read.len() - self.res_read.len(), false);
        }
        if self.res_read.len() > other.res_read.len(){
            other.res_read.grow(self.res_read.len() - other.res_read.len(), false);
        }
        
        if self.res_write.len() < other.res_write.len(){
            self.res_write.grow(other.res_write.len() - self.res_write.len(), false);
        }
        if self.res_write.len() > other.res_write.len(){
            other.res_write.grow(self.res_write.len() - other.res_write.len(), false);
        }
        
        if self.comp_read.len() < other.comp_read.len(){
            self.comp_read.grow(other.comp_read.len() - self.comp_read.len(), false);
        }
        if self.comp_read.len() > other.comp_read.len(){
            other.comp_read.grow(self.comp_read.len() - other.comp_read.len(), false);
        }
        
        if self.comp_write.len() < other.comp_write.len(){
            self.comp_write.grow(other.comp_write.len() - self.comp_write.len(), false);
        }
        if self.comp_write.len() > other.comp_write.len(){
            other.comp_write.grow(self.comp_write.len() - other.comp_write.len(), false);
        }
    }

    pub fn and(&mut self, other: &DepVec) -> DepVec{
        let mut check = self.clone();
        let mut other = other.clone();
        check.len_fix(&mut other);

        let mut res_read = check.res_read.clone();
        res_read.and(&other.res_read);
        let mut res_write = check.res_write.clone();
        res_write.and(&other.res_read);
        
        let mut comp_read = check.comp_read.clone();
        comp_read.and(&other.comp_read);
        let mut comp_write = check.comp_write.clone();
        comp_write.and(&other.comp_write);

        DepVec{
            res_read,
            res_write,
            comp_read,
            comp_write,
        }
    }

    fn len_fix_single(&mut self, other: &mut BitVec){
        if self.res_read.len() < other.len(){
            self.res_read.grow(other.len() - self.res_read.len(), false);
        }
        if self.res_read.len() > other.len(){
            other.grow(self.res_read.len() - other.len(), false);
        }
        
        if self.res_write.len() < other.len(){
            self.res_write.grow(other.len() - self.res_write.len(), false);
        }
        if self.res_write.len() > other.len(){
            other.grow(self.res_write.len() - other.len(), false);
        }
        
        if self.comp_read.len() < other.len(){
            self.comp_read.grow(other.len() - self.comp_read.len(), false);
        }
        if self.comp_read.len() > other.len(){
            other.grow(self.comp_read.len() - other.len(), false);
        }
        
        if self.comp_write.len() < other.len(){
            self.comp_write.grow(other.len() - self.comp_write.len(), false);
        }
        if self.comp_write.len() > other.len(){
            other.grow(self.comp_write.len() - other.len(), false);
        }
    }

    pub fn intersection(&self, mut other_res: BitVec, mut other_comp: BitVec) -> bool{
        let mut check = self.clone();
        check.len_fix_single(&mut other_res);

        let mut res_read = check.res_read.clone();
        res_read.and(&other_res);
        let mut res_write = check.res_write.clone();
        res_write.and(&other_res);
        
        if res_read.any() || res_write.any(){
            return true;
        }

        check.len_fix_single(&mut other_comp);

        let mut comp_read = check.comp_read.clone();
        comp_read.and(&other_comp);
        let mut comp_write = check.comp_write.clone();
        comp_write.and(&other_comp);

        if comp_read.any() || comp_write.any(){
            return true;
        }

        false
    }
}

pub enum AccessType{
    Read,
    Write
}

pub trait Resource{}

impl<T: Any + Send + Sync> Resource for T{}