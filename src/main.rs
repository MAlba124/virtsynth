use virtsynth::gui::VirtSynth;

fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "VirtSynth",
        native_options,
        Box::new(|cc| Ok(Box::new(VirtSynth::new(cc)))),
    )
    .unwrap();
}
