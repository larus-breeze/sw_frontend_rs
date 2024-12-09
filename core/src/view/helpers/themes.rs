use crate::utils::Colors;
use u8g2_fonts::{fonts, FontRenderer};

#[cfg(feature = "air_avionics_ad57")]
pub const FONT_SMALL: FontRenderer = FontRenderer::new::<fonts::u8g2_font_helvB14_tf>();
#[cfg(feature = "larus_frontend_v1")]
pub const FONT_SMALL: FontRenderer = FontRenderer::new::<fonts::u8g2_font_helvB14_tf>();
#[cfg(feature = "larus_frontend_v2")]
pub const FONT_SMALL: FontRenderer = FontRenderer::new::<fonts::u8g2_font_fub17_tf>();

#[cfg(feature = "air_avionics_ad57")]
pub const FONT_BIG: FontRenderer = FontRenderer::new::<fonts::u8g2_font_fub17_tf>();
#[cfg(feature = "larus_frontend_v1")]
pub const FONT_BIG: FontRenderer = FontRenderer::new::<fonts::u8g2_font_fub20_tf>();
#[cfg(feature = "larus_frontend_v2")]
pub const FONT_BIG: FontRenderer = FontRenderer::new::<fonts::u8g2_font_fub30_tf>();

pub type PaletteColors = [Colors; 24];

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
    Text1,
    Text1Bold,
    Text2,
    Text2Bold,

    HorizonSky,
    HorizonEarth,

    VarioSpeedToFly,
    VarioPicInfo1,
    VarioWindPlus,
    VarioWindMinus,
}

#[cfg(feature = "larus_frontend_v1")]
pub const DARK_MODE: PaletteColors = [
    Colors::Black,        // BackGround
    Colors::White,        // Scale,
    Colors::Black,        // Needle1,
    Colors::Red,          // Needle2,
    Colors::Green,        // Needle3,
    Colors::Yellow,       // Needle4,
    Colors::Red,          // Needle5,
    Colors::White,        // Sprite1Stroke,
    Colors::DodgerBlue,   // Sprite1Fill,
    Colors::White,        // Sprite2Stroke,
    Colors::DodgerBlue,   // Sprite2Fill,
    Colors::Red,          // SignalStop,
    Colors::Yellow,       // SignalWarning,
    Colors::LimeGreen,    // SignalGo,
    Colors::Coral,        // Text1,
    Colors::Bisque,       // Text1Bold,
    Colors::LightSkyBlue, // Text2,
    Colors::White,        // Text2Bold,
    Colors::LightSkyBlue, // HorizonSky,
    Colors::Sienna,       // HorizonEarth,
    Colors::Orange,       // VarioSpeedToFly
    Colors::Orange,       // VarioPicInfo1
    Colors::Orange,       // VarioWindPlus
    Colors::LightPink,    // VarioWindMinus
];

#[cfg(feature = "larus_frontend_v2")]
pub const DARK_MODE: PaletteColors = [
    Colors::Black,        // BackGround
    Colors::White,        // Scale,
    Colors::Black,        // Needle1,
    Colors::Red,          // Needle2,
    Colors::Green,        // Needle3,
    Colors::Yellow,       // Needle4,
    Colors::Red,          // Needle5,
    Colors::White,        // Sprite1Stroke,
    Colors::DodgerBlue,   // Sprite1Fill,
    Colors::White,        // Sprite2Stroke,
    Colors::DodgerBlue,   // Sprite2Fill,
    Colors::Red,          // SignalStop,
    Colors::Yellow,       // SignalWarning,
    Colors::LimeGreen,    // SignalGo,
    Colors::Coral,        // Text1,
    Colors::Bisque,       // Text1Bold,
    Colors::LightSkyBlue, // Text2,
    Colors::White,        // Text2Bold,
    Colors::LightSkyBlue, // HorizonSky,
    Colors::Sienna,       // HorizonEarth,
    Colors::Orange,       // VarioSpeedToFly
    Colors::Orange,       // VarioPicInfo1
    Colors::Orange,       // VarioWindPlus
    Colors::LightPink,    // VarioWindMinus
];

