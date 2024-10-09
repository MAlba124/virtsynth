use std::sync::atomic::Ordering;

use eframe::egui::{self, DragValue, Margin, Theme};
use knob::Knob;

use crate::{
    keyboard::{Key, KeyBitflags, Keyboard},
    waveform::Waveform,
};

mod knob;

pub struct VirtSynth {
    keyboard: Keyboard,
}

impl VirtSynth {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        cc.egui_ctx.set_theme(Theme::Light);
        Self {
            keyboard: Keyboard::new(),
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

            ui.horizontal_wrapped(|ui| {
                egui::Frame::default()
                    .stroke(ui.visuals().widgets.noninteractive.bg_stroke)
                    .inner_margin(Margin::same(5.0))
                    .rounding(ui.visuals().widgets.noninteractive.rounding)
                    .show(ui, |ui| {
                        ui.vertical(|ui| {
                            ui.label("Master");
                            ui.columns(1, |columns| {
                                columns[0].vertical_centered(|ui| {
                                    ui.label("Gain");
                                    let mut gain = self.keyboard.gain.load(Ordering::Acquire);
                                    ui.add(Knob::new(&mut gain));
                                    let mut gain_perc = (gain * 100.0) as u8;
                                    ui.add(DragValue::new(&mut gain_perc ).speed(1).suffix("%"));
                                    gain = gain_perc as f32 / 100.0;
                                    self.keyboard.gain.store(gain, Ordering::Release);
                                });
                            });
                        });
                    });

                ui.end_row();

                egui::Frame::default()
                    .stroke(ui.visuals().widgets.noninteractive.bg_stroke)
                    .inner_margin(Margin::same(5.0))
                    .rounding(ui.visuals().widgets.noninteractive.rounding)
                    .show(ui, |ui| {
                        ui.vertical(|ui| {
                            let mut osc = self.keyboard.osc_active.load(Ordering::Acquire);
                            ui.horizontal(|ui| {
                                ui.checkbox(&mut osc, "");
                                ui.label("Oscillator");
                            });
                            self.keyboard.osc_active.store(osc, Ordering::Release);

                            ui.columns(3, |columns| {
                                columns[0].vertical_centered(|ui| {
                                    if !osc {
                                        ui.disable();
                                    }

                                    let mut freq = self.keyboard.osc_frequency.load(Ordering::Acquire);
                                    ui.label("Frequency");
                                    ui.add(Knob::new(&mut freq).range(1.0..=300.0).speed(1.0));
                                    ui.add(DragValue::new(&mut freq).range(1.0..=300.0).speed(0.5).suffix("Hz"));
                                    self.keyboard.osc_frequency.store(freq, Ordering::Release);
                                });

                                columns[1].vertical_centered(|ui| {
                                    if !osc {
                                        ui.disable();
                                    }

                                    let mut scale = self.keyboard.osc_scale.load(Ordering::Acquire);
                                    ui.label("Scale");
                                    ui.add(Knob::new(&mut scale));
                                    ui.add(DragValue::new(&mut scale).range(0.0..=1.0).speed(0.01));
                                    self.keyboard.osc_scale.store(scale, Ordering::Release);
                                });

                                columns[2].vertical(|ui| {
                                    if !osc {
                                        ui.disable();
                                    }

                                    ui.label("Waveform");
                                    let mut osc_wave =
                                        self.keyboard.osc_waveform.load(Ordering::Acquire);
                                    ui.radio_value(&mut osc_wave, Waveform::Sin, "Sine");
                                    ui.radio_value(&mut osc_wave, Waveform::Square, "Square");
                                    self.keyboard
                                        .osc_waveform
                                        .store(osc_wave, Ordering::Release);
                                });
                            });
                        });
                    });

                ui.end_row();

                egui::Frame::default()
                    .stroke(ui.visuals().widgets.noninteractive.bg_stroke)
                    .inner_margin(Margin::same(5.0))
                    .rounding(ui.visuals().widgets.noninteractive.rounding)
                    .show(ui, |ui| {
                        ui.vertical(|ui| {
                            ui.label("Envelope");
                            ui.columns(4, |columns| {
                                columns[0].vertical_centered(|ui| {
                                    let mut attack = self.keyboard.attack.load(Ordering::Acquire);
                                    ui.label("Attack");
                                    ui.add(Knob::new(&mut attack));
                                    self.keyboard.attack.store(attack, Ordering::Release);
                                });
                                columns[1].vertical_centered(|ui| {
                                    let mut decay = self.keyboard.decay.load(Ordering::Acquire);
                                    ui.label("Decay");
                                    ui.add(Knob::new(&mut decay));
                                    self.keyboard.decay.store(decay, Ordering::Release);
                                });
                                columns[2].vertical_centered(|ui| {
                                    let mut sustain = self.keyboard.sustain.load(Ordering::Acquire);
                                    ui.label("Sustain");
                                    ui.add(Knob::new(&mut sustain));
                                    self.keyboard.sustain.store(sustain, Ordering::Release);
                                });
                                columns[3].vertical_centered(|ui| {
                                    let mut release = self.keyboard.release.load(Ordering::Acquire);
                                    ui.label("Release");
                                    ui.add(Knob::new(&mut release));
                                    self.keyboard.release.store(release, Ordering::Release);
                                });
                            });
                        });
                    });
            });
        });
    }
}
