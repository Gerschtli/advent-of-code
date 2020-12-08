// necessary for intellij support
#[cfg(test)]
#[macro_use]
extern crate hamcrest2;

use error::{AppError, Result};
use file::read_lines;

use crate::code::Code;
use crate::state::{RunResult, State};

mod code;
mod mutator;
mod state;

fn main() -> Result<()> {
    let acc_before_infinite_loop = get_acc_before_infinite_loop()?;

    println!(
        "accumulator before infinite loop: {}",
        acc_before_infinite_loop
    );

    Ok(())
}

fn get_acc_before_infinite_loop() -> Result<i32> {
    let lines = read_lines("./files/boot_code.txt")?;
    let code = parse_lines(&lines)?;
    let mut state = State::init(&code);

    loop {
        match state.run() {
            RunResult::InfiniteLoopReached => return Ok(state.get_accumulator()),
            RunResult::Success => (),
            RunResult::InvalidInstruction => {
                return Err(AppError::init("invalid instruction found"));
            }
            RunResult::End => return Err(AppError::init("ended without infinite loop")),
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
    fn test_get_acc_before_infinite_loop() {
        let result = get_acc_before_infinite_loop();

        assert_that!(result, has(1818));
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
