pub mod entity;
pub mod component;
pub mod system;
use std::any::Any;

use std::cell::{RefCell, Ref, RefMut};
use component::{ComponentStorage, Component};

pub trait WorldCommon{
    fn get<T: Any>(&self) -> Ref<T>;

    fn get_mut<T: Any>(&mut self) -> RefMut<T>;

    fn insert<R: 'static + Any>(&mut self, resource: R);
    
    fn get_comp<T: Any>(&self) -> Ref<ComponentStorage<T>>;

    fn get_mut_comp<T: Any>(&mut self) -> RefMut<ComponentStorage<T>>;

    fn register_comp<T: 'static + Component + Any>(&mut self);
}

pub trait Resource{}

impl<T: Any + Send + Sync> Resource for T{}