use std::fs::File;
use std::io::{BufReader, Error, Read};
use std::thread::sleep;
use std::time;

use byteorder::{BigEndian, ReadBytesExt};

use crate::utils::unix_input_buffering;
use crate::Memory;

#[repr(u16)]
enum MemMappedRegs {
    KeyboardStatus = 0xFE00, /* keyboard status */
    KeyboardData = 0xFE02,   /* keyboard data */
}

fn read_image_file(file: &mut File, memory: &mut Memory) -> Result<(), Error> {
    let mut reader = BufReader::new(file);
    let origin = reader
        .read_u16::<BigEndian>()
        .expect("fail to read image size");

    let mut next_address: usize = origin as usize;
    while let Ok(bytes) = reader.read_u16::<BigEndian>() {
        memory[next_address] = bytes;
        next_address += 1;
    }
    dbg!("locked and loaded");
    sleep(time::Duration::from_secs(2));
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
    if address == MemMappedRegs::KeyboardStatus as u16 {
        if unix_input_buffering::check_key() {
            memory[MemMappedRegs::KeyboardStatus as usize] = 1 << 15;
            memory[MemMappedRegs::KeyboardData as usize] = std::io::stdin()
                .bytes()
                .next()
                .and_then(|result| result.ok())
                .map(|byte| byte as u16)
                .expect("mem_read failed");
        } else {
            memory[MemMappedRegs::KeyboardStatus as usize] = 0;
        }
    }
    return memory[address as usize];
}
