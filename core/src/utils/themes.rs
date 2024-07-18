use crate::utils::Colors;

pub type PaletteColors = [Colors; 16];

pub enum Palette {
    Background,
    Scale,
    Needle1,
    Needle2,
    Needle3,
    Needle4,
    Needle5,
    Sprite1Stroke,
    Sprite1Fill,
    Sprite2Stroke,
    Sprite2Fill,
    SignalStop,
    SignalWarning,
    SignalGo,
    HorizonSky,
    HorizonEarth,
}

#[cfg(feature = "larus_ad57")]
pub const DARK_MODE: PaletteColors = [
    Colors::Black,        // BackGround
    Colors::White,        // Scale,
    Colors::White,        // Needle1,
    Colors::Red,          // Needle2,
    Colors::LimeGreen,    // Needle3,
    Colors::Yellow,       // Needle4,
    Colors::Coral,        // Needle5,
    Colors::LightSkyBlue, // Sprite1Stroke,
    Colors::Blue,         // Sprite1Fill,
    Colors::White,        // Sprite2Stroke,
    Colors::Magenta,      // Sprite2Fill,
    Colors::Red,          // SignalStop,
    Colors::Yellow,       // SignalWarning,
    Colors::LimeGreen,    // SignalGo,
    Colors::LightSkyBlue, // HorizonSky,
    Colors::Sienna,       // HorizonEarth,
];

#[cfg(feature = "air_avionics_ad57")]
pub const DARK_MODE: PaletteColors = [
    Colors::Black,        // BackGround
    Colors::White,        // Scale,
    Colors::White,        // Needle1,
    Colors::Red,          // Needle2,
    Colors::LimeGreen,    // Needle3,
    Colors::Yellow,       // Needle4,
    Colors::Coral,        // Needle5,
    Colors::LightSkyBlue, // Sprite1Stroke,
    Colors::Blue,         // Sprite1Fill,
    Colors::White,        // Sprite2Stroke,
    Colors::Magenta,      // Sprite2Fill,
    Colors::Red,          // SignalStop,
    Colors::Yellow,       // SignalWarning,
    Colors::LimeGreen,    // SignalGo,
    Colors::DodgerBlue,   // HorizonSky,
    Colors::Sienna,       // HorizonEarth,
];

#[cfg(feature = "larus_ad57")]
pub const BRIGHT_MODE: PaletteColors = [
    Colors::White,     // BackGround
    Colors::Black,     // Scale,
    Colors::Black,     // Needle1,
    Colors::Red,       // Needle2,
    Colors::Green,     // Needle3,
    Colors::Green,     // Needle4,
    Colors::Sienna,    // Needle5,
    Colors::Black,     // Sprite1Stroke,
    Colors::Blue,      // Sprite1Fill,
    Colors::Black,     // Sprite2Stroke,
    Colors::Magenta,   // Sprite2Fill,
    Colors::Red,       // SignalStop,
    Colors::Gold,      // SignalWarning,
    Colors::LimeGreen, // SignalGo,
    Colors::Azure,     // HorizonSky,
    Colors::Sienna,    // HorizonEarth,
];

#[cfg(feature = "air_avionics_ad57")]
pub const BRIGHT_MODE: PaletteColors = [
    Colors::White,        // BackGround
    Colors::Black,        // Scale,
    Colors::Black,        // Needle1,
    Colors::Red,          // Needle2,
    Colors::Green,        // Needle3,
    Colors::Green,        // Needle4,
    Colors::Sienna,       // Needle5,
    Colors::Black,        // Sprite1Stroke,
    Colors::Blue,         // Sprite1Fill,
    Colors::Black,        // Sprite2Stroke,
    Colors::Magenta,      // Sprite2Fill,
    Colors::Red,          // SignalStop,
    Colors::Gold,         // SignalWarning,
    Colors::LimeGreen,    // SignalGo,
    Colors::LightSkyBlue, // HorizonSky,
    Colors::Sienna,       // HorizonEarth,
];
