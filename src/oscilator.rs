use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

use crate::{
    atomicf::{AtomicF32, AtomicWaveform},
    synthesizer::TWO_PI,
    waveform::Waveform,
};

pub struct Oscilator {
    counter: f32,
    amplitude: f32,
    pub waveform: Waveform,
    pub frequency: f32,
    active: bool,
    frequency_a: Arc<AtomicF32>,
    waveform_a: Arc<AtomicWaveform>,
    active_a: Arc<AtomicBool>,
    scale: f32,
    scale_a: Arc<AtomicF32>,
}

impl Oscilator {
    pub fn new(
        frequency_a: Arc<AtomicF32>,
        waveform_a: Arc<AtomicWaveform>,
        active_a: Arc<AtomicBool>,
        scale_a: Arc<AtomicF32>,
    ) -> Self {
        Self {
            counter: 0.0,
            amplitude: 1.0,
            waveform: Waveform::Sin,
            frequency: 0.0,
            active: false,
            frequency_a,
            waveform_a,
            active_a,
            scale: 1.0,
            scale_a,
        }
    }

    #[inline(always)]
    pub fn update(&mut self) {
        self.frequency = self.frequency_a.load(Ordering::Acquire);
        self.waveform = self.waveform_a.load(Ordering::Acquire);
        self.active = self.active_a.load(Ordering::Acquire);
        self.scale = self.scale_a.load(Ordering::Acquire);
    }

    #[inline(always)]
    pub fn tick(&mut self, sample_rate: f32) -> f32 {
        if !self.active {
            return 1.0;
        }

        match self.waveform {
            Waveform::Sin => {
                self.amplitude += TWO_PI * self.frequency / sample_rate;
                if self.amplitude > TWO_PI {
                    self.amplitude -= TWO_PI;
                }
                self.amplitude.sin() * self.scale
            }
            Waveform::Square => {
                self.counter += 1.0;

                let period_samples = sample_rate / self.frequency;

                if self.counter < period_samples / 2.0 {
                    return 1.0 * self.scale;
                } else if self.counter < period_samples {
                    return -1.0 * self.scale;
                }

                self.counter = 0.0;
                1.0 * self.scale
            }
        }
    }
}
