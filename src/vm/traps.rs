#[derive(Debug, PartialEq)]
pub enum TrapCode {
    TrapGetc = 0x20,
    TrapOut = 0x21,
    TrapPuts = 0x22,
    TrapIn = 0x23,
    TrapPutSp = 0x24,
    TrapHalt = 0x25,
}
