#![deny(warnings)]
#![allow(unused_imports)]

// necessary for intellij support
#[cfg(test)]
#[macro_use]
extern crate hamcrest2;

use error::Result;

use crate::instruction::Instructions;
use crate::state::State;
use crate::state_fixed::StateFixed;

mod instruction;
mod state;
mod state_fixed;

fn main() -> Result<()> {
    let (distance, distance_fixed) = get_distance()?;

    println!("manhattan distance: {}", distance);
    println!("manhattan distance (fixed): {}", distance_fixed);

    Ok(())
}

fn get_distance() -> Result<(i32, i32)> {
    let lines = file::read_lines("./files/instructions.txt")?;
    let instructions = parse_instructions(&lines)?;
    let mut state = State::new();
    let mut state_fixed = StateFixed::new();

    for instruction in instructions.get_all() {
        state.apply(instruction);
        state_fixed.apply(instruction);
    }

    Ok((
        state.get_manhattan_distance(),
        state_fixed.get_manhattan_distance(),
    ))
}

fn parse_instructions(lines: &[String]) -> Result<Instructions> {
    let mut instructions = Instructions::new();

    for line in lines {
        instructions.add(line)?;
    }

    Ok(instructions)
}

#[cfg(test)]
mod tests {
    use hamcrest2::prelude::*;

    use crate::instruction::Instruction;

    use super::*;

    #[test]
    fn get_distance_returns_result() {
        assert_that!(get_distance(), has((962, 56135)));
    }

    #[test]
    fn parse_instructions_returns_parsed_instructions() {
        let lines = vec!["N12".to_string(), "W7".to_string()];
        let result = parse_instructions(&lines);

        assert_that!(&result, ok());
        assert_that!(
            result,
            has(Instructions::init(vec![
                Instruction::North(12),
                Instruction::West(7),
            ]))
        );
    }

    #[test]
    fn parse_instructions_returns_error_for_invalid_lines() {
        let lines = vec!["N12".to_string(), "A7".to_string()];
        let result = parse_instructions(&lines);

        assert_that!(&result, err());
        assert_that!(
            format!("{}", result.unwrap_err()),
            equal_to("app error: unknown instruction: A7")
        );
    }
}
