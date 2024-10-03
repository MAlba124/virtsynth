use eframe::egui::{self, Slider, Theme};
use egui_plot::PlotPoints;

use crate::keyboard::{Key, KeyBitflags, Keyboard};

struct Envelope {
    pub attack: f32,
    pub decay: f32,
    pub sustain: f32,
    pub release: f32,
}

impl Default for Envelope {
    fn default() -> Self {
        Self {
            attack: 0.2,
            decay: 1.0,
            sustain: 1.0,
            release: 0.2,
        }
    }
}

pub struct VirtSynth {
    keyboard: Keyboard,
    envelope: Envelope,
}

impl VirtSynth {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        cc.egui_ctx.set_theme(Theme::Light);

        let keyboard = Keyboard::new();

        Self {
            keyboard,
            envelope: Envelope::default(),
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
            ui.label("Gain");
            let mut new_gain = self.keyboard.gain();
            let old_gain = new_gain;
            ui.add(
                Slider::new(&mut new_gain, 0.0..=1.0)
                    .step_by(0.01)
                    .orientation(egui::SliderOrientation::Vertical),
            );
            if new_gain != old_gain {
                self.keyboard.set_gain(new_gain);
            }

            let active_keys = self.get_active_keys(ctx);
            self.keyboard.set_active_keys(active_keys.0);

            ui.horizontal(|ui| {
                ui.columns(2, |cols| {
                    cols[0].vertical_centered_justified(|ui| {
                        // TODO: Cache
                        let samples = 256;
                        let sin: egui_plot::PlotPoints = (0..samples)
                            .map(|i| {
                                let x = (i as f64 / samples as f64) * (2.0 * std::f64::consts::PI);
                                let mut val: f64 = 0.0;
                                let mut n_keys = 0;
                                for key in active_keys {
                                    n_keys += 1;
                                    val +=
                                        (2.0 * std::f64::consts::PI * key.freq() as f64 * x).sin();
                                }

                                if n_keys > 1 {
                                    val = val / n_keys as f64;
                                }

                                [x, val]
                            })
                            .collect();
                        let line = egui_plot::Line::new(sin);
                        egui_plot::Plot::new("waveform")
                            .view_aspect(1.5)
                            .allow_zoom(false)
                            .allow_scroll(false)
                            .allow_drag(false)
                            .show(ui, |plot_ui| plot_ui.line(line));
                    });
                    cols[1].vertical(|ui| {
                        ui.label("Envelope");
                        ui.horizontal(|ui| {
                            ui.add(
                                Slider::new(&mut self.envelope.attack, 0.0..=1.0)
                                    .step_by(0.01)
                                    .text("Attack")
                                    .suffix("s")
                                    .orientation(egui::SliderOrientation::Vertical),
                            );
                            self.keyboard.attack.store(
                                (std::u16::MAX as f32 * self.envelope.attack) as u16,
                                std::sync::atomic::Ordering::Relaxed,
                            );
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
                            self.keyboard.release.store(
                                (std::u16::MAX as f32 * self.envelope.release) as u16,
                                std::sync::atomic::Ordering::Relaxed,
                            );
                        });
                        // TODO
                        // let points = PlotPoints::from_ys_f32(&[
                        //     0.0,
                        //     self.envelope.attack,
                        //     self.envelope.decay,
                        //     self.envelope.sustain,
                        //     self.envelope.release,
                        //     0.0,
                        // ]);
                        // egui_plot::Plot::new("envelope").show(ui, |plot_ui| {
                        //     plot_ui.polygon(egui_plot::Polygon::new(points))
                        // });
                    });
                })
            });
        });
    }
}
