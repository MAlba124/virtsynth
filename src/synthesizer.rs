use std::sync::{
    atomic::{AtomicU16, AtomicUsize, Ordering},
    Arc,
};

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

use crate::keyboard::Key;

const TWO_PI: f32 = 2.0 * std::f32::consts::PI;

struct PhaseStore {
    phases: [f32; 12],
}

impl PhaseStore {
    pub fn new() -> Self {
        Self { phases: [0.0; 12] }
    }

    #[inline(always)]
    fn get_phase_index(&self, key: Key) -> usize {
        match key {
            Key::C4 => 0,
            Key::CS => 1,
            Key::D4 => 2,
            Key::DS => 3,
            Key::E4 => 4,
            Key::F4 => 5,
            Key::FS => 6,
            Key::G4 => 7,
            Key::GS => 8,
            Key::A4 => 9,
            Key::AS => 10,
            Key::B4 => 11,
        }
    }

    #[inline(always)]
    pub fn get_phase(&mut self, key: Key) -> &mut f32 {
        &mut self.phases[self.get_phase_index(key)]
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
enum KeyState {
    Pressed,
    Released,
}

struct ADSR {
    pub attack: f32,
    pub _decay: f32,
    pub _sustain: f32,
    pub release: f32,
}

#[derive(Clone, Copy)]
struct TrackElement {
    pub state: KeyState,
    pub amplitude: f32,
    pub position: f32,
    pub t_amplitude: f32,
}

impl TrackElement {
    pub fn press(&mut self) {
        if self.state != KeyState::Pressed {
            self.state = KeyState::Pressed;
            self.position = 0.0;
            self.t_amplitude = self.amplitude;
        }
    }

    pub fn release(&mut self) {
        if self.state != KeyState::Released {
            self.state = KeyState::Released;
            self.position = 0.0;
            self.t_amplitude = self.amplitude;
        }
    }

    pub fn tick(&mut self, sample_rate: f32, adsr: &ADSR) {
        match self.state {
            KeyState::Pressed => {
                self.position += 1.0;
                if self.position < (sample_rate * adsr.attack) {
                    self.amplitude += (1.0 - self.t_amplitude) / (sample_rate * adsr.attack);
                } else {
                    self.amplitude = 1.0;
                }
            }
            KeyState::Released => {
                self.position += 1.0;
                if self.position < (sample_rate * adsr.release) {
                    self.amplitude -= self.t_amplitude / (sample_rate * adsr.release);
                } else {
                    self.amplitude = 0.0;
                }
            }
        }
    }
}

impl Default for TrackElement {
    fn default() -> Self {
        Self {
            state: KeyState::Released,
            amplitude: 0.0,
            position: 0.0,
            t_amplitude: 0.0,
        }
    }
}

struct KeyAmplitudeTracker {
    sample_rate: f32,
    keys: [TrackElement; 12],
    pub adsr: ADSR,
}

impl KeyAmplitudeTracker {
    pub fn new(sample_rate: f32) -> Self {
        Self {
            sample_rate,
            keys: [TrackElement::default(); 12],
            adsr: ADSR {
                attack: 0.4,
                _decay: 0.0,
                _sustain: 0.0,
                release: 0.2,
            },
        }
    }

    #[inline(always)]
    pub fn update(&mut self, keys: usize) {
        let mut mask = 0b1;
        for i in 0..12 {
            if (keys & mask) > 0 {
                self.keys[i].press();
            } else {
                self.keys[i].release();
            }
            mask <<= 1;
        }
    }

    #[inline(always)]
    pub fn tick(&mut self) -> &[TrackElement; 12] {
        for k in self.keys.iter_mut() {
            k.tick(self.sample_rate, &self.adsr);
        }
        &self.keys
    }
}

struct Synth {
    sample_rate: f32,
    phases: PhaseStore,
    key_tracker: KeyAmplitudeTracker,
    attack_a: Arc<AtomicU16>,
    release_a: Arc<AtomicU16>,
    active_keys: Arc<AtomicUsize>,
    gain_a: Arc<AtomicU16>,
}

impl Synth {
    pub fn new(
        sample_rate: f32,
        attack_a: Arc<AtomicU16>,
        release_a: Arc<AtomicU16>,
        active_keys: Arc<AtomicUsize>,
        gain_a: Arc<AtomicU16>,
    ) -> Self {
        Self {
            sample_rate,
            phases: PhaseStore::new(),
            key_tracker: KeyAmplitudeTracker::new(sample_rate),
            attack_a,
            release_a,
            active_keys,
            gain_a,
        }
    }

    #[inline(always)]
    pub fn on_buffer(&mut self, buffer: &mut [f32]) {
        self.key_tracker
            .update(self.active_keys.load(Ordering::Relaxed));
        self.key_tracker.adsr.attack =
            self.attack_a.load(Ordering::Relaxed) as f32 / std::u16::MAX as f32;
        self.key_tracker.adsr.release =
            self.release_a.load(Ordering::Relaxed) as f32 / std::u16::MAX as f32;

        let fgain = self.gain_a.load(Ordering::Relaxed) as f32 / std::u16::MAX as f32;

        for sample in buffer.iter_mut() {
            let amps = self.key_tracker.tick();
            let mut sum_amps: f32 = 0.0;

            let mut sample_w: f32 = 0.0;
            for (index, element) in amps.iter().enumerate() {
                if element.amplitude == 0.0 {
                    continue;
                }

                let key = Key::from_zero_index(index);

                let phase = self.phases.get_phase(key);

                let phase_increment = TWO_PI * key.freq() / self.sample_rate;
                *phase += phase_increment;

                // Keep phase in the range [0, 2π]
                if *phase > TWO_PI {
                    *phase -= 2.0 * TWO_PI;
                }

                sum_amps += element.amplitude;
                sample_w += element.amplitude * phase.sin();
            }

            sample_w *= 1.0 / 1.0f32.max(sum_amps);

            *sample = fgain * sample_w;
        }
    }
}

pub struct Synthesizer {
    _host: cpal::Host,
    _device: cpal::Device,
    _supported_config: cpal::SupportedStreamConfig,
    _stream: cpal::Stream,
}

impl Synthesizer {
    pub fn new(
        gain: Arc<AtomicU16>,
        active_keys: Arc<AtomicUsize>,
        attack: Arc<AtomicU16>,
        release: Arc<AtomicU16>,
    ) -> Self {
        let host = cpal::host_from_id(
            cpal::available_hosts()
                .into_iter()
                .find(|id| *id == cpal::HostId::Jack)
                .unwrap(),
        )
        .unwrap();
        let device = host.default_output_device().unwrap();
        let supported_configs_range = device.supported_output_configs().unwrap();

        let supported_config = supported_configs_range
            .filter(|c| c.channels() >= 2)
            .next()
            .unwrap()
            .with_max_sample_rate();

        let sample_rate = supported_config.sample_rate().0 as f32;
        let mut synth = Synth::new(sample_rate, attack, release, active_keys, gain);
        println!("[DEBUG] Sample rate: {sample_rate}");
        println!("[DEBUG] Buffer size: {:?}", supported_config.buffer_size());
        let stream = device
            .build_output_stream(
                &supported_config.config(),
                move |data: &mut [f32], _info: &cpal::OutputCallbackInfo| {
                    synth.on_buffer(data);
                },
                move |_err| {},
                None,
            )
            .unwrap();

        stream.play().unwrap();

        Self {
            _host: host,
            _device: device,
            _supported_config: supported_config,
            _stream: stream,
        }
    }
}
