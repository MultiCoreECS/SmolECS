use super::component::*;
use crate::system::WriteComp;

pub trait EntityCommon: PartialEq + Eq{
    fn add<'e, 'd: 'e, T: Component>(&'e self, storage: &'e mut WriteComp<'d, T>, comp: T) -> &'e Self;
    fn remove<'e, 'd: 'e, T: Component>(&'e self, storage: &'e mut WriteComp<'d, T>) -> &'e Self;
}