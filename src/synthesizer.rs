use std::sync::{
    atomic::{AtomicBool, AtomicUsize, Ordering},
    Arc,
};

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

use crate::{
    atomicf::{AtomicF32, AtomicWaveform},
    envelope::ADSR,
    keyboard::Key,
    oscilator::Oscilator,
};

pub const TWO_PI: f32 = 2.0 * std::f32::consts::PI;

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
    Decay,
    Sustain,
    Released,
}

#[derive(Clone, Copy)]
struct TrackElement {
    pub state: KeyState,
    pub amplitude: f32,
    pub position: f32,
    pub t_amplitude: f32,
}

impl TrackElement {
    #[inline(always)]
    pub fn press(&mut self) {
        if self.state == KeyState::Released {
            self.state = KeyState::Pressed;
            self.position = 0.0;
            self.t_amplitude = self.amplitude;
        }
    }

    #[inline(always)]
    pub fn release(&mut self) {
        if self.state != KeyState::Released {
            self.state = KeyState::Released;
            self.position = 0.0;
            self.t_amplitude = self.amplitude;
        }
    }

    #[inline(always)]
    pub fn tick(&mut self, sample_rate: f32, adsr: &ADSR) {
        match self.state {
            KeyState::Pressed => {
                self.position += 1.0;
                if self.position < sample_rate * adsr.attack {
                    self.amplitude += (1.0 - self.t_amplitude) / (sample_rate * adsr.attack);
                    return;
                }

                self.amplitude = 1.0;
                self.position = 0.0;
                self.state = KeyState::Decay;
            }
            KeyState::Decay => {
                self.position += 1.0;
                if self.position < sample_rate * adsr.decay {
                    self.amplitude -= (1.0 - adsr.sustain) / (sample_rate * adsr.decay);
                    return;
                }

                self.amplitude = adsr.sustain;
                self.state = KeyState::Sustain;
            }
            KeyState::Sustain => {
                self.amplitude = adsr.sustain;
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
    pub fn new(
        sample_rate: f32,
        attack_a: Arc<AtomicF32>,
        decay_a: Arc<AtomicF32>,
        sustain_a: Arc<AtomicF32>,
        release_a: Arc<AtomicF32>,
    ) -> Self {
        Self {
            sample_rate,
            keys: [TrackElement::default(); 12],
            adsr: ADSR::new(attack_a, decay_a, sustain_a, release_a),
        }
    }

    #[inline(always)]
    pub fn update(&mut self, keys: usize) {
        self.adsr.update();

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

struct Engine {
    sample_rate: f32,
    phases: PhaseStore,
    key_tracker: KeyAmplitudeTracker,
    active_keys: Arc<AtomicUsize>,
    gain_a: Arc<AtomicF32>,
    osc: Oscilator,
}

impl Engine {
    pub fn new(
        sample_rate: f32,
        attack_a: Arc<AtomicF32>,
        decay_a: Arc<AtomicF32>,
        sustain_a: Arc<AtomicF32>,
        release_a: Arc<AtomicF32>,
        active_keys: Arc<AtomicUsize>,
        gain_a: Arc<AtomicF32>,
        osc_active: Arc<AtomicBool>,
        osc_frequency: Arc<AtomicF32>,
        osc_waveform: Arc<AtomicWaveform>,
        osc_scale: Arc<AtomicF32>,
    ) -> Self {
        Self {
            sample_rate,
            phases: PhaseStore::new(),
            key_tracker: KeyAmplitudeTracker::new(
                sample_rate,
                attack_a,
                decay_a,
                sustain_a,
                release_a,
            ),
            active_keys,
            gain_a,
            osc: Oscilator::new(osc_frequency, osc_waveform, osc_active, osc_scale),
        }
    }

    #[inline(always)]
    pub fn on_buffer(&mut self, buffer: &mut [f32], channels: usize) {
        self.key_tracker
            .update(self.active_keys.load(Ordering::Acquire));

        let fgain = self.gain_a.load(Ordering::Acquire);

        self.osc.update();

        for sample_frame in buffer.chunks_mut(channels) {
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

                if *phase > TWO_PI {
                    *phase -= TWO_PI;
                }

                sum_amps += element.amplitude;
                sample_w += element.amplitude * phase.sin();
            }

            // TODO: Learn how synths work
            if sum_amps > 0.0 {
                let osc_amp = self.osc.tick(self.sample_rate);
                sample_w += osc_amp * sum_amps.min(1.0);
                // sum_amps += osc_amp;
            }

            sample_w *= 1.0 / 1.0f32.max(sum_amps);

            // let the_sample = fgain * sample_w * self.osc.tick(self.sample_rate);
            let the_sample = fgain * sample_w;

            for sample in sample_frame.iter_mut() {
                *sample = the_sample;
            }
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
        gain: Arc<AtomicF32>,
        active_keys: Arc<AtomicUsize>,
        attack: Arc<AtomicF32>,
        decay: Arc<AtomicF32>,
        sustain: Arc<AtomicF32>,
        release: Arc<AtomicF32>,
        osc_active: Arc<AtomicBool>,
        osc_frequency: Arc<AtomicF32>,
        osc_waveform: Arc<AtomicWaveform>,
        osc_scale: Arc<AtomicF32>,
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
        let mut synth = Engine::new(
            sample_rate,
            attack,
            decay,
            sustain,
            release,
            active_keys,
            gain,
            osc_active,
            osc_frequency,
            osc_waveform,
            osc_scale,
        );
        let channels = supported_config.channels() as usize;

        println!("[DEBUG] Channels:    {channels}");
        println!("[DEBUG] Sample rate: {sample_rate}");
        println!("[DEBUG] Buffer size: {:?}", supported_config.buffer_size());

        let stream = device
            .build_output_stream(
                &supported_config.config(),
                move |data: &mut [f32], _info: &cpal::OutputCallbackInfo| {
                    synth.on_buffer(data, channels);
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
