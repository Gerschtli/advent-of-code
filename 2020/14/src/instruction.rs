#[derive(Debug, PartialEq)]
pub(super) struct BitMask {
    key: usize,
    value: bool,
}

impl BitMask {
    pub(super) fn new(key: usize, value: bool) -> Self {
        Self { key, value }
    }
}

#[derive(Debug, PartialEq)]
pub(super) enum Instruction {
    Mask(Vec<BitMask>),
    Mem(usize, i64),
}
