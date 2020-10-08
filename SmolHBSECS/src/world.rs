use SmolCommon::{WorldCommon, Resource, DepVec, AccessType, BitVec};
use SmolCommon::component::{Component, ComponentStorage};
use std::collections::HashMap;
use std::any::{Any, TypeId};
use std::cell::{RefCell, Ref, RefMut};
use crate::component::VecStorage;

pub struct World{
    resource_ids: HashMap<TypeId, usize>,
    component_ids: HashMap<TypeId, usize>,
    resources: HashMap<TypeId, RefCell<Box<dyn Any>>>,
    components: HashMap<TypeId, RefCell<Box<dyn Any>>>
}

impl World{
    pub fn new() -> Self{
        World{
            resource_ids: HashMap::new(),
            component_ids: HashMap::new(),
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
        let id = TypeId::of::<R>();
        self.resource_ids.insert(id.clone(), self.resource_ids.len());
        self.resources.insert(id, RefCell::new(Box::new(resource)));
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
        let id = TypeId::of::<T>();
        self.component_ids.insert(id.clone(), self.component_ids.len());
        self.components.insert(id, RefCell::new(Box::new(VecStorage::<T>::new())));
    }
    
    fn get_dep_vec_res<T: Any>(&self, at: AccessType) -> DepVec{
        let mut res = BitVec::from_elem(self.resource_ids.len(), false);
        res.set(*self.resource_ids.get(&TypeId::of::<T>()).unwrap(), true);
        match at{
            AccessType::Read =>
            DepVec{
                comp_read: BitVec::new(),
                comp_write: BitVec::new(),
                res_read: res,
                res_write: BitVec::new(),
            },
            AccessType::Write =>
            DepVec{
                comp_read: BitVec::new(),
                comp_write: BitVec::new(),
                res_write: res,
                res_read: BitVec::new(),
            },
        }
    }

    fn get_dep_vec_comp<T: Any>(&self, at: AccessType) -> DepVec{
        let mut comp = BitVec::from_elem(self.component_ids.len(), false);
        comp.set(*self.component_ids.get(&TypeId::of::<T>()).unwrap(), true);
        match at{
            AccessType::Read =>
            DepVec{
                res_read: BitVec::new(),
                res_write: BitVec::new(),
                comp_read: comp,
                comp_write: BitVec::new(),
            },
            AccessType::Write =>
            DepVec{
                res_read: BitVec::new(),
                res_write: BitVec::new(),
                comp_write: comp,
                comp_read: BitVec::new(),
            },
        }
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