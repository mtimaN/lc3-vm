use std::{env, error::Error};

const MEMORY_MAX: usize = 1 << 16;
type Memory = [u16; MEMORY_MAX];
    
enum OpCode {
    Br = 0, /* branch */
    Add,    /* add  */
    Ld,     /* load */
    St,     /* store */
    Jsr,    /* jump register */
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

#[derive(Default)]
struct Registers {
    r0: u16,
    r1: u16,
    r2: u16,
    r3: u16,
    r4: u16,
    r5: u16,
    r6: u16,
    r7: u16,
    pc: u16, /* program counter */
    cond: u16,
    count: u16,
}

trait RegisterStore {
    fn get_register(&self, position: u16) -> Result<u16, ()>;
    fn get_register_mut(&mut self, position: u16) -> Result<&mut u16, ()>;
}

impl RegisterStore for Registers {
    fn get_register(&self, position: u16) -> Result<u16, ()> {
        match position {
            0 => Ok(self.r0),
            1 => Ok(self.r1),
            2 => Ok(self.r2),
            3 => Ok(self.r3),
            4 => Ok(self.r4),
            5 => Ok(self.r5),
            6 => Ok(self.r6),
            7 => Ok(self.r7),
            8 => Ok(self.pc),
            9 => Ok(self.cond),
            10 => Ok(self.count),
            _ => Err(()),
        }
    }
    fn get_register_mut(&mut self, position: u16) -> Result<&mut u16, ()> {
        match position {
            0 => Ok(&mut self.r0),
            1 => Ok(&mut self.r1),
            2 => Ok(&mut self.r2),
            3 => Ok(&mut self.r3),
            4 => Ok(&mut self.r4),
            5 => Ok(&mut self.r5),
            6 => Ok(&mut self.r6),
            7 => Ok(&mut self.r7),
            8 => Ok(&mut self.pc),
            9 => Ok(&mut self.cond),
            10 => Ok(&mut self.count),
            _ => Err(()),
        }
    }
}

#[repr(u16)]
enum Flag {
    Pos = 1 << 0, /* P */
    Zro = 1 << 1, /* Z */
    Neg = 1 << 2, /* N */
}

fn sign_extend(x: u16, bit_count: u16) -> u16
{
    if (x >> (bit_count - 1)) & 1 == 1 {
        x | 0xFFFF << bit_count
    } else {
        x
    }
}

fn update_flags(value_register: u16, cond_register: &mut u16)
{
    *cond_register = if value_register == 0
    {
        Flag::Zro
    } else if value_register >> 15 == 1 {
        Flag::Neg
    } else {
        Flag::Pos
    } as u16
}

fn add(regs: &mut Registers, instr: u16) {
    /* destination register (DR) */
    let dr = (instr >> 9) & 0x7;
    /* first operand (SR1) */
    let sr1 = (instr >> 6) & 0x7;
    /* whether we are in immediate mode */
    let imm_flag = (instr >> 5) & 0x1;

    let r1 = regs.get_register(sr1).unwrap();
    if imm_flag == 1
    {
        let imm5 = sign_extend(instr & 0x1F, 5);
        *regs.get_register_mut(dr).unwrap() = r1 + imm5;
    }
    else
    {
        let sr2 = instr & 0x7;

        *regs.get_register_mut(dr).unwrap() = r1 + regs.get_register(sr2).unwrap();
    }

    update_flags(regs.get_register(dr).unwrap(), &mut regs.cond);
}

fn and(regs: &mut Registers, instr: u16) {
    /* destination register (DR) */
    let dr = (instr >> 9) & 0x7;
    /* first operand (SR1) */
    let sr1 = (instr >> 6) & 0x7;
    /* whether we are in immediate mode */
    let imm_flag = (instr >> 5) & 0x1;

    let r1 = regs.get_register(sr1).unwrap();
    if imm_flag == 1
    {
        let imm5 = sign_extend(instr & 0x1F, 5);
        *regs.get_register_mut(dr).unwrap() = r1 + imm5;
    }
    else
    {
        let sr2 = instr & 0x7;

        *regs.get_register_mut(dr).unwrap() = r1 & regs.get_register(sr2).unwrap();
    }

    update_flags(regs.get_register(dr).unwrap(), &mut regs.cond);
}

fn not(regs: &mut Registers, instr: u16) {

}

fn ldi(regs: &mut Registers, instr: u16) {
    let dr = (instr >> 9) & 0x7;
    let pc_offset = sign_extend(instr & 0x1FF, 9);

    let r0 = *regs.get_register_mut(dr).unwrap();
    r0 = mem_read(mem_read(regs.pc + pc_offset));

    update_flags(r0, &mut regs.cond);
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
    regs.cond = Flag::Zro as u16;

    /* set the PC to starting position */
    /* 0x3000 is the default */
    const PC_START: u16 = 0x3000;
    regs.pc = PC_START;

    let running: bool = true;
    while running {
        /* FETCH */
        let instr: u16 = mem_read(regs.pc);
        regs.pc += 1;
        let op: OpCode = instr >> 12;

        match op {
            OpCode::Add => add(&mut regs, instr),
            OpCode::And  => and(&mut regs, instr),
            OpCode::Not  => not(&mut regs, instr),
            OP_BR   => ,
            OP_JMP  => ,
            OP_JSR  => ,
            OP_LD   => ,
            OpCode::Ldi  => ldi(&mut regs, instr),
            OP_LDR  => ,
            OP_LEA  => ,
            OP_ST   => ,
            OP_STI  => ,
            OP_STR  => ,
            OP_TRAP => ,
            OpCode::Rti | OpCode::Res | _ => return Err(()),
        }
    }

    // todo!("Shutdown");
    Ok(())
}
