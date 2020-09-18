use SmolCommon::{WorldCommon, Resource};
use std::collections::HashMap;
use std::any::{Any, TypeId};

pub struct World{
    resources: HashMap<TypeId, Box<dyn Resource>>
}

impl WorldCommon for World{
    fn get<'w, T: 'static>(&'w self) -> &'w dyn Resource{
        self.resources.get(&TypeId::of::<T>()).unwrap()
    }

    fn get_mut<'w, T: 'static>(&'w mut self) -> &'w mut dyn Resource{
        self.resources.get_mut(&TypeId::of::<T>()).unwrap()
    }

    fn insert<'w, R: 'static + Resource>(&'w mut self, resource: R){
        self.resources.insert(TypeId::of::<R>(), Box::new(resource));
    }
}