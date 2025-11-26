use corelib::*;
use std::io::{Read, Write};

const FILE_NAME: &str = "eeprom.bin";

pub struct Storage {
    data: [u8; 8192],
}

impl Storage {
    pub fn new(is_unique: fn(PersistenceId)->bool) -> Result<Eeprom<Storage>, CoreError> {
        let mut data = [0_u8; eeprom::SIZE as usize];
        if let Ok(mut f) = std::fs::File::open(FILE_NAME) {
            f.read_exact(&mut data).unwrap()
        }
        let storage = Storage { data };
        Eeprom::new(storage, is_unique)
    }
}

impl EepromTrait for Storage {
    fn write_byte(&mut self, address: u32, data: u8) -> Result<(), CoreError> {
        
        #[cfg(feature = "trace_eeprom")]
        println!("write_byte() 0x{:04x}: 0x{:02x}", address, data);

        if address >= eeprom::SIZE {
            return Err(CoreError::OutOfRange);
        }
        self.data[address as usize] = data;
        let mut f = std::fs::File::create(FILE_NAME).unwrap();
        f.write_all(&self.data).unwrap();
        Ok(())
    }

    fn write_page(&mut self, address: u32, data: &[u8]) -> Result<(), CoreError> {
        
        #[cfg(feature = "trace_eeprom")]
        println!("write_page() 0x{:04x}: 0x{}", address, hex::encode(data));

        let start = address as usize;
        let end = address as usize + data.len();
        if end as u32 > eeprom::SIZE {
            return Err(CoreError::OutOfRange);
        }
        self.data[start..end].copy_from_slice(data);
        let mut f = std::fs::File::create(FILE_NAME).unwrap();
        f.write_all(&self.data).unwrap();
        Ok(())
    }

    fn read_byte(&mut self, address: u32) -> Result<u8, CoreError> {
        if address >= eeprom::SIZE {
            return Err(CoreError::OutOfRange);
        }
        let r = self.data[address as usize];

        #[cfg(feature = "trace_eeprom")]
        println!("read_byte() 0x{:04x}: 0x{:02x}", address, r);

        Ok(r)
    }

    fn read_data(&mut self, address: u32, data: &mut [u8]) -> Result<(), CoreError> {
        let start = address as usize;
        let end = address as usize + data.len();
        if end as u32 > eeprom::SIZE {
            return Err(CoreError::OutOfRange);
        }
        data.copy_from_slice(&self.data[start..end]);
        
        #[cfg(feature = "trace_eeprom")]
        println!("read_data() 0x{:04x}: 0x{}", address, hex::encode(data));
        
        Ok(())
    }
}
