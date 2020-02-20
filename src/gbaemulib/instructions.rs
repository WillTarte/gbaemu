use crate::registers;

pub enum Instruction {
    ADD(TargetRegister),
}

pub enum TargetRegister {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
}

impl Instruction {
    fn add(register: u8, val: u8) -> u8 {
        let (new_val, did_overflow) = register.overflowing_add(val);
        //TODO: set flags
        new_val
    }
}