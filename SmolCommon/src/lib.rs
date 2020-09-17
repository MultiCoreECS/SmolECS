pub mod entity;
pub mod component;

pub trait World<'w>{
    fn get<T>(&self) -> &'w dyn Resource<T>;

    fn get_mut<T>(&mut self) -> &'w mut dyn Resource<T>;
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

pub trait Resource<T>{}
