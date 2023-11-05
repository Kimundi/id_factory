use super::Id;
use std::{fmt::Debug, marker::PhantomData};

#[derive(Debug)]
pub struct SyncIdFactory<T> {
    factory: crate::untyped::SyncIdFactory,
    _marker: PhantomData<T>,
}

impl<T> Default for SyncIdFactory<T> {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl<T> SyncIdFactory<T> {
    #[inline]
    pub const fn new() -> Self {
        Self {
            factory: crate::untyped::SyncIdFactory::new(),
            _marker: PhantomData,
        }
    }

    #[inline]
    pub fn next_id(&self) -> Id<T> {
        self.factory.next_id_typed()
    }
}
