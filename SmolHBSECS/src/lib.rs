use SmolCommon::component::*;
use SmolCommon::entity::*;
use bit_vec::BitVec;
use std::iter::FilterMap;

struct Entity{
    index: usize,
    generation: usize,
}

impl EntityCommon for Entity{
    fn add_component<'a, C: Component, CS: ComponentStorage<C>>(&mut self, comp: C, comp_storage: &mut CS){
        todo!()
    }
}

/// Stores components as a normal vector
struct VecStorage<T>{
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
    type Entity = Entity;

    /// Gets a reference to a component at the given index (entity)
    fn get<'cs>(&'cs self, entity: &Self::Entity) -> Option<&'cs T>{
        todo!()
    }

    /// Gets a mutable reference to a component at the given index (entity)
    fn get_mut<'cs>(&'cs mut self, entity: &Self::Entity) -> Option<&'cs mut T>{
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
    fn set<'cs>(&'cs mut self, entity: &Self::Entity, comp: T){
        // This is bad, but should almost never happen
        while entity.index >= self.storage.len(){
            self.storage.push(None);
            self.valid.push(false);
        }
        *self.storage.get_mut(entity.index).unwrap() = Some(comp);
        self.valid.set(entity.index, true);
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
            storage.set(&e, i);
        }

        for (n, i) in storage.iter().enumerate(){
            assert_eq!(n, *i);
        }
        println!("Pee pee poo poo");
    }
}