pub mod entity;
pub mod component;

use std::any::Any;

pub trait WorldCommon{
    fn get<'w, T: Any>(&'w self) -> &'w dyn Resource;

    fn get_mut<'w, T: Any>(&'w mut self) -> &'w mut dyn Resource;

    fn insert<'w, R: 'static + Resource>(&'w mut self, resource: R);
}

pub trait Scheduler{
    fn new() -> Self;

    fn add<S: System>(&mut self, system: S);

    fn schedule(&self) -> dyn FnOnce() -> ();
}

pub trait System{
    type SystemData;

    fn init(&mut self, resources: Self::SystemData);

    fn run(&mut self, resources: Self::SystemData);
}

pub trait Resource{}

impl<T: Any> Resource for T{}