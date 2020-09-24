use SmolCommon::system::*;
use SmolCommon::component::Component;

#[cfg(test)]
mod tests{
    use crate::world::World;
    use SmolCommon::WorldCommon;
    use SmolCommon::system::*;

    #[test]
    fn read(){
        let mut world = World::new();

        world.insert(String::from("Hello, my name is James!"));

        let reader = Read::<String>::get_data(&world);

        assert_eq!(String::from("Hello, my name is James!"), *reader);
    }

    fn write(){
        let mut world = World::new();

        world.insert::<usize>(20);

        let mut writer = Write::<usize>::get_data(&world);

        *writer *= 2;

        assert_eq!(40, *writer);
    }

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