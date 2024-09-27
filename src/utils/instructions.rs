use super::registers::{RegisterNumber, RegisterStore, Registers};
// use super::mem_ops::;

#[repr(u16)]
pub enum Flag {
    Pos = 1 << 0, /* P */
    Zro = 1 << 1, /* Z */
    Neg = 1 << 2, /* N */
}

pub enum Trap {
    Getc = 0x20,  /* get character from keyboard, not echoed onto the terminal */
    Out = 0x21,   /* output a character */
    Puts = 0x22,  /* output a word string */
    In = 0x23,    /* get character from keyboard, echoed onto the terminal */
    PutSP = 0x24, /* output a byte string */
    Halt = 0x25,  /* halt the program */
}

impl TryFrom<u16> for Trap {
    type Error = ();

    fn try_from(v: u16) -> Result<Self, Self::Error> {
        match v {
            x if x == Trap::Getc as u16 => Ok(Trap::Getc),
            x if x == Trap::Out as u16 => Ok(Trap::Out),
            x if x == Trap::Puts as u16 => Ok(Trap::Puts),
            x if x == Trap::In as u16 => Ok(Trap::In),
            x if x == Trap::PutSP as u16 => Ok(Trap::PutSP),
            x if x == Trap::Halt as u16 => Ok(Trap::Halt),
            _ => Err(()),
        }
    }
}

fn sign_extend(x: u16, bit_count: u16) -> u16 {
    if (x >> (bit_count - 1)) & 1 == 1 {
        x | 0xFFFF << bit_count
    } else {
        x
    }
}

fn update_flags(value_register: u16, cond_register: &mut u16) {
    *cond_register = if value_register == 0 {
        Flag::Zro
    } else if value_register >> 15 == 1 {
        Flag::Neg
    } else {
        Flag::Pos
    } as u16
}

pub fn add(regs: &mut Registers, instr: u16) {
    /* destination register (DR) */
    let dr: RegisterNumber = ((instr >> 9) & 0x7).try_into().unwrap();
    /* first operand (SR1) */
    let sr1: RegisterNumber = ((instr >> 6) & 0x7).try_into().unwrap();
    /* whether we are in immediate mode */
    let imm_flag = (instr >> 5) & 0x1;

    let r1 = regs.get_register(sr1);
    if imm_flag == 1 {
        let imm5 = sign_extend(instr & 0x1F, 5);
        *regs.get_register_mut(dr) = r1 + imm5;
    } else {
        let sr2 = (instr & 0x7).try_into().unwrap();

        *regs.get_register_mut(dr) = r1 + regs.get_register(sr2);
    }

    update_flags(
        regs.get_register(dr),
        regs.get_register_mut(RegisterNumber::Cond),
    );
}

pub fn and(regs: &mut Registers, instr: u16) {
    /* destination register (DR) */
    let dr = ((instr >> 9) & 0x7).try_into().unwrap();
    /* first operand (SR1) */
    let sr1 = ((instr >> 6) & 0x7).try_into().unwrap();
    /* whether we are in immediate mode */
    let imm_flag = (instr >> 5) & 0x1;

    let r1 = regs.get_register(sr1);
    if imm_flag == 1 {
        let imm5 = sign_extend(instr & 0x1F, 5);
        *regs.get_register_mut(dr) = r1 + imm5;
    } else {
        let sr2 = (instr & 0x7).try_into().unwrap();

        *regs.get_register_mut(dr) = r1 & regs.get_register(sr2);
    }

    update_flags(
        regs.get_register(dr),
        regs.get_register_mut(RegisterNumber::Cond),
    );
}

pub fn not(regs: &mut Registers, instr: u16) {
    /* destination register (DR) */
    let dr = ((instr >> 9) & 0x7).try_into().unwrap();
    /* operand (SR) */
    let sr = ((instr >> 6) & 0x7).try_into().unwrap();

    *regs.get_register_mut(dr) = !regs.get_register(sr);
    update_flags(
        regs.get_register(dr),
        regs.get_register_mut(RegisterNumber::Cond),
    );
}

pub fn br(regs: &mut Registers, instr: u16) {
    let nzp = (instr >> 9) & 0x7;

    let pc_offset = sign_extend(instr & 0x1FF, 9);

    if nzp & regs.get_register(RegisterNumber::Cond) > 0 {
        *regs.get_register_mut(RegisterNumber::PC) += pc_offset;
    }
}

pub fn jmp(regs: &mut Registers, instr: u16) {
    let base_r = (instr >> 6) & 0x7;

    if base_r == 7 {
        *regs.get_register_mut(RegisterNumber::PC) = regs.get_register(RegisterNumber::R7);
    } else {
        *regs.get_register_mut(RegisterNumber::PC) = base_r;
    }
}

pub fn jsr(regs: &mut Registers, instr: u16) {
    let flag: bool = (instr >> 11) == 1;

    *regs.get_register_mut(RegisterNumber::R7) = regs.get_register(RegisterNumber::PC);

    if flag {
        let pc_offset = sign_extend(instr & 0x7FF, 11);

        *regs.get_register_mut(RegisterNumber::PC) += pc_offset;
    } else {
        let base_r = (instr >> 6) & 0x7;

        *regs.get_register_mut(RegisterNumber::PC) = base_r;
    }
}

pub fn ld(regs: &mut Registers, instr: u16) {
    /* destination register (DR) */
    let dr = ((instr >> 9) & 0x7).try_into().unwrap();
    let pc_offset = sign_extend(instr & 0x1FF, 9);

    *regs.get_register_mut(dr) = mem_read(regs.get_register(RegisterNumber::PC) + pc_offset);
}

pub fn ldi(regs: &mut Registers, instr: u16) {
    let dr = ((instr >> 9) & 0x7).try_into().unwrap();
    let pc_offset = sign_extend(instr & 0x1FF, 9); // 0x1FF = 0b111111111

    let r0 = *regs.get_register_mut(dr);
    r0 = mem_read(mem_read(regs.get_register(RegisterNumber::PC) + pc_offset));

    update_flags(
        regs.get_register(dr),
        regs.get_register_mut(RegisterNumber::Cond),
    );
}

pub fn ldr(regs: &mut Registers, instr: u16) {
    let dr = ((instr >> 9) & 0x7).try_into().unwrap();
    let base_r = (instr >> 6) & 0x7;
    let pc_offset = sign_extend(instr & 0x3F, 6);

    *regs.get_register_mut(dr) = mem_read(base_r + pc_offset);

    update_flags(
        regs.get_register(dr),
        regs.get_register_mut(RegisterNumber::Cond),
    );
}

pub fn lea(regs: &mut Registers, instr: u16) {
    let dr = ((instr >> 9) & 0x7).try_into().unwrap();
    let pc_offset = sign_extend(instr & 0x1FF, 9);

    *regs.get_register_mut(dr) = regs.get_register(RegisterNumber::PC) + pc_offset;

    update_flags(
        regs.get_register(dr),
        regs.get_register_mut(RegisterNumber::Cond),
    );
}

pub fn st(regs: &mut Registers, instr: u16) {
    // TODO after mem_write implementation
}

pub fn sti(regs: &mut Registers, instr: u16) {
    // TODO after mem_write implementation
}

pub fn str(regs: &mut Registers, instr: u16) {
    // TODO after mem_write implementation
}

pub fn trap(regs: &mut Registers, instr: u16) {
    match (instr & 0xFF).try_into().unwrap() {
        Trap::Puts => {}
        _ => todo!(),
    }
}
