use super::component::*;

pub trait EntityCommon{
    fn add_component<C: Component, CS: ComponentStorage<C>>(&mut self, comp: C, comp_storage: &mut CS);
}