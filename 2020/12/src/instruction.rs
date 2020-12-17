use error::{AppError, Result};

fn build_unknown_instruction_error(instruction_str: &str) -> AppError {
    AppError::init(format!("unknown instruction: {}", instruction_str))
}

#[derive(Debug, PartialEq)]
pub(super) struct Instructions {
    instructions: Vec<Instruction>,
}

impl Instructions {
    pub(super) fn new() -> Self {
        Instructions {
            instructions: vec![],
        }
    }

    #[cfg(test)]
    pub(super) fn init(instructions: Vec<Instruction>) -> Self {
        Instructions { instructions }
    }

    pub(super) fn add(&mut self, instruction_str: &str) -> Result<()> {
        if instruction_str.len() < 2 {
            return Err(build_unknown_instruction_error(instruction_str));
        }

        let command = &instruction_str[..1];
        let number = instruction_str[1..].parse::<i32>()?;

        let instruction = match command {
            "N" => Instruction::North(number),
            "E" => Instruction::East(number),
            "S" => Instruction::South(number),
            "W" => Instruction::West(number),
            "L" => Instruction::Left(number),
            "R" => Instruction::Right(number),
            "F" => Instruction::Forward(number),
            _ => return Err(build_unknown_instruction_error(instruction_str)),
        };

        self.instructions.push(instruction);

        Ok(())
    }

    pub(super) fn get_all(&self) -> &[Instruction] {
        &self.instructions
    }
}

#[derive(Debug, PartialEq)]
pub(super) enum Instruction {
    North(i32),
    East(i32),
    South(i32),
    West(i32),
    Left(i32),
    Right(i32),
    Forward(i32),
}

#[cfg(test)]
mod tests {
    use hamcrest2::prelude::*;

    use super::*;

    #[test]
    fn instructions_new_returns_empty_instructions_object() {
        let instructions = Instructions::new();

        assert_that!(
            instructions,
            equal_to(Instructions {
                instructions: vec![]
            })
        );
    }

    #[test]
    fn instructions_init_sets_provided_instructions() {
        let instructions = Instructions::init(vec![Instruction::East(5)]);

        assert_that!(
            instructions,
            equal_to(Instructions {
                instructions: vec![Instruction::East(5)]
            })
        );
    }

    macro_rules! test_instruction_add {
        ($name:ident, $input:expr, $expected:expr) => {
            #[test]
            fn $name() {
                let mut instructions = Instructions::new();
                let result = instructions.add($input);

                assert_that!(&result, ok());
                assert_that!(
                    instructions,
                    equal_to(Instructions {
                        instructions: vec![$expected]
                    })
                );
            }
        };
    }

    test_instruction_add!(instructions_add_for_north, "N15", Instruction::North(15));
    test_instruction_add!(instructions_add_for_east, "E3", Instruction::East(3));
    test_instruction_add!(instructions_add_for_south, "S1", Instruction::South(1));
    test_instruction_add!(instructions_add_for_west, "W30", Instruction::West(30));
    test_instruction_add!(instructions_add_for_left, "L270", Instruction::Left(270));
    test_instruction_add!(instructions_add_for_right, "R90", Instruction::Right(90));
    test_instruction_add!(instructions_add_for_forward, "F3", Instruction::Forward(3));

    #[test]
    fn instructions_add_returns_error_for_invalid_command() {
        let mut instructions = Instructions::new();
        let result = instructions.add("J15");

        assert_that!(&result, err());
        assert_that!(
            format!("{}", result.unwrap_err()),
            equal_to("app error: unknown instruction: J15")
        );
        assert_that!(
            instructions,
            equal_to(Instructions {
                instructions: vec![]
            })
        );
    }
}
