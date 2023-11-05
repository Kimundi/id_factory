use std::{marker::PhantomData, num::NonZeroU64};

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
    pub const fn new() -> Self {
        Self {
            next_id: NonZeroU64::MIN,
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

#[derive(Copy, Clone, Hash, PartialEq, PartialOrd, Eq, Ord, Debug)]
pub struct Id(pub(crate) NonZeroU64);

impl Id {
    #[inline]
    pub const fn raw_value(self) -> u64 {
        self.0.get()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        let mut factory = IdFactory::new();
        let id1 = factory.next_id();
        let id2 = factory.next_id();
        assert_eq!(id1.raw_value(), 1);
        assert_eq!(id2.raw_value(), 2);
    }

    const _: IdFactory = IdFactory::new();
}
