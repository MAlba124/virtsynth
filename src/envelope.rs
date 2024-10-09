use std::sync::{atomic::Ordering, Arc};

use crate::atomicf::AtomicF32;

#[allow(clippy::upper_case_acronyms)]
pub struct ADSR {
    pub attack: f32,
    pub decay: f32,
    pub sustain: f32,
    pub release: f32,
    attack_a: Arc<AtomicF32>,
    decay_a: Arc<AtomicF32>,
    sustain_a: Arc<AtomicF32>,
    release_a: Arc<AtomicF32>,
}

impl ADSR {
    pub fn new(
        attack_a: Arc<AtomicF32>,
        decay_a: Arc<AtomicF32>,
        sustain_a: Arc<AtomicF32>,
        release_a: Arc<AtomicF32>,
    ) -> Self {
        Self {
            attack: 0.0,
            decay: 0.0,
            sustain: 1.0,
            release: 0.0,
            attack_a,
            decay_a,
            sustain_a,
            release_a,
        }
    }

    #[inline(always)]
    pub fn update(&mut self) {
        self.attack = self.attack_a.load(Ordering::Acquire);
        self.decay = self.decay_a.load(Ordering::Acquire);
        self.sustain = self.sustain_a.load(Ordering::Acquire);
        self.release = self.release_a.load(Ordering::Acquire);
    }
}
