use stm32h7xx_hal::{
    dma::dma::{Stream1, Stream2},
    gpio::Pin,
    pac::{self, DMA1, USART1},
    prelude::*,
    rcc::{rec::Usart1, CoreClocks},
};

const NMEA_BUF_SIZE: usize = 82;

#[link_section = ".axisram.buffers"]
static mut TX_BUFFER: [u8; NMEA_BUF_SIZE] = [0_u8; NMEA_BUF_SIZE];

#[link_section = ".axisram.buffers"]
static mut RX_BUFFER: [u8; NMEA_BUF_SIZE] = [0_u8; NMEA_BUF_SIZE];

pub struct NmeaTxRx {}

impl NmeaTxRx {
    /// Init NMEA struct
    pub fn new(
        _dma1_stream1: Stream1<DMA1>,
        _dma1_stream2: Stream2<DMA1>,
        tx: Pin<'A', 9>,
        rx: Pin<'B', 15>,
        usart1: USART1,
        prec: Usart1,
        clocks: &CoreClocks,
    ) -> (NmeaTx, NmeaRx) {
        let rx = rx.into_alternate();
        let tx = tx.into_alternate();

        // init usart with correct settings
        let _serial = usart1.serial((tx, rx), 38_400.bps(), prec, clocks).unwrap();

        unsafe {
            // configure usart1
            let usart1 = &(*pac::USART1::ptr());
            usart1.cr1.modify(|_, w| w.ue().disabled());
            usart1.cr3.modify(|_, w| w.dmat().set_bit()); // dma enable tx dma trigger
            usart1.cr3.modify(|_, w| w.dmar().set_bit()); // dma enable rx dma trigger
            usart1.cr2.write(|w| w.add().bits(b'\r')); // match char
            usart1.cr1.modify(|_, w| w.cmie().set_bit()); // generate IR on match
            usart1.cr1.modify(|_, w| w.ue().enabled());

            let dmamux1 = &(*pac::DMAMUX1::ptr());
            dmamux1.ccr[1].write(|w| w.dmareq_id().bits(42)); // channel 1: usart1_tx_dma
            dmamux1.ccr[2].write(|w| w.dmareq_id().bits(41)); // channel 2: usart1_rx_dma

            // configure dma1 stream 1
            let dma1 = &(*pac::DMA1::ptr());
            dma1.st[1].par.write(|w| w.bits(0x4001_1028)); // tdr transmit data register
                                                           // Bit 4     transfer complete ie
                                                           // Bit 7:6   0b01 memory to peripheral
                                                           // Bit 10    memory increment after transfer
                                                           // Bit 12:11 0b00 peripheral data size 8 Bit
                                                           // Bit 14:13 0b00 memory data size 8 Bit
                                                           // Bit 20    Alternative DMA Channel protocol errata sheet
                                                           //           DMA stream locked when transferring data to/from USART
            dma1.st[1]
                .cr
                .write(|w| w.bits(0b_0001_0000_0000_0100_0101_0000));

            // configure dma1 stream 2
            dma1.st[2].par.write(|w| w.bits(0x4001_1024)); // rdr receive data register
            dma1.st[2]
                .m0ar
                .write(|w| w.bits(&raw const RX_BUFFER as u32)); // dest ptr
            dma1.st[2]
                .ndtr
                .write(|w| w.ndt().bits(NMEA_BUF_SIZE as u16)); // buf len
                                                                // Bit 7:6   0b00 peripheral to memory
                                                                // Bit 8     circular mode
                                                                // Bit 10    memory increment after transfer
                                                                // Bit 12:11 0b00 peripheral data size 8 Bit
                                                                // Bit 14:13 0b00 memory data size 8 Bit
                                                                // Bit 20    Alternative DMA Channel protocol errata sheet
                                                                //           DMA stream locked when transferring data to/from USART
            dma1.st[2]
                .cr
                .write(|w| w.bits(0b_0001_0000_0000_0101_0000_0000));
            dma1.st[2].cr.modify(|_, w| w.en().set_bit()); // enable dma
        }

        (NmeaTx { is_ready: true }, NmeaRx { head: 0, tail: 0 })
    }
}

pub struct NmeaTx {
    is_ready: bool,
}

impl NmeaTx {
    /// Send NMEA data through serial interface
    pub fn send(&mut self, src: &[u8]) {
        if src.len() > 0 && self.is_ready {
            unsafe {
                TX_BUFFER[..src.len()].copy_from_slice(src);
                let dma1 = &(*pac::DMA1::ptr());

                dma1.st[1].cr.modify(|_, w| w.en().clear_bit()); // stop dma transfer
                dma1.st[1]
                    .m0ar
                    .write(|w| w.bits(&raw const TX_BUFFER as u32)); // src ptr
                dma1.st[1].ndtr.write(|w| w.ndt().bits(src.len() as u16)); // cnt dma
                                                                           //dma1.lifcr.write(|w| w.ctcif1().set_bit()); // clear transfer complete interrupt flag
                dma1.st[1].cr.modify(|_, w| w.en().set_bit()); // enable dma
            }
            self.is_ready = false;
        }
    }

    /// Check if last transfer is complete always befor sending data
    pub fn ready(&mut self) -> bool {
        unsafe {
            let dma1 = &(*pac::DMA1::ptr());
            if dma1.lisr.read().tcif1().bit_is_set() {
                // clear transfer complete interrupt flag
                dma1.lifcr.write(|w| w.ctcif1().set_bit());
                self.is_ready = true;
            }
        }
        self.is_ready
    }
}

pub struct NmeaRx {
    head: usize, // is set by the interrupt service routine
    tail: usize, // is set by reading routine
}

impl NmeaRx {
    /// After intterupt clear interrupt flag first
    pub fn on_interrupt(&mut self) {
        self.head = unsafe {
            let usart1 = &(*pac::USART1::ptr());
            usart1.icr.write(|w| w.cmcf().set_bit()); // char match clear flag

            let dma1 = &(*pac::DMA1::ptr());
            let cnt_down = dma1.st[2].ndtr.read().ndt().bits() as usize;
            NMEA_BUF_SIZE - cnt_down
        }
    }

    /// Read data from rx buffer if available
    ///
    ///
    pub fn read(&mut self) -> Option<&[u8]> {
        if self.head == self.tail {
            None
        } else {
            let start = self.tail;
            if self.head > self.tail {
                self.tail = self.head;
                unsafe { Some(&RX_BUFFER[start..self.head]) }
            } else {
                // head < tail
                self.tail = 0;
                unsafe { Some(&RX_BUFFER[start..NMEA_BUF_SIZE]) }
            }
        }
    }
}
