use crate::{
    Mass, Pressure, Speed,
};

pub enum Variant {
    Bool(bool),
    I8(i8),
    I32(i32),
    F32(f32),
    U8(u8),
    U32(u32),

    Mass(Mass),
    Pressure(Pressure),
    Speed(Speed),
}
