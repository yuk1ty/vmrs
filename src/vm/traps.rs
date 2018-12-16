#[derive(Debug, PartialEq)]
pub enum TrapCode {
    TrapGetc = 0x20,
    TrapOut = 0x21,
    TrapPuts = 0x22,
    TrapIn = 0x23,
    TrapPutSp = 0x24,
    TrapHalt = 0x25,
    TrapEmpty,
}

impl From<u16> for TrapCode {
    fn from(x: u16) -> Self {
        match x {
            0x20 => TrapCode::TrapGetc,
            0x21 => TrapCode::TrapOut,
            0x22 => TrapCode::TrapPuts,
            0x23 => TrapCode::TrapIn,
            0x24 => TrapCode::TrapPutSp,
            0x25 => TrapCode::TrapHalt,
            _ => TrapCode::TrapEmpty,
        }
    }
}
