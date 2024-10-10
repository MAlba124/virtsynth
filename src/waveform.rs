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

#[derive(PartialEq)]
pub enum Waveform {
    Sin = 1,
    Square = 2,
    Saw = 3,
    Triangle = 4,
}

impl From<i32> for Waveform {
    fn from(value: i32) -> Self {
        match value {
            1 => Self::Sin,
            2 => Self::Square,
            3 => Self::Saw,
            4 => Self::Triangle,
            _ => panic!("Invalid waveform integer"),
        }
    }
}
