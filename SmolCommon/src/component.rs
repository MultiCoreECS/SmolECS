use super::entity::*;

pub trait ComponentStorage<T: Component>{
    fn get<'cs>(&'cs self, entity: &usize) -> Option<&'cs T>;

    fn get_mut<'cs>(&'cs mut self, entity: &usize) -> Option<&'cs mut T>;

    fn iter<'cs>(&'cs self) -> Box<(dyn Iterator<Item = (bool, Option<&'cs T>)> + 'cs)>;

    fn iter_mut<'cs>(&'cs mut self) -> Box<(dyn Iterator<Item = (bool, Option<&'cs mut T>)> + 'cs)>;
    
    fn set(&mut self, entity: &usize, comp: T);

    fn delete(&mut self, entity: &usize);

}

pub trait Component: Sized + Copy + Clone + Send + Sync{}

impl<T> Component for T
    where T: Sized + Copy + Clone + Send + Sync{}