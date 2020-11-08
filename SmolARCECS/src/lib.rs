use SmolCommon::entity::EntityCommon;
use SmolCommon::system::WriteCompCommon;
use SmolCommon::component::Component;
use SmolCommon::WorldCommon;
use SmolCommon::DepVec;
use SmolCommon::Resource;
use SmolCommon::system::SystemData;
use std::ops::DerefMut;
use std::ops::Deref;
use SmolCommon::join::JoinIter;
use SmolCommon::join::Joinable;
use SmolCommon::AccessType;
use parking_lot::MappedRwLockWriteGuard;
use parking_lot::MappedRwLockReadGuard;
use parking_lot::RwLockWriteGuard;
use parking_lot::RwLockReadGuard;
use std::any::Any;
use std::marker::PhantomData;

mod world;

use world::World;

pub struct EntityStorage<'w>{
    world: &'w mut hecs::World,
}

impl<'w> EntityStorage<'w>{
    pub fn new(world: &'w mut hecs::World) -> Self{
        EntityStorage{
            world
        }
    }

    pub fn create_entity(&mut self) -> Entity{
        Entity{ent: self.world.spawn(())}
    }

    pub fn delete_entity(&mut self, entity: &Entity){
        self.world.despawn(entity.ent);
    }
}

#[derive(Eq, PartialEq)]
pub struct Entity{
    ent: hecs::Entity
}

impl EntityCommon for Entity{
    fn add<'e, T: 'static + Component + hecs::DynamicBundle, S: Any>(&'e self, storage: &'e mut S, comp: T) -> &'e Self{
        let storage = storage as &'e mut dyn Any;
        let storage = storage.downcast_mut::<WriteComp<T>>().unwrap();
        storage.world.insert(self.ent.clone(), comp);
        self
    }
    fn remove<'e, T: 'static + Component + hecs::DynamicBundle + hecs::Bundle, S: Any>(&'e self, storage: &'e mut S) -> &'e Self{
        let storage = storage as &'e mut dyn Any;
        let storage = storage.downcast_mut::<WriteComp<T>>().unwrap();
        storage.world.remove::<T>(self.ent.clone());
        self
    }
}


pub struct ReadComp<'d, T: 'static + Component>{
    world: &'d hecs::World,
    mark:  std::marker::PhantomData<T>
}

impl<'d, T: Component> ReadComp<'d, T>{
    pub fn get(&'d self, entity: &Entity) -> Option<&'d T>{
        self.world.get::<T>(entity.ent).map(|x| x.deref()).ok()
    }
}

impl<'d, T> SystemData<'d> for ReadComp<'d, T>
    where T: Component + 'static{
    fn get_data<'w: 'd, W: 'static + WorldCommon + Any>(world: &'w W) -> Self{
        let world = world as &'w dyn Any;
        let world = world.downcast_ref::<World>().unwrap();
        Self{
            world: &world.world,
            mark: PhantomData{}
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
        todo!()
    }
}

pub struct WriteComp<'d, T: Component>{
    world: &'d hecs::World,
    mark:  std::marker::PhantomData<T>
}

impl<'d, T: 'static + Component> WriteComp<'d, T>{
    pub fn get(&'d self, entity: &Entity) -> Option<&'d T>{
        self.world.get::<T>(entity.ent).map(|x| x.deref()).ok()
    }

    pub fn get_mut(&'d mut self, entity: &Entity) -> Option<&'d mut T>{
        self.world.get_mut::<T>(entity.ent).map(|x| x.deref_mut()).ok()
    }

    fn set(&mut self, entity: &Entity, comp: T){
        todo!()
    }

    fn delete(&mut self, entity: &Entity){
        todo!()
    }
}

impl<'d, T: Component + hecs::DynamicBundle + hecs::Bundle> WriteCompCommon<T, Entity> for WriteComp<'d, T>{
    fn set(&mut self, entity: &Entity, comp: T){
        self.world.insert(entity.ent, comp);
    }

    fn delete(&mut self, entity: &Entity){
        self.world.remove::<T>(entity.ent);
    }
}

impl<'d, T> SystemData<'d> for WriteComp<'d, T>
    where T: Component + 'static{
    fn get_data<'w: 'd, W: WorldCommon + Any + 'static>(world: &'w W) -> Self{
        let world = world as &'w Any;
        let world = world.downcast_ref::<World>().unwrap();
        Self{
            world: &world.world,
            mark: PhantomData{}
        }
    }

    fn get_dep_vec<'w: 'd, W: WorldCommon>(world: &W) -> DepVec{
        world.get_dep_vec_comp::<T>(AccessType::Write)
    }
}

impl<'j, 'd: 'j, T> Joinable<'j> for &'j mut WriteComp<'d, T>
    where T: Component + hecs::Query + 'static{
    type Target = &'j mut T;

    fn join(self) -> JoinIter<'j, Self::Target>{
        JoinIter{
            items: Box::new(self.world.query::<Self::Target>().iter().map(|(e, t)| (true, Some(t)))),
        }
    }
}

impl<'j, 'd: 'j, T> Joinable<'j> for &'j WriteComp<'d, T>
    where T: Component+ hecs::Query + 'static{
    type Target = &'j T;

    fn join(self) -> JoinIter<'j, Self::Target>{
        JoinIter{
            items: Box::new(self.world.query::<Self::Target>().iter().map(|(e, t)| (true, Some(t)))),
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

