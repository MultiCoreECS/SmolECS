use super::entity::*;

pub trait ComponentStorage<T: Component>{
    type Entity: EntityCommon;

    fn get<'cs>(&'cs self, entity: &Self::Entity) -> Option<&'cs T>;

    fn get_mut<'cs>(&'cs mut self, entity: &Self::Entity) -> Option<&'cs mut T>;

    fn iter<'cs>(&'cs self) -> Box<(dyn Iterator<Item = &'cs T> + 'cs)>;

    fn iter_mut<'cs>(&'cs mut self) -> Box<(dyn Iterator<Item = &'cs mut T> + 'cs)>;
    
    fn set<'cs>(&'cs mut self, entity: &Self::Entity, comp: T);
}

pub trait Component: Sized + Copy + Clone + Send + Sync{}

impl<T> Component for T
    where T: Sized + Copy + Clone + Send + Sync{}