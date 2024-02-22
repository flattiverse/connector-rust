use std::sync::atomic::{AtomicU64, Ordering};

pub struct AtomicF64(AtomicU64);

impl From<f64> for AtomicF64 {
    #[inline]
    fn from(value: f64) -> Self {
        Self(AtomicU64::new(value.to_bits()))
    }
}

impl AtomicF64 {
    #[inline]
    pub fn store(&self, value: f64, ordering: Ordering) {
        self.0.store(value.to_bits(), ordering)
    }

    #[inline]
    pub fn load(&self, ordering: Ordering) -> f64 {
        f64::from_bits(self.0.load(ordering))
    }
}
