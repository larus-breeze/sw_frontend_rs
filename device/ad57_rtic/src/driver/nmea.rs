use stm32f4xx_hal::{
    dma::{Stream5, Stream7}, 
    gpio::Pin, pac::{self, DMA2, USART1}, 
    prelude::*, 
    rcc::Clocks, 
    serial::{config::Config, Serial}
};

const NMEA_BUF_SIZE: usize = 82;

#[allow(unused)]
static mut TX_BUFFER: [u8; NMEA_BUF_SIZE] = [0_u8; NMEA_BUF_SIZE];
static mut RX_BUFFER: [u8; NMEA_BUF_SIZE] = [0_u8; NMEA_BUF_SIZE];

pub struct NmeaTxRx {}

impl NmeaTxRx {
    pub fn new(
        usart1: USART1,
        _stream5: Stream5<DMA2>, // just to make clear, that this resource is used
        _stream7: Stream7<DMA2>, // just to ...
        tx: Pin<'A', 9>,
        rx: Pin<'A', 10>,
        clocks: &Clocks,
    ) -> (NmeaTx, NmeaRx)
    {
       let _serial: Serial<USART1> = usart1
            .serial(
                (tx, rx), 
                Config::default().baudrate(38400.bps()), 
                &clocks)
            .unwrap();
            
        unsafe {
            let usart1 = &(*pac::USART1::ptr());
            usart1.cr1.modify(|_, w| w.ue().disabled());
            usart1.cr3.modify(|_, w| w.dmat().set_bit()); // dma enable tx dma trigger
            usart1.cr3.modify(|_, w| w.dmar().set_bit()); // dma enable rx dma trigger
            usart1.cr1.modify(|_, w| w.ue().enabled());

            let dma2 = &(*pac::DMA2::ptr());
            // configure stream 3: USART3_TX
            dma2.st[7].par.write(|w| w.bits(0x4001_1004)); // dr data register
            dma2.st[7]
                .cr
                .write(|w| w.bits(0b_0000_1000_0000_0000__0000_0100_0101_0000));
                    // Bit 4     transfer complete ie
                    // Bit 7:6   0b01 memory to peripheral
                    // Bit 10    memory increment after transfer
                    // Bit 12:11 0b00 peripheral data size 8 Bit
                    // Bit 14:13 0b00 memory data size 8 Bit
                    // Bit 27:25 channel select 0b100 (4)

            // configure stream 1: USART3_RX
            dma2.st[5].par.write(|w| w.bits(0x4001_1004)); // dr data register
            dma2.st[5].m0ar.write(|w| w.bits(RX_BUFFER.as_ptr() as u32)); // dest ptr
            dma2.st[5]
                .ndtr
                .write(|w| w.ndt().bits(NMEA_BUF_SIZE as u16)); // buf len
            dma2.st[5]
                .cr
                .write(|w| w.bits(0b_0000_1000_0000_0000__0000_0101_0000_0000));
                    // Bit 7:6   0b00 peripheral to memory
                    // Bit 8     circular mode
                    // Bit 10    memory increment after transfer
                    // Bit 12:11 0b00 peripheral data size 8 Bit
                    // Bit 14:13 0b00 memory data size 8 Bit
                    // Bit 27:25 channel select 0b100 (4)
            dma2.st[5].cr.modify(|_, w| w.en().set_bit()); // enable dma
        }

        (NmeaTx { is_ready: true }, NmeaRx { tail: 0 })
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

                let dma2 = &(*pac::DMA2::ptr());
                dma2.st[7].cr.modify(|_, w| w.en().clear_bit()); // stop dma transfer
                dma2.st[7].m0ar.write(|w| w.bits(TX_BUFFER.as_ptr() as u32)); // src ptr
                dma2.st[7].ndtr.write(|w| w.ndt().bits(src.len() as u16)); // cnt dma
                dma2.st[7].cr.modify(|_, w| w.en().set_bit()); // enable dma
            }
            self.is_ready = false;
        } // else ... ignore failures
    }

    /// Check if last transfer is complete always before sending data
    pub fn ready(&mut self) -> bool {
        unsafe {
            let dma2 = &(*pac::DMA2::ptr());
            if dma2.hisr.read().tcif7().bit_is_set() {
                // clear transfer complete interrupt flag
                dma2.hifcr.write(|w| w.ctcif7().set_bit());
                self.is_ready = true;
            }
        }
        self.is_ready
    }
}

pub struct NmeaRx {
    tail: usize, // is set by reading routine
}

impl NmeaRx {
    /// Read data from rx buffer if available
    /// 
    /// This routine has to be called every 20 ms @ buffer size is 82 Bytes and bps is 38400.
    /// Calculation: 82 * 11 * 1s/38400 = 23.5 ms. The stm32f407 uart can not match a received
    /// byte, so we use polling to get the data.
    pub fn read(&mut self) -> Option<&[u8]> {
        // Check dma for new data
        let head = unsafe {
            let dma2 = &(*pac::DMA2::ptr());
            let cnt_down = dma2.st[5].ndtr.read().ndt().bits() as usize;
            NMEA_BUF_SIZE - cnt_down
        };

        if head == self.tail {
            None
        } else {
            let start = self.tail;
            if head > self.tail {
                self.tail = head;
                unsafe { Some(&RX_BUFFER[start..head]) }
            } else {
                // head < tail, ringbuffer wrap arround
                // the calling routine should call once more to get the rest
                self.tail = 0;
                unsafe { Some(&RX_BUFFER[start..NMEA_BUF_SIZE]) }
            }
        }
    }
}
