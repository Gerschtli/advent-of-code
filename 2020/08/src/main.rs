// necessary for intellij support
#[cfg(test)]
#[macro_use]
extern crate hamcrest2;

use error::{AppError, Result};

use crate::code::Code;

mod code;
mod state;

fn main() {}

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
    fn parse_lines_returns_code() {
        let lines = vec![
            "acc -2".to_string(),
            "nop +0".to_string(),
            "jmp +3".to_string(),
        ];

        let code = parse_lines(&lines);

        assert_that!(&code, ok());
        assert_that!(
            code.unwrap(),
            eq(Code::init(vec![
                Instruction::Acc(-2),
                Instruction::Nop,
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
