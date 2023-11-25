#![no_std]

mod fmc_lcd;
mod timing;

#[cfg(feature = "stm32f407")]
pub use stm32f4xx_hal as hal;

#[cfg(feature = "stm32f407")]
pub use hal::pac::{FSMC, fsmc};

#[cfg(feature = "stm32f407")]
mod fmcregs {
    pub type Bcr1 = super::fsmc::BCR1;
    pub type Bcr2 = super::fsmc::BCR;
    pub type Bcr3 = super::fsmc::BCR;
    pub type Bcr4 = super::fsmc::BCR;

    pub type Btr1 = super::fsmc::BTR;
    pub type Btr2 = super::fsmc::BTR;
    pub type Btr3 = super::fsmc::BTR;
    pub type Btr4 = super::fsmc::BTR;

    pub type Bwtr1 = super::fsmc::BWTR;
    pub type Bwtr2 = super::fsmc::BWTR;
    pub type Bwtr3 = super::fsmc::BWTR;
    pub type Bwtr4 = super::fsmc::BWTR;
}

#[cfg(feature = "stm32h743")]
pub use stm32h7xx_hal as hal;

#[cfg(feature = "stm32h743")]
pub use hal::pac::{FMC as FSMC, fmc as fsmc};

#[cfg(feature = "stm32h743")]
mod fmcregs {
    pub type Bcr1 = super::fsmc::BCR1;
    pub type Bcr2 = super::fsmc::BCR2;
    pub type Bcr3 = super::fsmc::BCR3;
    pub type Bcr4 = super::fsmc::BCR4;

    pub type Btr1 = super::fsmc::BTR1;
    pub type Btr2 = super::fsmc::BTR2;
    pub type Btr3 = super::fsmc::BTR3;
    pub type Btr4 = super::fsmc::BTR4;

    pub type Bwtr1 = super::fsmc::BWTR1;
    pub type Bwtr2 = super::fsmc::BWTR2;
    pub type Bwtr3 = super::fsmc::BWTR3;
    pub type Bwtr4 = super::fsmc::BWTR4;
}

pub use fmc_lcd::*;
pub use timing::{AccessMode, Timing, config_btr, config_bwtr};

