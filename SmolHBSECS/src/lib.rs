pub mod component;
pub mod world;
pub mod system;

use SmolCommon::entity::*;
use SmolCommon::component::*;
use SmolCommon::system::WriteComp;
use SmolCommon::join::{JoinIter, Joinable};
use std::collections::VecDeque;

#[derive(Clone)]
pub struct Entity{
    index: usize,
    generation: usize,
}

impl EntityCommon for Entity{

    fn add<'d, T: Component>(&'d self, storage: &'d mut WriteComp<'d, T>, comp: T){
        let index = self.index;
        storage.set(&index, comp);
    }

    fn remove<'d, T: Component>(&'d self, storage: &'d mut WriteComp<'d, T>){
        storage.delete(&(self.index.clone()));
    }
}

impl PartialEq for Entity {
    fn eq(&self, other: &Self) -> bool {
        self.index == other.index && self.generation == other.generation
    }
}
impl Eq for Entity {}

struct EntityStorage<'w>{
    entities: Vec<Entity>,
    empties: VecDeque<Entity>,
    world: &'w world::World,
}

impl<'w> EntityStorage<'w>{
    pub fn new(world: &'w world::World) -> Self{
        EntityStorage{
            entities: Vec::new(),
            empties: VecDeque::new(),
            world,
        }
    }

    pub fn create_entity(&mut self) -> &Entity{
        match self.empties.pop_front(){
            Some(mut entity) => {
                self.entities.get(entity.index).unwrap()
            },
            None => {
                self.entities.push(Entity{index: self.entities.len(), generation: 0});
                &self.entities[self.entities.len()-1]
            },
        }
    }

    pub fn delete_entity(&mut self, entity: &Entity){
        self.entities[entity.index].generation += 1;
        self.empties.push_back(entity.clone());
    }
}

impl<'j, 'w: 'j> Joinable<'j> for &'j EntityStorage<'w>{
    type Target = &'j Entity;

    fn join(self) -> JoinIter<'j, Self::Target>{
        JoinIter{
            items: Box::new(
                self.entities.iter().filter_map(move |entity|{
                    Some((!self.empties.contains(entity), Some(entity)))
                })),
        }
    }
}