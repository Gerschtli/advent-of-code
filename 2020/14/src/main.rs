#![deny(warnings)]
#![allow(unused_imports)]

// necessary for intellij support
#[cfg(test)]
#[macro_use]
extern crate hamcrest2;

use error::Result;
use lazy_static::lazy_static;
use regex::Regex;

use crate::instruction::{BitMask, BitValue, Instruction, State};

mod instruction;

fn main() -> Result<()> {
    let (sum, sum_v2) = evaluate()?;

    println!("sum of all memory values: {}", sum);
    println!("sum of all memory values (v2): {}", sum_v2);

    Ok(())
}

fn evaluate() -> Result<(i64, i64)> {
    let lines = file::read_lines("./files/program.txt")?;
    let instructions = parse_program(&lines)?;
    let state = run_program(&instructions);
    let state_v2 = run_program_v2(&instructions);

    Ok((state.sum(), state_v2.sum()))
}

fn parse_program(lines: &[String]) -> Result<Vec<Instruction>> {
    let mut instructions = vec![];

    lazy_static! {
        static ref RE_MASK: Regex = Regex::new(r"^mask = ([X10]{36})$").unwrap();
        static ref RE_MEM: Regex = Regex::new(r"^mem\[(\d+)\] = (\d+)$").unwrap();
    }

    for line in lines {
        if let Some(captures) = RE_MASK.captures(line) {
            let mask = captures.get(1).unwrap().as_str();
            let mut masks = vec![];

            for (i, char) in mask.chars().enumerate() {
                let value = match char {
                    '1' => BitValue::One,
                    '0' => BitValue::Zero,
                    'X' => BitValue::Floating,
                    _ => continue,
                };

                masks.push(BitMask::new(35 - i as i64, value));
            }

            instructions.push(Instruction::Mask(masks));
        } else if let Some(captures) = RE_MEM.captures(line) {
            let key = captures.get(1).unwrap().as_str().parse::<i64>()?;
            let value = captures.get(2).unwrap().as_str().parse::<i64>()?;

            instructions.push(Instruction::Mem(key, value));
        }
    }

    Ok(instructions)
}

fn run_program(instructions: &[Instruction]) -> State {
    let mut state = State::new();

    for instruction in instructions {
        state.run(instruction);
    }

    state
}

fn run_program_v2(instructions: &[Instruction]) -> State {
    let mut state = State::new();

    for instruction in instructions {
        state.run_v2(instruction);
    }

    state
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use hamcrest2::prelude::*;

    use super::*;

    #[test]
    fn evaluate_returns_sum() {
        assert_that!(evaluate(), has((9967721333886, 4355897790573)));
    }

    #[test]
    fn parse_program_returns_instructions() {
        let result = parse_program(&vec![
            "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X".to_string(),
            "mem[8] = 11".to_string(),
            "mem[7] = 101".to_string(),
            "mem[8] = 0".to_string(),
        ]);

        let expected = vec![
            Instruction::Mask(vec![
                BitMask::new(35, BitValue::Floating),
                BitMask::new(34, BitValue::Floating),
                BitMask::new(33, BitValue::Floating),
                BitMask::new(32, BitValue::Floating),
                BitMask::new(31, BitValue::Floating),
                BitMask::new(30, BitValue::Floating),
                BitMask::new(29, BitValue::Floating),
                BitMask::new(28, BitValue::Floating),
                BitMask::new(27, BitValue::Floating),
                BitMask::new(26, BitValue::Floating),
                BitMask::new(25, BitValue::Floating),
                BitMask::new(24, BitValue::Floating),
                BitMask::new(23, BitValue::Floating),
                BitMask::new(22, BitValue::Floating),
                BitMask::new(21, BitValue::Floating),
                BitMask::new(20, BitValue::Floating),
                BitMask::new(19, BitValue::Floating),
                BitMask::new(18, BitValue::Floating),
                BitMask::new(17, BitValue::Floating),
                BitMask::new(16, BitValue::Floating),
                BitMask::new(15, BitValue::Floating),
                BitMask::new(14, BitValue::Floating),
                BitMask::new(13, BitValue::Floating),
                BitMask::new(12, BitValue::Floating),
                BitMask::new(11, BitValue::Floating),
                BitMask::new(10, BitValue::Floating),
                BitMask::new(9, BitValue::Floating),
                BitMask::new(8, BitValue::Floating),
                BitMask::new(7, BitValue::Floating),
                BitMask::new(6, BitValue::One),
                BitMask::new(5, BitValue::Floating),
                BitMask::new(4, BitValue::Floating),
                BitMask::new(3, BitValue::Floating),
                BitMask::new(2, BitValue::Floating),
                BitMask::new(1, BitValue::Zero),
                BitMask::new(0, BitValue::Floating),
            ]),
            Instruction::Mem(8, 11),
            Instruction::Mem(7, 101),
            Instruction::Mem(8, 0),
        ];

        assert_that!(&result, ok());
        assert_that!(result, has(expected));
    }

    #[test]
    fn run_program_returns_state() {
        let result = run_program(&vec![
            Instruction::Mask(vec![
                BitMask::new(6, BitValue::One),
                BitMask::new(1, BitValue::Zero),
            ]),
            Instruction::Mem(8, 11),
            Instruction::Mem(7, 101),
            Instruction::Mem(8, 0),
        ]);

        let mut map = HashMap::new();
        map.insert(7, 101);
        map.insert(8, 64);

        assert_that!(
            result,
            equal_to(State::init(
                vec![
                    BitMask::new(6, BitValue::One),
                    BitMask::new(1, BitValue::Zero),
                ],
                map
            ))
        );
    }
}
