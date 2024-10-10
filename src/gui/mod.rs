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

use std::sync::atomic::Ordering;

use eframe::egui::{self, DragValue, Margin, Theme, Ui};
use knob::Knob;

use crate::{
    keyboard::{Key, KeyBitflags, Keyboard, Osc},
    waveform::Waveform,
};

mod knob;

fn osc_ui(ui: &mut Ui, osc: &mut Osc, label: &str) {
    egui::Frame::default()
        .stroke(ui.visuals().widgets.noninteractive.bg_stroke)
        .inner_margin(Margin::same(5.0))
        .rounding(ui.visuals().widgets.noninteractive.rounding)
        .show(ui, |ui| {
            ui.vertical(|ui| {
                let mut active = osc.active.load(Ordering::Acquire);
                ui.horizontal(|ui| {
                    ui.checkbox(&mut active, label);
                });
                osc.active.store(active, Ordering::Release);

                ui.columns(2, |columns| {
                    columns[0].vertical_centered(|ui| {
                        if !active {
                            ui.disable();
                        }

                        let mut gain = osc.gain.load(Ordering::Acquire);
                        ui.label("Volume");
                        ui.add(Knob::new(&mut gain, 0.0..=1.0, 0.01));
                        ui.add(DragValue::new(&mut gain).range(0.0..=1.0).speed(0.01));
                        osc.gain.store(gain, Ordering::Release);
                    });

                    columns[1].vertical(|ui| {
                        if !active {
                            ui.disable();
                        }

                        ui.label("Waveform");
                        let mut osc_wave = osc.waveform.load(Ordering::Acquire);
                        ui.radio_value(&mut osc_wave, Waveform::Sin, "Sine");
                        ui.radio_value(&mut osc_wave, Waveform::Square, "Square");
                        ui.radio_value(&mut osc_wave, Waveform::Saw, "Saw");
                        ui.radio_value(&mut osc_wave, Waveform::Triangle, "Triangle");
                        osc.waveform.store(osc_wave, Ordering::Release);
                    });
                });
            });
        });
}

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
                                    ui.label("Volume");
                                    let mut gain = self.keyboard.gain.load(Ordering::Acquire);
                                    ui.add(Knob::new(&mut gain, 0.0..=1.0, 0.01));
                                    let mut gain_perc = (gain * 100.0) as u8;
                                    ui.add(DragValue::new(&mut gain_perc).speed(1).suffix("%"));
                                    gain = gain_perc as f32 / 100.0;
                                    self.keyboard.gain.store(gain, Ordering::Release);
                                });
                            });
                        });
                    });

                ui.end_row();

                ui.columns(3, |colums| {
                    colums[0].horizontal(|ui| {
                        osc_ui(ui, &mut self.keyboard.osc1, "Oscillator 1");
                    });
                    colums[1].horizontal(|ui| {
                        osc_ui(ui, &mut self.keyboard.osc2, "Oscillator 2");
                    });
                    colums[2].horizontal(|ui| {
                        osc_ui(ui, &mut self.keyboard.osc3, "Oscillator 3");
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
                                    ui.add(Knob::new(&mut attack, 0.0..=1.0, 0.01));
                                    self.keyboard.attack.store(attack, Ordering::Release);
                                });
                                columns[1].vertical_centered(|ui| {
                                    let mut decay = self.keyboard.decay.load(Ordering::Acquire);
                                    ui.label("Decay");
                                    ui.add(Knob::new(&mut decay, 0.0..=1.0, 0.01));
                                    self.keyboard.decay.store(decay, Ordering::Release);
                                });
                                columns[2].vertical_centered(|ui| {
                                    let mut sustain = self.keyboard.sustain.load(Ordering::Acquire);
                                    ui.label("Sustain");
                                    ui.add(Knob::new(&mut sustain, 0.0..=1.0, 0.01));
                                    self.keyboard.sustain.store(sustain, Ordering::Release);
                                });
                                columns[3].vertical_centered(|ui| {
                                    let mut release = self.keyboard.release.load(Ordering::Acquire);
                                    ui.label("Release");
                                    ui.add(Knob::new(&mut release, 0.0..=1.0, 0.01));
                                    self.keyboard.release.store(release, Ordering::Release);
                                });
                            });
                        });
                    });
            });
        });
    }
}
