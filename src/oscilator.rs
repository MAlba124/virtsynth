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

use std::{
    f32::consts::TAU,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
};

use crate::{
    atomicf::{AtomicF32, AtomicWaveform},
    waveform::Waveform,
};

pub struct Oscilator {
    pub waveform: Waveform,
    pub active: bool,
    waveform_a: Arc<AtomicWaveform>,
    active_a: Arc<AtomicBool>,
    pub gain: f32, // Gain?
    gain_a: Arc<AtomicF32>,
}

impl Oscilator {
    pub fn new(
        waveform_a: Arc<AtomicWaveform>,
        active_a: Arc<AtomicBool>,
        gain_a: Arc<AtomicF32>,
    ) -> Self {
        Self {
            waveform: Waveform::Sin,
            active: false,
            waveform_a,
            active_a,
            gain: 1.0,
            gain_a,
        }
    }

    #[inline(always)]
    pub fn update(&mut self) {
        self.waveform = self.waveform_a.load(Ordering::Acquire);
        self.active = self.active_a.load(Ordering::Acquire);
        self.gain = self.gain_a.load(Ordering::Acquire);
    }

    #[inline(always)]
    pub fn tick(&mut self, phase: f32) -> f32 {
        (match self.waveform {
            Waveform::Sin => (phase * TAU).sin(),
            Waveform::Square => {
                if phase > 0.5 {
                    -1.0
                } else {
                    1.0
                }
            }
            Waveform::Saw => 2.0 * phase - 1.0,
            Waveform::Triangle => 2.0 * (2.0 * phase - 1.0).abs() - 1.0,
        }) * self.gain
    }
}
