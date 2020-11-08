use super::component::*;
use crate::system::WriteCompCommon;
use std::any::Any;

pub trait EntityCommon: PartialEq + Eq{
    fn add<'e, T: Component, S: Any>(&'e self, storage: &'e mut S, comp: T) -> &'e Self;
    fn remove<'e, T: Component, S: Any>(&'e self, storage: &'e mut S) -> &'e Self;
}