pub mod world{
    pub use SmolCommon::WorldCommon;
    pub use SmolHBSECS::world::World;
}

pub mod entity{
    pub use SmolCommon::entity::EntityCommon;
    pub use SmolHBSECS::{Entity, EntityStorage};

}

pub mod component{
    pub use SmolCommon::component::{Component, ComponentStorage};
    pub use SmolHBSECS::component::VecStorage;
}

pub mod system{
    pub use SmolCommon::system::{ReadComp, WriteComp, Read, Write, System, SystemData, Scheduler};
    pub use SmolCommon::join::Joinable;
    pub use SmolHBSECS::system::SystemScheduler;
}

pub use rayon;

#[cfg(test)]
mod tests{
    use crate::world::*;
    use crate::system::*;
    use crate::entity::*;
    use crate::rayon;
    use std::sync::Arc;

    struct AddOne;

    impl<'d, 'w: 'd> System<'d, 'w, World> for AddOne{
        type SystemData = (
            Write<'d, EntityStorage>,
            WriteComp<'d, usize>,
            WriteComp<'d, isize>
        );

        fn run(&self, (mut es, mut us, mut is): Self::SystemData) { 
            let ent = es.create_entity();
            ent.add(&mut us, 0);
            ent.add(&mut is, 0);
            for (u, i) in (&mut us, &mut is).join(){
                *u += 1;
                *i += 1;
            }
        }
    }

    struct SubOne;

    impl<'d, 'w: 'd> System<'d, 'w, World> for SubOne{
        type SystemData = 
            WriteComp<'d, isize>
        ;

        fn run(&self, mut is: Self::SystemData) { 
            for i in (&mut is).join(){
                *i -= 1;
            }
        }
    }

    struct CounterCheck;

    impl<'d, 'w: 'd> System<'d, 'w, World> for CounterCheck{
        type SystemData = (
            ReadComp<'d, usize>,
            ReadComp<'d, isize>,
            Write<'d, usize>
        );

        fn run(&self, (us, is, mut counter): Self::SystemData) { 
            for (u, i) in (&us, &is).join(){
                *counter = std::cmp::max(*u, *counter);
            }
        }
    }

    struct SubCheck;

    impl<'d, 'w: 'd> System<'d, 'w, World> for SubCheck{
        type SystemData = (
            ReadComp<'d, isize>,
            Write<'d, isize>
        );

        fn run(&self, (is, mut counter): Self::SystemData) { 
            for (i) in (&is).join(){
                *counter = *i;
            }
        }
    }

    #[test]
    fn run_ten_times(){
        let mut world = World::new();

        world.register_comp::<usize>();
        world.register_comp::<isize>();

        world.insert(0 as isize);
        world.insert(0 as usize);
        world.insert(EntityStorage::new());

        let pool = Arc::new(rayon::ThreadPoolBuilder::new().num_threads(8).build().unwrap());
        let mut schedule = SystemScheduler::new(pool);

        schedule.add(AddOne{}, "AddOne", vec![]);
        schedule.add(SubOne{}, "SubOne", vec!["AddOne"]);
        schedule.add(CounterCheck{}, "CounterCheck", vec!["SubOne"]);
        schedule.add(SubCheck{}, "SubCheck", vec!["SubOne"]);

        for i in 1..1000{
            schedule.run(&world);
            let counter_reader = Read::<usize>::get_data(&world);
            let sub_reader = Read::<isize>::get_data(&world);
            
            assert_eq!(*counter_reader, i);
            assert_eq!(*sub_reader, 0);
        }
    }
}