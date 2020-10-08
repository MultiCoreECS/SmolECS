use crate::world::World;
use SmolCommon::system::*;
use SmolCommon::component::Component;
use SmolCommon::{DepVec, BitVec};
use SmolCommon::{WorldCommon};
use std::sync::{Arc, Mutex, atomic::{Ordering, AtomicBool}};
use std::collections::HashMap;
use rayon;

// Stores systems as a tuple of dependencies, init funcs, and run funcs
pub struct SystemScheduler<'w>{
    systems: HashMap<String, StoredSys<'w>>,
    max_threads: usize,
}

struct StoredSys<'w>{
    dep: Vec<String>,
    get_dep_vec: Box<dyn Fn(&'w World) -> DepVec>,
    run: Run<'w>,
}

struct Run<'w>{
    run: Box<dyn Fn(&'w World)>,
}

impl<'w> Run<'w>{
    fn run(&self, world: &'w World){
        self.run(world)
    }
}

unsafe impl<'w> Send for Run<'w>{}
unsafe impl<'w> Sync for Run<'w>{}

impl<'w> SystemScheduler<'w>{
    fn new(max_threads: usize) -> Self{
        SystemScheduler{
            systems: HashMap::new(),
            max_threads
        }
    }
}

impl<'w> Scheduler<'w, World> for SystemScheduler<'w>{

    fn add<S: System<'w>>(&mut self, name: String, dep: Vec<String>){
        self.systems.insert(name, 
            StoredSys{
                dep,
                get_dep_vec: Box::new(|world: &'w World| {S::get_system_dependencies(world)}),
                run: Run{run: Box::new(|world: &'w World| {S::run(S::get_system_data(world))})},
            });
    }

    fn run(&mut self, world: &'w World){

        let mut systems_done: HashMap<String, AtomicBool> = self.systems.iter().map(|(key, value)| (key.clone(), AtomicBool::from(false))).collect();
        let mut dep_vecs: HashMap<String, DepVec> = self.systems.iter_mut().map(|(key, value)| (key.clone(), (value.get_dep_vec)(world))).collect();
        let mut in_use_resources: Arc<Mutex<Vec<DepVec>>> = Arc::new(Mutex::new(Vec::new()));
        
        let mut all_systems_done = false;

        while !all_systems_done{
            
        //Check if all systems are complete
        for (sys, done) in systems_done.iter(){
            if done.load(Ordering::Relaxed){
                continue;
            }

            //When incomplete system found, check if it's dependencies are complete
            let sys_dep = &self.systems.get(sys).unwrap().dep;
            match sys_dep.iter().find(|dependency| systems_done.get(*dependency).unwrap().load(Ordering::Relaxed) == false){
                
                //If dependencies aren't complete, continue checking systems
                Some(dependency) => continue,
                _ => (),
            };
            
            //If dependencies are complete, check if it's resources are available
            let sys_res = dep_vecs.get(sys).unwrap();
            
            //If they aren't, continue checking systems
            if in_use_resources.lock().unwrap().iter().find(|dep| dep.intersection(sys_res.res_write.clone(), sys_res.comp_write.clone())).is_some(){
                continue;
            }
            
            in_use_resources.lock().unwrap().push(sys_res.clone());

            let in_use_clone = in_use_resources.clone();
            
            let run = self.systems.get(sys).unwrap().run;
            rayon::spawn(||{
                run.run(world)
            });

        }

        



        //If there is an open thread, send the system to it

        //If there isn't, wait for an open thread
        }
        todo!();
    }
}

#[cfg(test)]
mod tests{
    use crate::world::World;
    use SmolCommon::WorldCommon;
    use SmolCommon::system::*;
    use SmolCommon::join::Joinable;
    use std::convert::TryInto;

    #[test]
    fn read(){
        let mut world = World::new();

        world.insert(String::from("Hello, my name is James!"));

        let reader = Read::<String>::get_data(&world);

        assert_eq!(String::from("Hello, my name is James!"), *reader);
    }

    #[test]
    fn write(){
        let mut world = World::new();

        world.insert::<usize>(20);

        let mut writer = Write::<usize>::get_data(&world);

        *writer *= 2;

        assert_eq!(40, *writer);
    }

    #[test]
    fn read_comp(){
        let mut world = World::new();

        world.register_comp::<usize>();

        for i in 0..10{
            world.get_comp_mut::<usize>().set(&i, i);
        }

        let reader = ReadComp::<usize>::get_data(&world);

        for i in 0..10{
            assert_eq!(*reader.get(&i).unwrap(), i);
        }

        drop(reader);
        
        world.get_comp_mut::<usize>().delete(&2);
        world.get_comp_mut::<usize>().delete(&8);

        let reader = ReadComp::<usize>::get_data(&world);

        for i in 0..10{
            if i == 2 || i == 8{
                continue;
            }
            assert_eq!(*reader.get(&i).unwrap(), i);
        }

        let mut check: Vec<usize> = (0..10).collect();
        check.remove(8);
        check.remove(2);
        
        for (l, r) in check.iter().zip(reader.join()){
            assert_eq!(l, r);
        }
    }

    #[test]
    fn read_read_comp(){
        let mut world = World::new();

        world.register_comp::<usize>();
        world.register_comp::<isize>();

        for i in 0..10{
            world.get_comp_mut::<usize>().set(&i, i);
            world.get_comp_mut::<isize>().set(&i, -(i as isize));
        }

        let reader_usize = ReadComp::<usize>::get_data(&world);
        let reader_isize = ReadComp::<isize>::get_data(&world);

        for (u, i) in (&reader_usize, &reader_isize).join(){
            assert_eq!(*u, (-i).try_into().unwrap());
        }
    }


    #[test]
    fn read_read_read_comp(){
        let mut world = World::new();

        world.register_comp::<usize>();
        world.register_comp::<isize>();
        world.register_comp::<u8>();

        for i in 0..10{
            world.get_comp_mut::<usize>().set(&i, i);
            world.get_comp_mut::<isize>().set(&i, -(i as isize));
            world.get_comp_mut::<u8>().set(&i, i as u8);
        }

        let reader_usize = ReadComp::<usize>::get_data(&world);
        let reader_isize = ReadComp::<isize>::get_data(&world);
        let reader_u8 = ReadComp::<u8>::get_data(&world);

        for (u, i, smol_u) in (&reader_usize, &reader_isize, &reader_u8).join(){
            assert_eq!(*u, (-i).try_into().unwrap());
            assert_eq!(*u, *smol_u as usize);
        }
    }

    #[test]
    fn write_comp(){
        let mut world = World::new();

        world.register_comp::<usize>();

        for i in 0..10{
            world.get_comp_mut::<usize>().set(&i, i);
        }

        let reader = ReadComp::<usize>::get_data(&world);

        for i in 0..10{
            assert_eq!(*reader.get(&i).unwrap(), i);
        }

        drop(reader);
        
        world.get_comp_mut::<usize>().delete(&2);
        world.get_comp_mut::<usize>().delete(&8);

        let reader = ReadComp::<usize>::get_data(&world);

        for i in 0..10{
            if i == 2 || i == 8{
                continue;
            }
            assert_eq!(*reader.get(&i).unwrap(), i);
        }

        let mut check: Vec<usize> = (0..10).collect();
        check.remove(8);
        check.remove(2);
        
        for (l, r) in check.iter().zip(reader.join()){
            assert_eq!(l, r);
        }
    }

    #[test]
    fn multiple_read_write(){
        
        let mut world = World::new();

        world.insert(String::from("Hello, my name is James!"));
        world.insert::<usize>(20);

        let reader = Read::<String>::get_data(&world);
        let mut writer = Write::<usize>::get_data(&world);

        *writer *= 2;
        assert_eq!(String::from("Hello, my name is James!"), *reader);
        assert_eq!(40, *writer);
    }
}