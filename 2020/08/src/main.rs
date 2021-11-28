#![deny(warnings)]
#![allow(unused_imports)]

// necessary for intellij support
#[cfg(test)]
#[macro_use]
extern crate hamcrest2;

use error::{AppError, Result};
use file::read_lines;

use crate::code::Code;
use crate::mutator::Mutator;
use crate::state::{RunResult, State};

mod code;
mod mutator;
mod state;

fn main() -> Result<()> {
    let (acc_before_infinite_loop, acc_of_mutations) = get_accumulators()?;

    println!(
        "accumulator before infinite loop: {}",
        acc_before_infinite_loop
    );
    println!("accumulator after mutations: {}", acc_of_mutations);

    Ok(())
}

fn get_accumulators() -> Result<(i32, i32)> {
    let lines = read_lines("./files/boot_code.txt")?;
    let code = parse_lines(&lines)?;

    let acc_before_infinite_loop = get_accumulator(&code, RunResult::InfiniteLoopReached)?;
    let acc_of_mutations = get_accumulator_of_mutations(&code)?;

    Ok((acc_before_infinite_loop, acc_of_mutations))
}

fn get_accumulator_of_mutations(code: &Code) -> Result<i32> {
    let mut mutator = Mutator::init(code);
    loop {
        match mutator.get_next_mutation() {
            Some(code) => {
                if let Ok(value) = get_accumulator(&code, RunResult::End) {
                    return Ok(value);
                }
            }
            None => return Err(AppError::init("no mutation worked!")),
        }
    }
}

fn get_accumulator(code: &Code, expected_result: RunResult) -> Result<i32> {
    let mut state = State::init(code);

    loop {
        match state.run() {
            result if result == expected_result => return Ok(state.get_accumulator()),
            RunResult::Success => (),
            RunResult::InvalidInstruction => {
                return Err(AppError::init("invalid instruction found"));
            }
            RunResult::End => return Err(AppError::init("ended without infinite loop")),
            RunResult::InfiniteLoopReached => return Err(AppError::init("ended in infinite loop")),
        }
    }
}

fn parse_lines(lines: &[String]) -> Result<Code> {
    let mut code = Code::default();

    for line in lines {
        let instruction = &line[..3];
        let value = line[4..]
            .parse::<i32>()
            .map_err(|e| AppError::init_err(format!("invalid line '{}'", line), e))?;

        code.add_instruction(instruction, value)?;
    }

    Ok(code)
}

#[cfg(test)]
mod tests {
    use hamcrest2::prelude::*;

    use crate::code::Instruction;

    use super::*;

    #[test]
    fn test_get_accumulators() {
        let result = get_accumulators();

        assert_that!(result, has((1818, 631)));
    }

    #[test]
    fn parse_lines_returns_code() {
        let lines = vec![
            "acc -2".to_string(),
            "nop +1".to_string(),
            "jmp +3".to_string(),
        ];

        let code = parse_lines(&lines);

        assert_that!(&code, ok());
        assert_that!(
            code.unwrap(),
            eq(Code::init(vec![
                Instruction::Acc(-2),
                Instruction::Nop(1),
                Instruction::Jmp(3)
            ]))
        )
    }

    #[test]
    fn parse_lines_returns_error_for_invalid_instruction() {
        let lines = vec!["abc -2".to_string()];

        let code = parse_lines(&lines);

        assert_that!(&code, err());
        assert_that!(
            format!("{}", code.unwrap_err()),
            eq("app error: invalid instruction: abc")
        );
    }

    #[test]
    fn parse_lines_returns_error_for_invalid_number() {
        let lines = vec!["abc -x".to_string()];

        let code = parse_lines(&lines);

        assert_that!(&code, err());
        assert_that!(
            format!("{}", code.unwrap_err()),
            eq("app error: invalid line 'abc -x': invalid digit found in string")
        );
    }
}
