/*
 * Copyright (C) 2024 Marcus L. Hanestad  <marlhan@proton.me>
 *
 * VirtSynth is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * VirtSynth is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with VirtSynth .  If not, see <https://www.gnu.org/licenses/>.
 */

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
