use std::env;

use utils::registers::RegisterStore;

use crate::utils::instructions;
use crate::utils::registers::RegisterNumber;
use crate::utils::registers::Registers;

pub mod utils;

const MEMORY_MAX: usize = 1 << 16;
type Memory = [u16; MEMORY_MAX];

enum OpCode {
    Br = 0, /* branch */
    Add,    /* add  */
    Ld,     /* load */
    St,     /* store */
    Jsr,    /* jump to subroutine */
    And,    /* bitwise and */
    Ldr,    /* load register */
    Str,    /* store register */
    Rti,    /* unused */
    Not,    /* bitwise not */
    Ldi,    /* load indirect */
    Sti,    /* store indirect */
    Jmp,    /* jump */
    Res,    /* reserved (unused) */
    Lea,    /* load effective address */
    Trap,   /* execute trap */
}

fn main() -> Result<(), u8> {
    let regs: Registers = Registers::default();
    let memory: Memory;

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        /* show usage string */
        println!("lc3 [image-file1] ...")
    }

    for arg in args {
        if !read_image(arg) {
            println!("failed to load")
        }
    }

    // todo!("Setup");

    /* since exactly one condition flag should be set at any given time, set the Z flag */
    *regs.get_register_mut(RegisterNumber::Cond) = instructions::Flag::Zro as u16;

    /* set the PC to starting position */
    /* 0x3000 is the default */
    const PC_START: u16 = 0x3000;
    *regs.get_register_mut(RegisterNumber::PC) = PC_START;

    let running: bool = true;
    while running {
        /* FETCH */
        let pc = regs.get_register_mut(RegisterNumber::PC);
        let instr: u16 = mem_read(*pc);
        *pc += 1;
        let op: OpCode = instr >> 12;

        match op {
            OpCode::Add => instructions::add(&mut regs, instr),
            OpCode::And => instructions::and(&mut regs, instr),
            OpCode::Not => instructions::not(&mut regs, instr),
            OpCode::Br => instructions::br(&mut regs, instr),
            OpCode::Jmp => instructions::jmp(&mut regs, instr),
            OpCode::Jsr => instructions::jsr(&mut regs, instr),
            OpCode::Ld => instructions::ld(&mut regs, instr),
            OpCode::Ldi => instructions::ldi(&mut regs, instr),
            OpCode::Ldr => instructions::ldr(&mut regs, instr),
            OpCode::Lea => instructions::lea(&mut regs, instr),
            OpCode::St => instructions::st(&mut regs, instr),
            OpCode::Sti => instructions::sti(&mut regs, instr),
            OpCode::Str => instructions::str(&mut regs, instr),
            OpCode::Trap => instructions::trap(&mut regs, instr),
            OpCode::Rti | OpCode::Res | _ => return Err(()),
        }
    }

    // todo!("Shutdown");
    Ok(())
}
