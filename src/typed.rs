mod factory;
mod id;
mod sync_factory;

pub use factory::IdFactory;
pub use id::Id;

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

    const _: IdFactory<MyUnit> = IdFactory::new();
}
