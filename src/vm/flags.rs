#[derive(Debug, PartialEq)]
pub enum ConditionFlag {
    FlPos = 1 << 0,
    FlZro = 1 << 1,
    FlNeg = 1 << 2,
}
