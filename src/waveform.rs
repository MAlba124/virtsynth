#[derive(PartialEq)]
pub enum Waveform {
    Sin = 1,
    Square = 2,
}

impl From<i32> for Waveform {
    fn from(value: i32) -> Self {
        match value {
            1 => Self::Sin,
            2 => Self::Square,
            _ => panic!("Invalid waveform integer"),
        }
    }
}

#[inline(always)]
pub fn sin_to_square(sample: f32) -> f32 {
    if sample >= 0.0 {
        1.0
    } else {
        -1.0
    }
}
