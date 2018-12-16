use vm::memory::*;
use vm::memory_mapped_registers::*;
use vm::ops::*;
use vm::registers::*;

pub fn bootstrap() {
    // None is needed?
    let registers = [0; Register::R_COUNT as usize];
    let mut storage = RegisterStorage { reg: registers };

    storage.reg[Register::R_PC as usize] = PC_START;

    let mut memory = MemoryStorage { memory: [0; 65536] };

    loop {
        let address = storage.reg[Register::R_PC as usize] + 1;
        storage.reg[Register::R_PC as usize] = address;
        let instr = mem_read(address as usize, &mut memory.memory);
//        let op = OpCode::from(instr << 12);
        let op = OpCode::OpAdd;
        op.run(&mut storage.reg, instr);
    }
}
