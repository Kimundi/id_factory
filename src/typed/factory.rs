use super::Id;
use std::{fmt::Debug, hash::Hash, marker::PhantomData};

#[derive(Clone, Hash, PartialEq, PartialOrd, Eq, Ord, Debug)]
pub struct IdFactory<T> {
    factory: crate::untyped::IdFactory,
    _marker: PhantomData<T>,
}

impl<T> Default for IdFactory<T> {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl<T> IdFactory<T> {
    #[inline]
    pub const fn new() -> Self {
        Self {
            factory: crate::untyped::IdFactory::new(),
            _marker: PhantomData,
        }
    }

    #[inline]
    pub fn next_id(&mut self) -> Id<T> {
        self.factory.next_id_typed()
    }
}
