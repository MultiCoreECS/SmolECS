pub mod component;

use SmolCommon::entity::*;
use SmolCommon::component::*;
use std::collections::VecDeque;

#[derive(Clone)]
struct Entity{
    index: usize,
    generation: usize,
}

impl EntityCommon for Entity{}

impl PartialEq for Entity {
    fn eq(&self, other: &Self) -> bool {
        self.index == other.index && self.generation == other.generation
    }
}
impl Eq for Entity {}

struct EntityStorage{
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

    fn create_entity(&mut self) -> &Entity{
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

    fn delete_entity(&mut self, entity: &Entity){
        self.entities[entity.index].generation += 1;
        self.empties.push_back(entity.clone());
    }
}