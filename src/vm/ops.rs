use vm::registers::Register::*;
use vm::flags::ConditionFlag::*;

#[derive(Debug, PartialEq)]
pub enum OpCode {
    OpBr,   // branch
    OpAdd,  // add
    OpLd,   // load
    OpSt,   // store
    OpJsr,  // jump register
    OpAnd,  // bitwise and
    OpLdr,  // load register
    OpStr,  // store register
    OpRti,  // unused
    OpNot,  // bitwise not
    OpLdi,  // load indirect
    OpSti,  // store indirect
    OpJmp,  // jump
    OpRes,  // reserved
    OpLea,  // load effective address
    OpTrap, // execute trap
}

impl OpCode {
    pub fn run(&self, reg: &mut [u16; R_COUNT as usize]) {
        match self {
            OpCode::OpAdd => {
                let ret_instr: u16 = 0;
                // destination register
                let r0 = (ret_instr >> 9) & 0x7;
                // first operand
                let r1 = (ret_instr >> 6) & 0x7;
                // whether we are in immediate mode
                let imm_flag = (ret_instr >> 5) & 0x1 == 1;

                if imm_flag {
                    let imm5: u16 = sign_extend(ret_instr & 0x1F, 5);
                    reg[r0 as usize] = reg[r1 as usize] + imm5;
                } else {
                    let r2 = ret_instr & 0x7;
                    reg[r0 as usize] = reg[r1 as usize] + reg[r2 as usize];
                }

                update_flags(r0, reg);
            }
            _ => unimplemented!(),
        }
    }
}

impl From<u16> for OpCode {
    fn from(n: u16) -> Self {
        match n {
            0 => OpCode::OpBr,
            1 => OpCode::OpAdd,
            2 => OpCode::OpLd,
            3 => OpCode::OpSt,
            4 => OpCode::OpJsr,
            5 => OpCode::OpAnd,
            6 => OpCode::OpLdr,
            7 => OpCode::OpStr,
            8 => OpCode::OpRti,
            9 => OpCode::OpNot,
            10 => OpCode::OpSti,
            11 => OpCode::OpJmp,
            12 => OpCode::OpRes,
            13 => OpCode::OpLea,
            _ => OpCode::OpTrap,
        }
    }
}

fn sign_extend(mut x: u16, bit_count: i16) -> u16 {
    let negative = (x >> (bit_count - 1)) & 1 == 1;
    if negative {
        x |= 0xFFFF << bit_count
    }
    return x;
}

fn update_flags(r: u16, reg: &mut [u16; R_COUNT as usize]) {
    if reg[r as usize] == 0 {
        reg[R_COND as usize] = FlZro as u16;
    } else if (reg[r as usize] >> 15) == 1 {
        reg[R_COND as usize] = FlNeg as u16;
    } else {
        reg[R_COND as usize] = FlPos as u16;
    }
}
