enum_from_primitive! {
    #[derive(Debug, PartialEq)]
    pub enum Register {
        R_R0 = 0,
        R_R1,
        R_R2,
        R_R3,
        R_R4,
        R_R5,
        R_R6,
        R_R7,
        R_PC, // program counter
        R_COND,
        R_COUNT,
    }
}

pub const PC_START: u16 = 0x3000;

pub struct RegisterStorage {
    pub reg: [u16; Register::R_COUNT as usize],
}
