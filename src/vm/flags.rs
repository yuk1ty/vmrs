//enum_from_primitive! {
//    #[derive(Debug, PartialEq)]
//    pub enum ConditionFlag {
//        FL_POS = 1 << 0, // P
//        FL_ZRO = 1 << 1, // Z
//        FL_NEG = 1 << 2, // N
//    }
//}

#[derive(Debug, PartialEq)]
pub enum ConditionFlag {
    FlPos = 1 << 0,
    FlZro = 1 << 1,
    FlNeg = 1 << 2
}
