use std::fs::File;
use std::io::{BufReader, Error, Read};

use byteorder::{BigEndian, ReadBytesExt};

use crate::{Memory, MEMORY_MAX};

// #[repr(u16)]
// enum MemMappedRegs {
//     KeyboardStatus = 0xFE00, /* keyboard status */
//     KeyboardData = 0xFE02,   /* keyboard data */
// }

fn read_image_file(file: &mut File, memory: &mut Memory) -> Result<(), Error> {
    let mut reader = BufReader::new(file);
    let mut buffer = [0_u8; std::mem::size_of::<u16>()];
    reader.read_exact(&mut buffer)?;
    let origin = u16::from_be_bytes(buffer);

    reader.read_u16_into::<BigEndian>(&mut memory[origin as usize..MEMORY_MAX])?;

    Ok(())
}

pub fn read_image(image_path: String, memory: &mut Memory) -> Result<(), Error> {
    let mut file = File::open(image_path)?;

    read_image_file(&mut file, memory)?;

    Ok(())
}

pub fn mem_write(address: usize, value: u16, memory: &mut Memory) {
    memory[address] = value;
}

pub fn mem_read(address: u16, memory: &mut Memory) -> u16 {
    // if address == MemMappedRegs::KeyboardStatus as u16 {
    //     if (check_key()) {
    //         memory[MemMappedRegs::KeyboardStatus as usize] = 1 << 15;
    //         memory[MemMappedRegs::KeyboardData as usize] = getchar();
    //     } else {
    //         memory[MemMappedRegs::KeyboardStatus as usize] = 0;
    //     }
    // }
    return memory[address as usize];
}