#[cfg(feature = "air_avionics_ad57")]
pub const DARK_MODE: PaletteColors = [
    Colors::Black,        // BackGround
    Colors::White,        // Scale,
    Colors::Black,        // Needle1,
    Colors::Red,          // Needle2,
    Colors::Green,        // Needle3,
    Colors::Yellow,       // Needle4,
    Colors::Red,          // Needle5,
    Colors::White,        // Sprite1Stroke,
    Colors::DodgerBlue,   // Sprite1Fill,
    Colors::White,        // Sprite2Stroke,
    Colors::DodgerBlue,   // Sprite2Fill,
    Colors::Red,          // SignalStop,
    Colors::Yellow,       // SignalWarning,
    Colors::LimeGreen,    // SignalGo,
    Colors::Coral,        // Text1,
    Colors::Bisque,       // Text1Bold,
    Colors::LightSkyBlue, // Text2,
    Colors::White,        // Text2Bold,
    Colors::LightSkyBlue, // HorizonSky,
    Colors::Sienna,       // HorizonEarth,
    Colors::Orange,       // VarioSpeedToFly
    Colors::Orange,       // VarioPicInfo1
    Colors::Orange,       // VarioWindPlus
    Colors::LightPink,    // VarioWindMinus
];

#[cfg(feature = "larus_frontend_v1")]
pub const BRIGHT_MODE: PaletteColors = [
    Colors::White,        // BackGround
    Colors::Black,        // Scale,
    Colors::Yellow,       // Needle1,
    Colors::Red,          // Needle2,
    Colors::Green,        // Needle3,
    Colors::Green,        // Needle4,
    Colors::Red,          // Needle5,
    Colors::Black,        // Sprite1Stroke,
    Colors::Blue,         // Sprite1Fill,
    Colors::Black,        // Sprite2Stroke,
    Colors::Magenta,      // Sprite2Fill,
    Colors::Red,          // SignalStop,
    Colors::Gold,         // SignalWarning,
    Colors::LimeGreen,    // SignalGo,
    Colors::Coral,        // Text1,
    Colors::OldLace,      // Text1Bold,
    Colors::LightSkyBlue, // Text2,
    Colors::LightCyan,    // Text2Bold,
    Colors::Azure,        // HorizonSky,
    Colors::Sienna,       // HorizonEarth,
    Colors::Coral,        // VarioSpeedToFly
    Colors::Coral,        // VarioPicInfo1
    Colors::Orange,       // VarioWindPlus
    Colors::Red,          // VarioWindMinus
];

#[cfg(feature = "larus_frontend_v2")]
pub const BRIGHT_MODE: PaletteColors = [
    Colors::White,        // BackGround
    Colors::Black,        // Scale,
    Colors::Yellow,       // Needle1,
    Colors::Red,          // Needle2,
    Colors::Green,        // Needle3,
    Colors::Green,        // Needle4,
    Colors::Red,          // Needle5,
    Colors::Black,        // Sprite1Stroke,
    Colors::Blue,         // Sprite1Fill,
    Colors::Black,        // Sprite2Stroke,
    Colors::Magenta,      // Sprite2Fill,
    Colors::Red,          // SignalStop,
    Colors::Gold,         // SignalWarning,
    Colors::LimeGreen,    // SignalGo,
    Colors::Coral,        // Text1,
    Colors::OldLace,      // Text1Bold,
    Colors::LightSkyBlue, // Text2,
    Colors::LightCyan,    // Text2Bold,
    Colors::Azure,        // HorizonSky,
    Colors::Sienna,       // HorizonEarth,
    Colors::Coral,        // VarioSpeedToFly
    Colors::Coral,        // VarioPicInfo1
    Colors::Orange,       // VarioWindPlus
    Colors::Red,          // VarioWindMinus
];

#[cfg(feature = "air_avionics_ad57")]
pub const BRIGHT_MODE: PaletteColors = [
    Colors::White,        // BackGround
    Colors::Black,        // Scale,
    Colors::Yellow,       // Needle1,
    Colors::Red,          // Needle2,
    Colors::Green,        // Needle3,
    Colors::Green,        // Needle4,
    Colors::Red,          // Needle5,
    Colors::Black,        // Sprite1Stroke,
    Colors::Blue,         // Sprite1Fill,
    Colors::Black,        // Sprite2Stroke,
    Colors::Magenta,      // Sprite2Fill,
    Colors::Red,          // SignalStop,
    Colors::Gold,         // SignalWarning,
    Colors::LimeGreen,    // SignalGo,
    Colors::Coral,        // Text1,
    Colors::OldLace,      // Text1Bold,
    Colors::LightSkyBlue, // Text2,
    Colors::LightCyan,    // Text2Bold,
    Colors::Azure,        // HorizonSky,
    Colors::Sienna,       // HorizonEarth,
    Colors::Coral,        // VarioSpeedToFly
    Colors::Coral,        // VarioPicInfo1
    Colors::Orange,       // VarioWindPlus
    Colors::Red,          // VarioWindMinus
];
