use super::component::*;

pub trait Entity{
    fn add_component<'a, C: Component, CS: ComponentStorage<'a, C>>(&mut self, comp: C, comp_storage: &mut CS);
}