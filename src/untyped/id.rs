use std::num::NonZeroU64;

#[derive(Copy, Clone, Hash, PartialEq, PartialOrd, Eq, Ord, Debug)]
pub struct Id(pub(crate) NonZeroU64);

impl Id {
    #[inline]
    pub const fn raw_value(self) -> u64 {
        self.0.get()
    }
}
