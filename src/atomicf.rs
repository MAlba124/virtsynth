use std::sync::atomic::{AtomicI32, AtomicU32, Ordering};

use crate::waveform::Waveform;

pub struct AtomicF32 {
    inner: AtomicU32,
}

impl AtomicF32 {
    pub fn new(v: f32) -> Self {
        Self {
            inner: AtomicU32::new(v.to_bits()),
        }
    }

    #[inline(always)]
    pub fn load(&self, order: Ordering) -> f32 {
        f32::from_bits(self.inner.load(order))
    }

    #[inline(always)]
    pub fn store(&self, val: f32, order: Ordering) {
        self.inner.store(val.to_bits(), order)
    }
}

pub struct AtomicWaveform {
    inner: AtomicI32,
}

impl AtomicWaveform {
    pub fn new(v: Waveform) -> Self {
        Self {
            inner: AtomicI32::new(v as i32),
        }
    }

    #[inline(always)]
    pub fn load(&self, order: Ordering) -> Waveform {
        Waveform::from(self.inner.load(order))
    }

    #[inline(always)]
    pub fn store(&self, val: Waveform, order: Ordering) {
        self.inner.store(val as i32, order)
    }
}
