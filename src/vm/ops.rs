use vm::flags::ConditionFlag::*;
use vm::memory_mapped_registers::*;
use vm::registers::Register::*;
use vm::traps::TrapCode;

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
    OpLea,  // load effective addreiss
    OpTrap, // execute trap
}

impl OpCode {
    pub fn run(&self, memory: &mut [u16; 65536], reg: &mut [u16; R_COUNT as usize], instr: u16) {
        match self {
            OpCode::OpAdd => {
                // destination register
                let r0 = (instr >> 9) & 0x7;
                // first operand
                let r1 = (instr >> 6) & 0x7;
                // whether we are in immediate mode
                let imm_flag = (instr >> 5) & 0x1 == 1;

                if imm_flag {
                    let imm5: u16 = sign_extend(instr & 0x1F, 5);
                    reg[r0 as usize] = reg[r1 as usize] + imm5;
                } else {
                    let r2 = instr & 0x7;
                    reg[r0 as usize] = reg[r1 as usize] + reg[r2 as usize];
                }

                update_flags(r0, reg);
            }
            OpCode::OpAnd => {
                let r0 = (instr >> 9) & 0x7;
                let r1 = (instr >> 6) & 0x7;
                let imm_flag = (instr >> 5) & 0x1 == 1;

                if imm_flag {
                    let imm5: u16 = sign_extend(instr & 0x1F, 5);
                    reg[r0 as usize] = reg[r1 as usize] & imm5;
                } else {
                    let r2 = instr & 0x7;
                    reg[r0 as usize] = reg[r1 as usize] & reg[r2 as usize];
                }

                update_flags(r0, reg);
            }
            OpCode::OpNot => {
                let r0 = (instr >> 9) & 0x7;
                let r1 = (instr >> 6) & 0x7;
                reg[r0 as usize] = !reg[r1 as usize];

                update_flags(r0, reg);
            }
            OpCode::OpBr => {
                let pc_offset = sign_extend(instr & 0x1ff, 9);
                let cond_flag = (instr >> 9) & 0x7;
                if cond_flag & reg[R_COND as usize] == 1 {
                    reg[R_PC as usize] = reg[R_PC as usize] + pc_offset;
                }
            }
            OpCode::OpJmp => {
                let r1 = (instr >> 6) & 0x7;
                reg[R_PC as usize] = reg[r1 as usize];
            }
            OpCode::OpJsr => {
                let r1 = (instr >> 6) & 0x7;
                let mut long_pc_offset = sign_extend(instr & 0x7ff, 11);
                let long_flag = (instr >> 11) & 1 == 1;

                reg[R_R7 as usize] = reg[R_PC as usize];
                if long_flag {
                    long_pc_offset = long_pc_offset + 1;
                    reg[R_PC as usize] = long_pc_offset;
                } else {
                    reg[R_PC as usize] = reg[r1 as usize];
                }
            }
            OpCode::OpLd => {
                let r0 = (instr >> 9) & 0x7;
                let pc_offset = sign_extend(instr & 0x1ff, 9);
                reg[r0 as usize] = mem_read((reg[R_PC as usize] + pc_offset) as usize, memory);
            }
            OpCode::OpLdr => {
                let r0 = (instr >> 9) & 0x7;
                let r1 = (instr >> 6) & 0x7;
                let offset = sign_extend(instr & 0x3F, 6);
                reg[r0 as usize] = mem_read((reg[r1 as usize] + offset) as usize, memory);
                update_flags(r0, reg);
            }
            OpCode::OpLea => {
                let r0 = (instr >> 9) & 0x7;
                let pc_offset = sign_extend(instr & 0x1ff, 9);
                mem_write(
                    mem_read((reg[R_PC as usize] + pc_offset) as usize, memory) as usize,
                    reg[r0 as usize],
                    memory,
                );
            }
            OpCode::OpSt => {
                let r0 = (instr >> 9) & 0x7;
                let pc_offset = sign_extend(instr & 0x1ff, 9);
                mem_write(
                    mem_read((reg[R_PC as usize] + pc_offset) as usize, memory) as usize,
                    reg[r0 as usize],
                    memory,
                );
            }
            OpCode::OpSti => {
                let r0 = (instr >> 9) & 0x7;
                let r1 = (instr >> 6) & 0x7;
                let offset = sign_extend(instr & 0x3F, 6);
                mem_write(
                    (reg[r1 as usize] + offset) as usize,
                    reg[r0 as usize],
                    memory,
                );
            }
            OpCode::OpStr => {
                let r0 = (instr >> 9) & 0x7;
                let r1 = (instr >> 6) & 0x7;
                let offset = sign_extend(instr & 0x3F, 6);
                mem_write(
                    (reg[r1 as usize] + offset) as usize,
                    reg[r0 as usize],
                    memory,
                );
            }
            OpCode::OpTrap => {
                match TrapCode::from(instr & 0xFF) {
                    TrapCode::TrapGetc => unimplemented!(),
                    TrapCode::TrapOut => unimplemented!(),
                    TrapCode::TrapIn => unimplemented!(),
                    TrapCode::TrapPuts => unimplemented!(),
                    TrapCode::TrapPutSp => unimplemented!(),
                    TrapCode::TrapHalt => unimplemented!(),
                    _ => {
                        // TODO ここは普通に Ok を返す感じの実装にする必要がありそう
                        panic!("bad trap code");
                    }
                }
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
