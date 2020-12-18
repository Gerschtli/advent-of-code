use std::collections::HashMap;

fn set_bit(number: i64, position: i64, value: bool) -> i64 {
    let bit = 1 << position;
    let has_bit = number & bit != 0;

    if !has_bit && value {
        number + bit
    } else if has_bit && !value {
        number - bit
    } else {
        number
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(super) enum BitValue {
    One,
    Zero,
    Floating,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(super) struct BitMask {
    key: i64,
    value: BitValue,
}

impl BitMask {
    pub(super) fn new(key: i64, value: BitValue) -> Self {
        Self { key, value }
    }
}

#[derive(Debug, PartialEq)]
pub(super) enum Instruction {
    Mask(Vec<BitMask>),
    Mem(i64, i64),
}

#[derive(Debug, PartialEq)]
pub(super) struct State {
    current_mask: Vec<BitMask>,
    memory: HashMap<i64, i64>,
}

impl State {
    pub(super) fn new() -> Self {
        Self {
            current_mask: Vec::new(),
            memory: HashMap::new(),
        }
    }

    #[cfg(test)]
    pub(super) fn init(current_mask: Vec<BitMask>, memory: HashMap<i64, i64>) -> Self {
        Self {
            current_mask,
            memory,
        }
    }

    pub(crate) fn run(&mut self, instruction: &Instruction) {
        match &instruction {
            Instruction::Mask(mask) => self.current_mask = mask.clone(),
            Instruction::Mem(key, value) => {
                let mut final_value = *value;

                for mask in &self.current_mask {
                    let value = match mask.value {
                        BitValue::One => true,
                        BitValue::Zero => false,
                        BitValue::Floating => continue,
                    };

                    final_value = set_bit(final_value, mask.key, value);
                }

                self.memory.insert(*key, final_value);
            }
        }
    }

    pub(crate) fn run_v2(&mut self, instruction: &Instruction) {
        match &instruction {
            Instruction::Mask(mask) => self.current_mask = mask.clone(),
            Instruction::Mem(key, value) => {
                let mut addresses = vec![*key];
                for mask in &self.current_mask {
                    if mask.value == BitValue::Zero {
                        continue;
                    }

                    addresses = addresses
                        .into_iter()
                        .flat_map(|address| match mask.value {
                            BitValue::One => vec![set_bit(address, mask.key, true)],
                            BitValue::Zero => vec![address],
                            BitValue::Floating => vec![
                                set_bit(address, mask.key, true),
                                set_bit(address, mask.key, false),
                            ],
                        })
                        .collect();
                }

                for address in addresses {
                    self.memory.insert(address, *value);
                }
            }
        }
    }

    pub(super) fn sum(&self) -> i64 {
        self.memory.values().sum()
    }
}

#[cfg(test)]
mod tests {
    use hamcrest2::prelude::*;

    use super::*;

    #[test]
    fn state_run_sets_mask() {
        let mut state = State {
            current_mask: vec![BitMask {
                key: 1,
                value: BitValue::One,
            }],
            memory: HashMap::new(),
        };

        state.run(&Instruction::Mask(vec![BitMask {
            key: 5,
            value: BitValue::Zero,
        }]));

        assert_that!(
            state,
            equal_to(State {
                current_mask: vec![BitMask {
                    key: 5,
                    value: BitValue::Zero
                }],
                memory: HashMap::new(),
            })
        );
    }

    #[test]
    fn state_run_sets_memory() {
        let mut state = State {
            current_mask: vec![
                BitMask::new(6, BitValue::One),
                BitMask::new(1, BitValue::Zero),
            ],
            memory: HashMap::new(),
        };

        state.run(&Instruction::Mem(8, 11));

        let mut map = HashMap::new();
        map.insert(8, 73);

        assert_that!(
            state,
            equal_to(State {
                current_mask: vec![
                    BitMask::new(6, BitValue::One),
                    BitMask::new(1, BitValue::Zero)
                ],
                memory: map,
            })
        );
    }

    #[test]
    fn state_run_v2_sets_mask() {
        let mut state = State {
            current_mask: vec![BitMask {
                key: 1,
                value: BitValue::One,
            }],
            memory: HashMap::new(),
        };

        state.run_v2(&Instruction::Mask(vec![BitMask {
            key: 5,
            value: BitValue::Zero,
        }]));

        assert_that!(
            state,
            equal_to(State {
                current_mask: vec![BitMask {
                    key: 5,
                    value: BitValue::Zero
                }],
                memory: HashMap::new(),
            })
        );
    }

    #[test]
    fn state_run_v2_sets_memory() {
        let mut state = State {
            current_mask: vec![
                BitMask::new(9, BitValue::Zero),
                BitMask::new(8, BitValue::Zero),
                BitMask::new(7, BitValue::Zero),
                BitMask::new(6, BitValue::Zero),
                BitMask::new(5, BitValue::Floating),
                BitMask::new(4, BitValue::One),
                BitMask::new(3, BitValue::Zero),
                BitMask::new(2, BitValue::Zero),
                BitMask::new(1, BitValue::One),
                BitMask::new(0, BitValue::Floating),
            ],
            memory: HashMap::new(),
        };

        state.run_v2(&Instruction::Mem(42, 100));

        let mut map = HashMap::new();
        map.insert(26, 100);
        map.insert(27, 100);
        map.insert(58, 100);
        map.insert(59, 100);

        assert_that!(state.memory, equal_to(map));
    }

    #[test]
    fn state_sum_returns_sum_of_all_memory_values() {
        let mut map = HashMap::new();
        map.insert(7, 101);
        map.insert(8, 64);

        let state = State {
            current_mask: vec![
                BitMask::new(6, BitValue::One),
                BitMask::new(1, BitValue::Zero),
            ],
            memory: map,
        };

        let result = state.sum();
        assert_that!(result, equal_to(165));
    }
}
