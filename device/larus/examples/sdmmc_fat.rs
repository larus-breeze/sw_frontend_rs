#![no_main]
#![no_std]
mod driver;

use defmt::assert_eq;
use defmt::*;
use defmt_rtt as _;
use panic_rtt_target as _;

use stm32h7xx_hal::gpio::Speed;

use {
    embedded_sdmmc::{Controller, Mode, VolumeIdx},
    stm32h7xx_hal::sdmmc::{SdCard, Sdmmc},
    stm32h7xx_hal::{pac, prelude::*, rcc},
};

// This is just a placeholder TimeSource. In a real world application
// one would probably use the RTC to provide time.
pub struct TimeSource;

impl embedded_sdmmc::TimeSource for TimeSource {
    fn get_timestamp(&self) -> embedded_sdmmc::Timestamp {
        embedded_sdmmc::Timestamp {
            year_since_1970: 0,
            zero_indexed_month: 0,
            zero_indexed_day: 0,
            hours: 0,
            minutes: 0,
            seconds: 0,
        }
    }
}

#[cortex_m_rt::entry]
unsafe fn main() -> ! {
    // Get peripherals
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();

    // Constrain and Freeze power
    let pwr = dp.PWR.constrain();
    let pwrcfg = pwr.freeze();

    // Constrain and Freeze clock
    let ccdr = dp
        .RCC
        .constrain()
        .sys_ck(200.MHz())
        .pll1_strategy(rcc::PllConfigStrategy::Iterative)
        .pll1_q_ck(100.MHz())
        .pll2_strategy(rcc::PllConfigStrategy::Iterative)
        .pll3_strategy(rcc::PllConfigStrategy::Iterative)
        .freeze(pwrcfg, &dp.SYSCFG);

    // Get the delay provider.
    let mut delay = cp.SYST.delay(ccdr.clocks);

    let gpioc = dp.GPIOC.split(ccdr.peripheral.GPIOC);
    let gpiod = dp.GPIOD.split(ccdr.peripheral.GPIOD);
    let gpioe = dp.GPIOE.split(ccdr.peripheral.GPIOE);

    // Card detect.  Activate Pull Down.    Level is high in case of an inserted uSD card
    let detect = gpioe.pe3.into_pull_down_input();

    // SDMMC pins
    let clk = gpioc
        .pc12
        .into_alternate()
        .internal_pull_up(false)
        .speed(Speed::VeryHigh);
    let cmd = gpiod
        .pd2
        .into_alternate()
        .internal_pull_up(true)
        .speed(Speed::VeryHigh);
    let d0 = gpioc
        .pc8
        .into_alternate()
        .internal_pull_up(true)
        .speed(Speed::VeryHigh);
    let d1 = gpioc
        .pc9
        .into_alternate()
        .internal_pull_up(true)
        .speed(Speed::VeryHigh);
    let d2 = gpioc
        .pc10
        .into_alternate()
        .internal_pull_up(true)
        .speed(Speed::VeryHigh);
    let d3 = gpioc
        .pc11
        .into_alternate()
        .internal_pull_up(true)
        .speed(Speed::VeryHigh);

    // Create SDMMC
    let mut sd: Sdmmc<_, SdCard> = dp.SDMMC1.sdmmc(
        (clk, cmd, d0, d1, d2, d3),
        ccdr.peripheral.SDMMC1,
        &ccdr.clocks,
    );

    while detect.is_low() {
        trace!("Waiting for card detection switch...");
        delay.delay_ms(1000u32);
    }

    // Loop until we have a card
    loop {
        // On most development boards this can be increased up to 50MHz. We choose a
        // lower frequency here so that it should work even with flying leads
        // connected to a SD card breakout.
        match sd.init(2.MHz()) {
            Ok(_) => break,
            Err(_err) => {
                trace!("Init err");
            }
        }

        trace!("Waiting for card...");

        delay.delay_ms(1000u32);
    }

    // See https://github.com/rust-embedded-community/embedded-sdmmc-rs for docs
    // and more examples

    trace!("Initialize file system manager");
    let mut sd_fatfs = Controller::new(sd.sdmmc_block_device(), TimeSource);
    let mut sd_fatfs_volume = sd_fatfs.get_volume(VolumeIdx(0)).unwrap();
    let sd_fatfs_root_dir = sd_fatfs.open_root_dir(&sd_fatfs_volume).unwrap();

    trace!("List all the directories and their info");
    sd_fatfs
        .iterate_dir(&sd_fatfs_volume, &sd_fatfs_root_dir, |_entry| {
            trace!("Listing received");
        })
        .unwrap();

    const WRITE_BUFFER: [u8; 8 * 1024] = [b'B'; 8 * 1024];

    for (filename, length) in &[("small.txt", 8), ("big.txt", WRITE_BUFFER.len())] {
        trace!("Open file {:?}", filename);
        let mut file = sd_fatfs
            .open_file_in_dir(
                &mut sd_fatfs_volume,
                &sd_fatfs_root_dir,
                filename,
                Mode::ReadWriteCreateOrTruncate,
            )
            .unwrap();

        trace!("Write {:?} characters in it", length);
        sd_fatfs
            .write(&mut sd_fatfs_volume, &mut file, &WRITE_BUFFER[0..*length])
            .unwrap();

        trace!("Read it back and confirm it contains the expected content");
        file.seek_from_start(0).unwrap();
        while !file.eof() {
            let mut buffer = [0u8; 1024];
            let num_read = sd_fatfs
                .read(&sd_fatfs_volume, &mut file, &mut buffer)
                .unwrap();
            for b in &buffer[0..num_read] {
                assert_eq!(*b as char, 'B');
            }
        }

        sd_fatfs.close_file(&sd_fatfs_volume, file).unwrap();
    }

    trace!("Test successfully finished");

    sd_fatfs.close_dir(&sd_fatfs_volume, sd_fatfs_root_dir);

    loop {
        cortex_m::asm::nop()
    }
}
