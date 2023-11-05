use super::Id;
use std::{
    marker::PhantomData,
    sync::atomic::{AtomicU64, Ordering},
};

#[derive(Debug)]
pub struct SyncIdFactory {
    next_id: AtomicU64,
}

impl Default for SyncIdFactory {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl SyncIdFactory {
    #[inline]
    pub const fn new() -> Self {
        Self {
            next_id: AtomicU64::new(1),
        }
    }

    #[inline]
    pub fn next_id(&self) -> Id {
        // NB: As long as we only do +1 steps, we
        // have a lower bound of 150 years until we overflow here :)
        let id = self.next_id.fetch_add(1, Ordering::SeqCst);
        Id(id.try_into().unwrap())
    }

    #[inline]
    pub fn next_id_typed<T>(&self) -> crate::typed::Id<T> {
        let id = self.next_id();
        crate::typed::Id(id, PhantomData)
    }
}
