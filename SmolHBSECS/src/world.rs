use SmolCommon::{WorldCommon, Resource};
use std::collections::HashMap;
use std::any::{Any, TypeId};

pub struct World{
    resources: HashMap<TypeId, Box<dyn Any>>
}

impl World{
    pub fn new() -> Self{
        World{
            resources: HashMap::new()
        }
    }
}

impl WorldCommon for World{
    fn get<'w, T: 'static>(&'w self) -> &'w T{
        self.resources.get(&TypeId::of::<T>()).unwrap().downcast_ref::<T>().unwrap()
    }

    fn get_mut<'w, T: 'static>(&'w mut self) -> &'w mut T{
        self.resources.get_mut(&TypeId::of::<T>()).unwrap().downcast_mut::<T>().unwrap()
    }

    fn insert<'w, R: 'static + Any>(&'w mut self, resource: R){
        self.resources.insert(TypeId::of::<R>(), Box::new(resource));
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
        let mut storage = VecStorage::<usize>::new();

        world.insert(storage);

        for i in 0..10{
            let e = Entity{index: i, generation: 0};
            world.get_mut::<VecStorage<usize>>().set(&e, i);
        }

        for (n, i) in world.get::<VecStorage<usize>>().iter().enumerate(){
            assert_eq!(n, *i);
        }
    }
}