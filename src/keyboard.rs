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

use std::sync::{
    atomic::{AtomicBool, AtomicUsize, Ordering},
    Arc,
};

use crate::{
    atomicf::{AtomicF32, AtomicWaveform},
    synthesizer::Synthesizer,
    waveform::Waveform,
};

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum Key {
    C4 = 40,
    CS = 41,
    D4 = 42,
    DS = 43,
    E4 = 44,
    F4 = 45,
    FS = 46,
    G4 = 47,
    GS = 48,
    A4 = 49,
    AS = 50,
    B4 = 51,
}

impl Key {
    pub fn freq(self) -> f32 {
        2.0f32.powf((self as i32 - 49) as f32 / 12.0) * 440.0
    }

    pub fn bitflag(self) -> usize {
        match self {
            Key::C4 => 0b000000000001,
            Key::CS => 0b000000000010,
            Key::D4 => 0b000000000100,
            Key::DS => 0b000000001000,
            Key::E4 => 0b000000010000,
            Key::F4 => 0b000000100000,
            Key::FS => 0b000001000000,
            Key::G4 => 0b000010000000,
            Key::GS => 0b000100000000,
            Key::A4 => 0b001000000000,
            Key::AS => 0b010000000000,
            Key::B4 => 0b100000000000,
        }
    }

    fn from_bitflag(bitflag: usize) -> Self {
        match bitflag {
            0b000000000001 => Key::C4,
            0b000000000010 => Key::CS,
            0b000000000100 => Key::D4,
            0b000000001000 => Key::DS,
            0b000000010000 => Key::E4,
            0b000000100000 => Key::F4,
            0b000001000000 => Key::FS,
            0b000010000000 => Key::G4,
            0b000100000000 => Key::GS,
            0b001000000000 => Key::A4,
            0b010000000000 => Key::AS,
            0b100000000000 => Key::B4,
            _ => unreachable!(),
        }
    }

    pub fn from_zero_index(index: usize) -> Self {
        match index {
            0 => Key::C4,
            1 => Key::CS,
            2 => Key::D4,
            3 => Key::DS,
            4 => Key::E4,
            5 => Key::F4,
            6 => Key::FS,
            7 => Key::G4,
            8 => Key::GS,
            9 => Key::A4,
            10 => Key::AS,
            11 => Key::B4,
            _ => unreachable!(),
        }
    }
}

#[derive(Copy, Clone)]
pub struct KeyBitflags(pub usize, pub usize);

impl KeyBitflags {
    pub fn all() -> Self {
        Self(
            Key::C4.bitflag()
                | Key::CS.bitflag()
                | Key::D4.bitflag()
                | Key::DS.bitflag()
                | Key::E4.bitflag()
                | Key::F4.bitflag()
                | Key::FS.bitflag()
                | Key::G4.bitflag()
                | Key::GS.bitflag()
                | Key::A4.bitflag()
                | Key::AS.bitflag()
                | Key::B4.bitflag(),
            1,
        )
    }
}

impl Iterator for KeyBitflags {
    type Item = Key;

    fn next(&mut self) -> Option<Self::Item> {
        if self.1 > Key::B4.bitflag() {
            return None;
        }

        while self.1 <= Key::B4.bitflag() {
            if (self.0 & self.1) > 0 {
                let key = Key::from_bitflag(self.1);
                self.1 <<= 1;

                return Some(key);
            }
            self.1 <<= 1;
        }

        None
    }
}

pub struct Osc {
    pub active: Arc<AtomicBool>,
    pub waveform: Arc<AtomicWaveform>,
    pub gain: Arc<AtomicF32>,
}

impl Osc {
    pub fn new(active: bool, waveform: Waveform, gain: f32) -> (Self, Self) {
        let active = Arc::new(AtomicBool::new(active));
        let waveform = Arc::new(AtomicWaveform::new(waveform));
        let gain = Arc::new(AtomicF32::new(gain));

        (
            Self {
                active: Arc::clone(&active),
                waveform: Arc::clone(&waveform),
                gain: Arc::clone(&gain),
            },
            Self {
                active,
                waveform,
                gain,
            },
        )
    }
}

pub struct Keyboard {
    pub gain: Arc<AtomicF32>,
    active_keys: Arc<AtomicUsize>,
    pub attack: Arc<AtomicF32>,
    pub decay: Arc<AtomicF32>,
    pub sustain: Arc<AtomicF32>,
    pub release: Arc<AtomicF32>,
    _synth: Synthesizer,
    pub osc1: Osc,
    pub osc2: Osc,
    pub osc3: Osc,
    // pub osc_active: Arc<AtomicBool>,
    // pub osc_waveform: Arc<AtomicWaveform>,
    // pub osc_scale: Arc<AtomicF32>,
}

impl Keyboard {
    pub fn new() -> Self {
        let active_keys = Arc::new(AtomicUsize::new(0));
        let active_keys_clone = Arc::clone(&active_keys);

        let gain = Arc::new(AtomicF32::new(0.5));
        let gain_clone = Arc::clone(&gain);

        let attack = Arc::new(AtomicF32::new(0.1));
        let attack_clone = Arc::clone(&attack);

        let decay = Arc::new(AtomicF32::new(0.0));
        let decay_clone = Arc::clone(&decay);

        let sustain = Arc::new(AtomicF32::new(1.0));
        let sustain_clone = Arc::clone(&sustain);

        let release = Arc::new(AtomicF32::new(0.1));
        let release_clone = Arc::clone(&release);

        // TODO: Make arcs or something
        let (osc1_clone, osc1) = Osc::new(true, Waveform::Sin, 1.0);
        let (osc2_clone, osc2) = Osc::new(false, Waveform::Sin, 1.0);
        let (osc3_clone, osc3) = Osc::new(false, Waveform::Sin, 1.0);

        // let osc_active = Arc::new(AtomicBool::new(false));
        // let osc_active_clone = Arc::clone(&osc_active);

        // let osc_waveform = Arc::new(AtomicWaveform::new(Waveform::Sin));
        // let osc_waveform_clone = Arc::clone(&osc_waveform);

        // let osc_scale = Arc::new(AtomicF32::new(1.0));
        // let osc_scale_clone = Arc::clone(&osc_scale);

        let synth = Synthesizer::new(
            gain_clone,
            active_keys_clone,
            attack_clone,
            decay_clone,
            sustain_clone,
            release_clone,
            osc1_clone,
            osc2_clone,
            osc3_clone,
            // osc_active_clone,
            // osc_waveform_clone,
            // osc_scale_clone,
        );

        Self {
            active_keys,
            gain,
            _synth: synth,
            attack,
            decay,
            sustain,
            release,
            osc1,
            osc2,
            osc3,
            // osc_active,
            // osc_waveform,
            // osc_scale,
        }
    }

    #[inline(always)]
    pub fn set_active_keys(&mut self, active_keys: usize) {
        self.active_keys.store(active_keys, Ordering::Release);
    }
}
