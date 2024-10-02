use virtsynth::gui::VirtSynth;

fn main() {
    let mut native_options = eframe::NativeOptions::default();
    native_options.vsync = true;
    eframe::run_native(
        "VirtSynth",
        native_options,
        Box::new(|cc| Ok(Box::new(VirtSynth::new(cc)))),
    )
    .unwrap();
}
