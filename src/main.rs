use std::env;
use std::io::{Error, ErrorKind};
use std::process;

use utils::instructions;
use utils::instructions::OpCode;
use utils::mem_ops::{mem_read, read_image};
use utils::registers::RegisterNumber;
use utils::registers::RegisterStore;
use utils::registers::Registers;
use utils::unix_input_buffering::{disable_input_buffering, restore_input_buffering};

pub mod utils;

const MEMORY_MAX: usize = 1 << 16;
type Memory = [u16; MEMORY_MAX];

fn main() -> Result<(), Error> {
    let original_tio = disable_input_buffering();
    if original_tio.is_err() {
        return Err(Error::new(ErrorKind::NotFound, "Original tio not found"));
    }
    let original_tio = original_tio.unwrap();

    ctrlc::set_handler(move || {
        println!("received Ctrl+C!");
        restore_input_buffering(original_tio).expect("restore failed");
        process::exit(0);
    })
    .expect("Error setting Ctrl-C handler");
    let mut regs: Registers = Registers::default();
    let mut memory: Memory = [0; MEMORY_MAX];

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        /* show usage string */
        println!("lc3 [image-file1] ...")
    }

    for arg in args.into_iter().skip(1) {
        dbg!(&arg);
        read_image(arg, &mut memory)?;
    }

    /* since exactly one condition flag should be set at any given time, set the Z flag */
    *regs.get_register_mut(RegisterNumber::Cond) = instructions::Flag::Zro as u16;

    /* set the PC to starting position */
    /* 0x3000 is the default */
    const PC_START: u16 = 0x3000;
    *regs.get_register_mut(RegisterNumber::PC) = PC_START;

    let mut running: bool = true;
    while running {
        /* FETCH */
        let pc = regs.get_register_mut(RegisterNumber::PC);
        let instr: u16 = mem_read(*pc, &mut memory);
        *pc += 1;
        let op: OpCode = (instr >> 12).try_into().unwrap();

        match op {
            OpCode::Add => instructions::add(&mut regs, instr),
            OpCode::And => instructions::and(&mut regs, instr),
            OpCode::Not => instructions::not(&mut regs, instr),
            OpCode::Br => instructions::br(&mut regs, instr),
            OpCode::Jmp => instructions::jmp(&mut regs, instr),
            OpCode::Jsr => instructions::jsr(&mut regs, instr),
            OpCode::Ld => instructions::ld(&mut regs, instr, &mut memory),
            OpCode::Ldi => instructions::ldi(&mut regs, instr, &mut memory),
            OpCode::Ldr => instructions::ldr(&mut regs, instr, &mut memory),
            OpCode::Lea => instructions::lea(&mut regs, instr),
            OpCode::St => instructions::st(&mut regs, instr, &mut memory),
            OpCode::Sti => instructions::sti(&mut regs, instr, &mut memory),
            OpCode::Str => instructions::str(&mut regs, instr, &mut memory),
            OpCode::Trap => instructions::trap(&mut regs, instr, &mut memory, &mut running),
            OpCode::Rti | OpCode::Res => {
                return Err(Error::new(ErrorKind::Other, "Invalid Opcode"))
            }
        }
    }

    restore_input_buffering(original_tio).expect("restore failed");
    Ok(())
}
