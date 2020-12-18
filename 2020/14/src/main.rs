// necessary for intellij support
#[cfg(test)]
#[macro_use]
extern crate hamcrest2;

use error::Result;
use lazy_static::lazy_static;
use regex::Regex;

use crate::instruction::{BitMask, Instruction};

mod instruction;

fn main() {}

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

            for (i, char) in mask.chars().into_iter().enumerate() {
                if char == 'X' {
                    continue;
                }

                masks.push(BitMask::new(35 - i, char == '1'));
            }

            instructions.push(Instruction::Mask(masks));
        } else if let Some(captures) = RE_MEM.captures(line) {
            let key = captures.get(1).unwrap().as_str().parse::<usize>()?;
            let value = captures.get(2).unwrap().as_str().parse::<i64>()?;

            instructions.push(Instruction::Mem(key, value));
        }
    }

    Ok(instructions)
}

#[cfg(test)]
mod tests {
    use hamcrest2::prelude::*;

    use super::*;

    #[test]
    fn parse_program_returns_instructions() {
        let result = parse_program(&vec![
            "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X".to_string(),
            "mem[8] = 11".to_string(),
            "mem[7] = 101".to_string(),
            "mem[8] = 0".to_string(),
        ]);

        let expected = vec![
            Instruction::Mask(vec![BitMask::new(6, true), BitMask::new(1, false)]),
            Instruction::Mem(8, 11),
            Instruction::Mem(7, 101),
            Instruction::Mem(8, 0),
        ];

        assert_that!(&result, ok());
        assert_that!(result, has(expected));
    }
}
