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
    pub fn new() -> Self {
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

pub struct Id<T>(pub(crate) crate::untyped::Id, pub(crate) PhantomData<T>);

impl<T> Copy for Id<T> {}

impl<T> Clone for Id<T> {
    #[inline]
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Hash for Id<T> {
    #[inline]
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state)
    }
}

impl<T> PartialEq for Id<T> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<T> PartialOrd for Id<T> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl<T> Eq for Id<T> {}

impl<T> Ord for Id<T> {
    #[inline]
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}

impl<T> Debug for Id<T> {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl<T> Id<T> {
    #[inline]
    pub fn raw_value(self) -> u64 {
        self.0.raw_value()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MyUnit;

    #[test]
    fn test_basic() {
        let mut factory = IdFactory::<MyUnit>::new();
        let id1 = factory.next_id();
        let id2 = factory.next_id();
        assert_eq!(id1.raw_value(), 1);
        assert_eq!(id2.raw_value(), 2);
    }
}
