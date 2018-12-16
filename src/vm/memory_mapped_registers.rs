use nix::libc::getchar;
use nix::sys::select::select;
use nix::sys::select::FdSet;
use nix::sys::time::TimeVal;
use nix::sys::time::TimeValLike;

#[derive(Debug, PartialEq)]
pub enum MemoryMappedRegister {
    MrKbsr = 0xFE00,
    MrKbdr = 0xFE02,
}

pub fn mem_write(address: usize, val: u16, memory: &mut [u16; 65536]) {
    memory[address] = val;
}

pub fn mem_read(address: usize, memory: &mut [u16; 65536]) -> u16 {
    if address == MemoryMappedRegister::MrKbdr as usize {
        if check_key() {
            memory[MemoryMappedRegister::MrKbsr as usize] = 1 << 15;
            unsafe {
                memory[MemoryMappedRegister::MrKbdr as usize] = getchar() as u16;
            }
        } else {
            memory[MemoryMappedRegister::MrKbsr as usize] = 0;
        }
    }

    memory[address]
}

fn check_key() -> bool {
    let mut fd_set = FdSet::new();
    fd_set.insert(0); // STDIN_FILENO = 0 なので？
    let mut timeval = TimeVal::seconds(0);
    select(None, &mut fd_set, None, None, &mut timeval).expect("failed to select!");
    true
}
