use std::{marker::PhantomData, num::NonZeroU64};

#[derive(Copy, Clone, Hash, PartialEq, PartialOrd, Eq, Ord, Debug)]
pub struct Id(pub(crate) NonZeroU64);

#[derive(Clone, Hash, PartialEq, PartialOrd, Eq, Ord, Debug)]
pub struct IdFactory {
    next_id: NonZeroU64,
}

impl Default for IdFactory {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl IdFactory {
    #[inline]
    pub fn new() -> Self {
        Self {
            next_id: NonZeroU64::new(1).unwrap(),
        }
    }
    #[inline]
    pub fn next_id(&mut self) -> Id {
        let id = self.next_id;
        // NB: As long as we only do +1 steps, we
        // have a lower bound of 150 years until we overflow here :)
        self.next_id = NonZeroU64::new(u64::from(id) + 1).unwrap();
        Id(id)
    }
    #[inline]
    pub fn next_id_typed<T>(&mut self) -> crate::typed::Id<T> {
        let id = self.next_id();
        crate::typed::Id(id, PhantomData)
    }
}
