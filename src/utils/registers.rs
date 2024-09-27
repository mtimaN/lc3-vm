use std::convert::TryFrom;

#[repr(u16)]
#[derive(Clone, Copy)]
pub enum RegisterNumber {
    R0 = 0,
    R1,
    R2,
    R3,
    R4,
    R5,
    R6,
    R7,
    PC,
    Cond,
    Count,
}

impl TryFrom<u16> for RegisterNumber {
    type Error = ();

    fn try_from(v: u16) -> Result<Self, Self::Error> {
        match v {
            x if x == RegisterNumber::R0 as u16 => Ok(RegisterNumber::R0),
            x if x == RegisterNumber::R1 as u16 => Ok(RegisterNumber::R1),
            x if x == RegisterNumber::R2 as u16 => Ok(RegisterNumber::R2),
            x if x == RegisterNumber::R3 as u16 => Ok(RegisterNumber::R3),
            x if x == RegisterNumber::R4 as u16 => Ok(RegisterNumber::R4),
            x if x == RegisterNumber::R5 as u16 => Ok(RegisterNumber::R5),
            x if x == RegisterNumber::R6 as u16 => Ok(RegisterNumber::R6),
            x if x == RegisterNumber::R7 as u16 => Ok(RegisterNumber::R7),
            x if x == RegisterNumber::PC as u16 => Ok(RegisterNumber::PC),
            x if x == RegisterNumber::Cond as u16 => Ok(RegisterNumber::Cond),
            x if x == RegisterNumber::Count as u16 => Ok(RegisterNumber::Count),
            _ => Err(()),
        }
    }
}

#[derive(Default)]
pub struct Registers {
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

pub trait RegisterStore {
    fn get_register(&self, position: RegisterNumber) -> u16;
    fn get_register_mut(&mut self, position: RegisterNumber) -> &mut u16;
}

impl RegisterStore for Registers {
    fn get_register(&self, position: RegisterNumber) -> u16 {
        match position {
            RegisterNumber::R0 => self.r0,
            RegisterNumber::R1 => self.r1,
            RegisterNumber::R2 => self.r2,
            RegisterNumber::R3 => self.r3,
            RegisterNumber::R4 => self.r4,
            RegisterNumber::R5 => self.r5,
            RegisterNumber::R6 => self.r6,
            RegisterNumber::R7 => self.r7,
            RegisterNumber::PC => self.pc,
            RegisterNumber::Cond => self.cond,
            RegisterNumber::Count => self.count,
        }
    }
    fn get_register_mut(&mut self, position: RegisterNumber) -> &mut u16 {
        match position {
            RegisterNumber::R0 => &mut self.r0,
            RegisterNumber::R1 => &mut self.r1,
            RegisterNumber::R2 => &mut self.r2,
            RegisterNumber::R3 => &mut self.r3,
            RegisterNumber::R4 => &mut self.r4,
            RegisterNumber::R5 => &mut self.r5,
            RegisterNumber::R6 => &mut self.r6,
            RegisterNumber::R7 => &mut self.r7,
            RegisterNumber::PC => &mut self.pc,
            RegisterNumber::Cond => &mut self.cond,
            RegisterNumber::Count => &mut self.count,
        }
    }
}
