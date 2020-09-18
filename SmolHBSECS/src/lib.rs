pub mod component;
pub mod world;

use SmolCommon::entity::*;
use SmolCommon::component::*;
use std::collections::VecDeque;

#[derive(Clone)]
pub struct Entity{
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

