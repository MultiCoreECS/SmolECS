use crate::world::World;
use SmolCommon::system::*;
use SmolCommon::component::Component;
use SmolCommon::{DepVec, BitVec};
use SmolCommon::{WorldCommon};
use std::sync::{Arc, Mutex, atomic::{Ordering, AtomicBool}};
use std::collections::HashMap;
use rayon;
use std::ops::Deref;

// Stores systems as a tuple of dependencies, init funcs, and run funcs
pub struct SystemScheduler<'w>{
    systems: HashMap<String, StoredSys<'w>>,
}

struct StoredSys<'w>{
    dep: Vec<String>,
    get_dep_vec: Box<dyn Fn(&'w World) -> DepVec>,
    run: Arc<Run>,
}

struct Run{
    function: Box<dyn Fn(&'static World)>,
}

unsafe impl Send for Run{}
unsafe impl Sync for Run{}

impl<'w> SystemScheduler<'w>{
    fn new() -> Self{
        SystemScheduler{
            systems: HashMap::new()
        }
    }
}

impl<'w> Scheduler<'w, World> for SystemScheduler<'w>{

    fn add<S: System<'w>>(&mut self, name: String, dep: Vec<String>){
        self.systems.insert(name, 
            StoredSys{
                dep,
                get_dep_vec: Box::new(|world: &'w World| {S::get_system_dependencies(world)}),
                run: Arc::new(Run{function: Box::new(|world: &'static World| {S::run(S::get_system_data(world))})}),
            });
    }

    fn run(&mut self, world: &'static World){

        let systems_done: HashMap<String, Arc<AtomicBool>> = self.systems.iter().map(|(key, value)| (key.clone(), Arc::new(AtomicBool::from(false)))).collect();
        let dep_vecs: HashMap<String, DepVec> = self.systems.iter_mut().map(|(key, value)| (key.clone(), (value.get_dep_vec)(&world))).collect();
        let mut in_use_resources: Arc<Mutex<HashMap<String, DepVec>>> = Arc::new(Mutex::new(HashMap::new()));
        
        let mut all_systems_done = false;

        while !all_systems_done{

            let mut in_use_clone = None;
            let mut run_fn = None;
            let mut world_clone = None;
            let mut done_clone = None;
            let mut sys_clone = None;

            let mut systems_done_check = true;

            //Check if all systems are complete
            for (sys, done) in systems_done.iter(){
                if done.load(Ordering::Relaxed){
                    continue;
                }

                systems_done_check = false;

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
                if in_use_resources.lock().unwrap().iter().find(|(d_sys, dep)| dep.intersection(sys_res.res_write.clone(), sys_res.comp_write.clone())).is_some(){
                    continue;
                }
                
                in_use_resources.lock().unwrap().insert(sys.clone(), sys_res.clone());

                in_use_clone = Some(in_use_resources.clone());
                run_fn = Some(self.systems.get(sys).unwrap().run.clone());
                world_clone = Some(world.clone());
                done_clone = Some(done.clone());
                sys_clone = Some(sys.clone());
                break;
            }
            
            if in_use_clone.is_some() && run_fn.is_some() && world_clone.is_some() && done_clone.is_some() && sys_clone.is_some(){
                let in_use_clone = in_use_clone.unwrap();
                let run_fn = run_fn.unwrap();
                let world_clone = world_clone.unwrap();
                let done_clone = done_clone.unwrap();
                let sys_clone = sys_clone.unwrap();
    
                rayon::spawn_fifo(move ||{
                    (run_fn.function)(&world_clone);
                    in_use_clone.lock().unwrap().remove(&sys_clone);
                    done_clone.store(true, Ordering::Relaxed);
                });
            }

            all_systems_done = systems_done_check;
        }
    }
}

#[cfg(test)]
mod tests{
    use crate::world::World;
    use crate::system::SystemScheduler;
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

    struct TimesTwo;

    impl<'w> System<'w> for TimesTwo{
        type SystemData = WriteComp<'w, usize>;

        fn run(mut data: Self::SystemData){
            for num in data.join(){
            }
        }
    }

    #[test]
    fn scheduler_basic_test(){
        let mut world = World::new();

        world.register_comp::<usize>();

        for i in 0..10{
            world.get_comp_mut::<usize>().set(&i, i);
        }

        let mut scheduler = SystemScheduler::new();
        scheduler.add::<TimesTwo>("times_two".to_string(), Vec::new());

        scheduler.run(&world);
    }
}