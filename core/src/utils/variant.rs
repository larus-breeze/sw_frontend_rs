use crate::{
    model::{DisplayActive, VarioModeControl},
    Mass, Pressure, Speed,
};

use super::Rotation;

pub enum Variant<'a> {
    Bool(bool),
    I8(i8),
    I32(i32),
    F32(f32),
    Str(&'a str),
    U8(u8),
    U32(u32),
    Usize(usize),

    DisplayActive(DisplayActive),
    Mass(Mass),
    Pressure(Pressure),
    Speed(Speed),
    VarioModeControl(VarioModeControl),
    Rotation(Rotation),
}
