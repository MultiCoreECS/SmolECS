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

    fn add<'e, 'd: 'e, T: Component>(&'e self, storage: &'e mut WriteComp<'d, T>, comp: T) -> &'e Self{
        storage.set(self.index, comp);
        self
    }

    fn remove<'e, 'd: 'e, T: Component>(&'e self, storage: &'e mut WriteComp<'d, T>) -> &'e Self{
        storage.delete(self.index);
        self
    }
}

impl PartialEq for Entity {
    fn eq(&self, other: &Self) -> bool {
        self.index == other.index && self.generation == other.generation
    }
}
impl Eq for Entity {}

pub struct EntityStorage{
    entities: Vec<Entity>,
    empties: VecDeque<Entity>,
}

impl EntityStorage{
    pub fn new() -> Self{
        EntityStorage{
            entities: Vec::new(),
            empties: VecDeque::new(),
        }
    }

    pub fn create_entity(&mut self) -> &Entity{
        match self.empties.pop_front(){
            Some(entity) => {
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

impl<'j> Joinable<'j> for &'j EntityStorage{
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