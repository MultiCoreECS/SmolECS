use super::entity::*;

pub trait ComponentStorage<'cs, T>{
    fn get<E: Entity>(&self, entity: E) -> Option<&'cs T>;

    fn get_mut<E: Entity>(&mut self, entity: E) -> Option<&'cs mut T>;

    fn iter(&self) -> dyn Iterator<Item = Option<&'cs T>>;

    fn iter_mut(&mut self) -> dyn Iterator<Item = Option<&'cs mut T>>;
}

pub trait Component{}