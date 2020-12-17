// necessary for intellij support
#[cfg(test)]
#[macro_use]
extern crate hamcrest2;

use error::Result;

use crate::instruction::Instructions;

mod instruction;
mod state;

fn main() {}

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
