use SmolCommon::{WorldCommon, Resource};
use SmolCommon::component::{Component, ComponentStorage};
use std::collections::HashMap;
use std::any::{Any, TypeId};
use std::cell::{RefCell, Ref, RefMut};
use crate::component::VecStorage;

pub struct World{
    resources: HashMap<TypeId, RefCell<Box<dyn Any>>>,
    components: HashMap<TypeId, RefCell<Box<dyn Any>>>
}

impl World{
    pub fn new() -> Self{
        World{
            resources: HashMap::new(),
            components: HashMap::new()
        }
    }
}

impl WorldCommon for World{
    fn get<T: 'static>(& self) -> Ref<T>{
        Ref::map(self.resources.get(&TypeId::of::<T>()).unwrap().borrow(), 
            |r| r.downcast_ref::<T>().unwrap())
    }

    fn get_mut<T: 'static>(&self) -> RefMut<T>{
        RefMut::map(self.resources.get(&TypeId::of::<T>()).unwrap().borrow_mut(),
            |r| r.downcast_mut::<T>().unwrap())
    }

    fn insert<R: 'static + Any>(&mut self, resource: R){
        self.resources.insert(TypeId::of::<R>(), RefCell::new(Box::new(resource)));
    }

    fn get_comp<T: Component + 'static>(&self) -> Ref<ComponentStorage<T>>{
        Ref::map(self.components.get(&TypeId::of::<T>()).unwrap().borrow(),
            |r| r.downcast_ref::<VecStorage<T>>().unwrap())
    }

    fn get_comp_mut<T: Component + 'static>(&self) -> RefMut<ComponentStorage<T>>{
        RefMut::map(self.components.get(&TypeId::of::<T>()).unwrap().borrow_mut(),
            |r| r.downcast_mut::<VecStorage<T>>().unwrap())
    }

    fn register_comp<T: Component + 'static>(&mut self){
        self.components.insert(TypeId::of::<T>(), RefCell::new(Box::new(VecStorage::<T>::new())));
    }
}

#[cfg(test)]
mod tests{
    
    use super::*;
    use crate::component::VecStorage;
    use SmolCommon::component::ComponentStorage;
    use crate::Entity;

    #[test]
    fn create_world_add_component_storage(){
        let mut world = World::new();
        world.register_comp::<usize>();

        for i in 0..10{
            world.get_comp_mut::<usize>().set(&i, i);
        }

        for (n, i) in world.get_comp::<usize>().iter().enumerate(){
            let (valid, num) = i;
            let val = num.unwrap();
            if valid{
                assert_eq!(n, *val);
            }
        }
    }
}