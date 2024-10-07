use crate::{synthesizer::TWO_PI, waveform::Waveform};

pub struct Oscilator {
    counter: f32,
    amplitude: f32,
    pub waveform: Waveform,
    pub frequency: f32,
}

impl Default for Oscilator {
    fn default() -> Self {
        Self {
            waveform: Waveform::Sin,
            counter: 0.0,
            amplitude: 1.0,
            frequency: 4.0,
        }
    }
}

impl Oscilator {
    #[inline(always)]
    pub fn tick(&mut self, sample_rate: f32) -> f32 {
        match self.waveform {
            Waveform::Sin => {
                self.amplitude += TWO_PI * self.frequency / sample_rate;
                if self.amplitude > TWO_PI {
                    self.amplitude -= TWO_PI;
                }
                self.amplitude.sin()
            }
            Waveform::Square => {
                self.counter += 1.0;

                let period_samples = sample_rate / self.frequency;

                if self.counter < period_samples / 2.0 {
                    return 1.0;
                } else if self.counter < period_samples {
                    return -1.0;
                }

                self.counter = 0.0;
                1.0
            }
        }
    }
}
