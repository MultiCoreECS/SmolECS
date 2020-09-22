use SmolCommon::component::*;
use super::Entity;
use bit_vec::BitVec;
use std::iter::FilterMap;

/// Stores components as a normal vector
pub struct VecStorage<T>{
    storage: Vec<Option<T>>,
    valid: BitVec,
}

impl<T> VecStorage<T>{
    pub fn new() -> Self{
        VecStorage{
            storage: Vec::new(),
            valid: BitVec::new(),
        }
    }
}

impl<T: Component> ComponentStorage<T> for VecStorage<T>{

    /// Gets a reference to a component at the given index (entity)
    fn get<'cs>(&'cs self, entity: &usize) -> Option<&'cs T>{
        todo!()
    }

    /// Gets a mutable reference to a component at the given index (entity)
    fn get_mut<'cs>(&'cs mut self, entity: &usize) -> Option<&'cs mut T>{
        todo!()
    }

    /// Iterates over the valid components.
    fn iter<'cs>(&'cs self) -> Box<(dyn Iterator<Item = &'cs T> + 'cs)>{
        Box::new(
            self.storage.iter()
                .zip(self.valid.iter())
                .filter_map(|(comp, v)| if v {Some(comp.as_ref().unwrap())} else {None}))
    }

    /// Mutabley iterates over the valid components.
    fn iter_mut<'cs>(&'cs mut self) -> Box<(dyn Iterator<Item = &'cs mut T> + 'cs)>{
        Box::new(
            self.storage.iter_mut()
                .zip(self.valid.iter())
                .filter_map(|(comp, v)| if v {Some(comp.as_mut().unwrap())} else {None}))
    }

    /// Puts a component at the given index, can also append new components
    fn set<'cs>(&'cs mut self, entity: &usize, comp: T){
        // This is bad, but should almost never happen
        while *entity >= self.storage.len(){
            self.storage.push(None);
            self.valid.push(false);
        }
        *self.storage.get_mut(*entity).unwrap() = Some(comp);
        self.valid.set(*entity, true);
    }

    fn delete<'cs>(&'cs mut self, entity: &usize){
        if *entity < self.storage.len(){
            *self.storage.get_mut(*entity).unwrap() = None;
            self.valid.set(*entity, false);
        }
    }
}

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn create_insert_iter(){
        let mut storage = VecStorage::new();

        for i in 0..10{
            let e = Entity{index: i, generation: 0};
            storage.set(&e.index, i);
        }

        for (n, i) in storage.iter().enumerate(){
            assert_eq!(n, *i);
        }
    }

    #[test]
    fn create_insert_delete_iter(){
        let mut storage = VecStorage::new();

        for i in 0..10{
            let e = Entity{index: i, generation: 0};
            storage.set(&e.index, i);
        }

        storage.delete(&Entity{index: 0, generation: 0}.index);

        for (n, i) in storage.iter_mut().enumerate(){
            assert_eq!(n + 1, *i);
            *i *= 2;
        }

        for (n, i) in storage.iter().enumerate(){
            assert_eq!((n + 1)* 2, *i);
        }
    }
}