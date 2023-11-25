#[allow(dead_code)]
mod concat;
#[allow(dead_code)]
mod file_io;
#[allow(dead_code)]
mod keyboard;
#[allow(dead_code)]
mod r61580;

pub use concat::*;
pub use file_io::*;
pub use keyboard::*;
pub use r61580::*;

/*
#[allow(dead_code)]
fn show_line(block_no: u32, line_idx: usize, block: &[u8; 512]) {
    let ofs = line_idx*16;
    let adr = block_no*512 + ofs as u32;
    let mut ascii = [' '; 16];
    rprint!("{:08X}  ", adr);

    for idx in ofs..ofs+16 {
        //ascii[idx % 16] = char::from_u8(block[idx]);
        let b = block[idx as usize];
        let c = if (b>31) & (b<128) {
            unsafe { char::from_u32_unchecked(b as u32) }
        } else {
            '.'
        };
        ascii[(idx as usize) % 16] = c;

        match idx % 16 {
            4 | 12 => rprint!(" "),
            8 => rprint!("| "),
            _ => (),
        }
        rprint!("{:02x} ", b);
   }
   rprint!(" ");
   for idx2 in 0..16 {
       rprint!("{}", ascii[idx2]);
   }
   rprintln!();
   delay_ms(1);
}*/
