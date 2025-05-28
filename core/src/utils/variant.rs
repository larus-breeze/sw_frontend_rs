use crate::{
    model::{DataSource, DisplayActive, DisplayTheme, VarioModeControl},
    Mass, Pressure, Speed,
};

use super::Rotation;

pub enum Variant {
    Bool(bool),
    I8(i8),
    I32(i32),
    F32(f32),
    U8(u8),
    U32(u32),

    DisplayActive(DisplayActive),
    DisplayTheme(DisplayTheme),
    Mass(Mass),
    Pressure(Pressure),
    Speed(Speed),
    VarioModeControl(VarioModeControl),
    Rotation(Rotation),
    DataSource(DataSource),
}
