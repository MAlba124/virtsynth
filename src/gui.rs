use std::sync::atomic::Ordering;

use eframe::egui::{self, Margin, Slider, Theme};

use crate::{
    keyboard::{Key, KeyBitflags, Keyboard},
    waveform::Waveform,
};

struct Envelope {
    pub attack: f32,
    pub _decay: f32,
    pub _sustain: f32,
    pub release: f32,
}

impl Default for Envelope {
    fn default() -> Self {
        Self {
            attack: 0.03,
            _decay: 1.0,
            _sustain: 1.0,
            release: 0.2,
        }
    }
}

pub struct VirtSynth {
    keyboard: Keyboard,
    envelope: Envelope,
    gain: f32,
}

impl VirtSynth {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        cc.egui_ctx.set_theme(Theme::Light);

        let keyboard = Keyboard::new();

        Self {
            keyboard,
            envelope: Envelope::default(),
            gain: 0.5,
        }
    }

    fn get_active_keys(&self, ctx: &egui::Context) -> KeyBitflags {
        let mut active_keys = KeyBitflags(0, 1);
        if ctx.input(|i| i.key_down(egui::Key::Z)) {
            active_keys.0 |= Key::C4.bitflag();
        }
        if ctx.input(|i| i.key_down(egui::Key::S)) {
            active_keys.0 |= Key::CS.bitflag();
        }
        if ctx.input(|i| i.key_down(egui::Key::X)) {
            active_keys.0 |= Key::D4.bitflag();
        }
        if ctx.input(|i| i.key_down(egui::Key::D)) {
            active_keys.0 |= Key::DS.bitflag();
        }
        if ctx.input(|i| i.key_down(egui::Key::C)) {
            active_keys.0 |= Key::E4.bitflag();
        }
        if ctx.input(|i| i.key_down(egui::Key::V)) {
            active_keys.0 |= Key::F4.bitflag();
        }
        if ctx.input(|i| i.key_down(egui::Key::G)) {
            active_keys.0 |= Key::FS.bitflag();
        }
        if ctx.input(|i| i.key_down(egui::Key::B)) {
            active_keys.0 |= Key::G4.bitflag();
        }
        if ctx.input(|i| i.key_down(egui::Key::H)) {
            active_keys.0 |= Key::GS.bitflag();
        }
        if ctx.input(|i| i.key_down(egui::Key::N)) {
            active_keys.0 |= Key::A4.bitflag();
        }
        if ctx.input(|i| i.key_down(egui::Key::J)) {
            active_keys.0 |= Key::AS.bitflag();
        }
        if ctx.input(|i| i.key_down(egui::Key::M)) {
            active_keys.0 |= Key::B4.bitflag();
        }
        active_keys
    }
}

impl eframe::App for VirtSynth {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let active_keys = self.get_active_keys(ctx);
            self.keyboard.set_active_keys(active_keys.0);

            ui.horizontal_top(|ui| {
                egui::Frame::default()
                    .stroke(ui.visuals().widgets.noninteractive.bg_stroke)
                    .inner_margin(Margin::same(5.0))
                    .rounding(ui.visuals().widgets.noninteractive.rounding)
                    .show(ui, |ui| {
                        ui.vertical(|ui| {
                            ui.label("Master");
                            ui.add(
                                Slider::new(&mut self.gain, 0.0..=1.0)
                                    .step_by(0.01)
                                    .text("Gain")
                                    .orientation(egui::SliderOrientation::Vertical),
                            );
                            self.keyboard.gain.store(self.gain, Ordering::Relaxed);
                        });
                    });

                egui::Frame::default()
                    .stroke(ui.visuals().widgets.noninteractive.bg_stroke)
                    .inner_margin(Margin::same(5.0))
                    .rounding(ui.visuals().widgets.noninteractive.rounding)
                    .show(ui, |ui| {
                        ui.vertical(|ui| {
                            let mut osc = self.keyboard.osc_active.load(Ordering::Relaxed);
                            ui.horizontal(|ui| {
                                ui.checkbox(&mut osc, "");
                                ui.label("Oscillator");
                            });
                            self.keyboard.osc_active.store(osc, Ordering::Relaxed);
                            ui.horizontal(|ui| {
                                if !osc {
                                    ui.disable();
                                }
                                let mut freq = self.keyboard.osc_frequency.load(Ordering::Relaxed);
                                ui.add(
                                    Slider::new(&mut freq, 1.0..=300.0)
                                        .text("Frequency")
                                        .suffix("Hz")
                                        .orientation(egui::SliderOrientation::Vertical),
                                );
                                self.keyboard.osc_frequency.store(freq, Ordering::Relaxed);

                                ui.vertical(|ui| {
                                    let mut osc_wave =
                                        self.keyboard.osc_waveform.load(Ordering::Relaxed);
                                    ui.radio_value(&mut osc_wave, Waveform::Sin, "Sine");
                                    ui.radio_value(&mut osc_wave, Waveform::Square, "Square");
                                    self.keyboard
                                        .osc_waveform
                                        .store(osc_wave, Ordering::Relaxed);
                                });
                            });
                        });
                    });

                egui::Frame::default()
                    .stroke(ui.visuals().widgets.noninteractive.bg_stroke)
                    .inner_margin(Margin::same(5.0))
                    .rounding(ui.visuals().widgets.noninteractive.rounding)
                    .show(ui, |ui| {
                        ui.vertical(|ui| {
                            ui.label("Envelope");
                            ui.horizontal(|ui| {
                                ui.add(
                                    Slider::new(&mut self.envelope.attack, 0.0..=1.0)
                                        .step_by(0.01)
                                        .text("Attack")
                                        .suffix("s")
                                        .orientation(egui::SliderOrientation::Vertical),
                                );
                                self.keyboard
                                    .attack
                                    .store(self.envelope.attack, Ordering::Relaxed);
                                // TODO
                                // Decay
                                // ui.add(
                                //     Slider::new(&mut self.envelope.decay, 0.0..=1.0)
                                //         .step_by(0.01)
                                //         .orientation(egui::SliderOrientation::Vertical),
                                // );
                                // Sustain
                                // ui.add(
                                //     Slider::new(&mut self.envelope.sustain, 0.0..=1.0)
                                //         .step_by(0.01)
                                //         .orientation(egui::SliderOrientation::Vertical),
                                // );
                                ui.add(
                                    Slider::new(&mut self.envelope.release, 0.0..=1.0)
                                        .step_by(0.01)
                                        .text("Release")
                                        .suffix("s")
                                        .orientation(egui::SliderOrientation::Vertical),
                                );
                                self.keyboard
                                    .release
                                    .store(self.envelope.release, Ordering::Relaxed);
                            });
                        });
                    });
            });
        });
    }
}
