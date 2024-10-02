use std::sync::{
    atomic::{AtomicU16, AtomicUsize, Ordering},
    Arc,
};

use crate::synthesizer::Synthesizer;

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

pub struct Keyboard {
    gain: Arc<AtomicU16>,
    active_keys: Arc<AtomicUsize>,
    attack: Arc<AtomicU16>,
    release: Arc<AtomicU16>,
    _synth: Synthesizer,
}

impl Keyboard {
    pub fn new() -> Self {
        let active_keys = Arc::new(AtomicUsize::new(0));
        let active_keys_clone = Arc::clone(&active_keys);

        let gain = Arc::new(AtomicU16::new(std::u16::MAX / 2));
        let gain_clone = Arc::clone(&gain);

        let attack = Arc::new(AtomicU16::new(std::u16::MAX / 2));
        let attack_clone = Arc::clone(&attack);

        let release = Arc::new(AtomicU16::new(std::u16::MAX / 2));
        let release_clone = Arc::clone(&release);

        let synth = Synthesizer::new(gain_clone, active_keys_clone, attack_clone, release_clone);

        Self {
            active_keys,
            gain,
            _synth: synth,
            attack,
            release,
        }
    }

    #[inline(always)]
    pub fn set_active_keys(&mut self, active_keys: usize) {
        self.active_keys.store(active_keys, Ordering::Relaxed);
    }

    #[inline(always)]
    pub fn gain(&self) -> f32 {
        self.gain.load(Ordering::Relaxed) as f32 / std::u16::MAX as f32
    }

    #[inline(always)]
    pub fn set_gain(&self, new_gain: f32) {
        self.gain
            .store((std::u16::MAX as f32 * new_gain) as u16, Ordering::Relaxed);
    }
}
