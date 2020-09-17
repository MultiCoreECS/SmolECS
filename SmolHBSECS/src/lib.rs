pub mod component;

use SmolCommon::entity::*;
use SmolCommon::component::*;

struct Entity{
    index: usize,
    generation: usize,
}

impl EntityCommon for Entity{
    fn add_component<'a, C: Component, CS: ComponentStorage<C>>(&mut self, comp: C, comp_storage: &mut CS){
        todo!()
    }
}