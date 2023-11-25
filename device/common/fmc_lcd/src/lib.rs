#![no_std]

mod fmc_lcd;
mod timing;

pub use fmc_lcd::*;
pub use timing::{AccessMode, Timing, config_btr, config_bwtr};

pub use stm32f4xx_hal as hal;
pub use hal::pac::{FSMC, fsmc};

#[allow(unused)]
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
