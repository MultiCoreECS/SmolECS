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
    pub use SmolHBSECS::system::SystemScheduler;
}