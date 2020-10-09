use super::component::*;
use crate::system::WriteComp;

pub trait EntityCommon: PartialEq + Eq{
    fn add<'d, T: Component>(&'d self, storage: &'d mut WriteComp<'d, T>, comp: T);
    fn remove<'d, T: Component>(&'d self, storage: &'d mut WriteComp<'d, T>);
}