/*
 * VirtSynth - A bare bones virtual synthesizer
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
