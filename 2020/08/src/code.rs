use error::{AppError, Result};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub(super) enum Instruction {
    Acc(i32),
    Jmp(i32),
    Nop(i32),
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub(super) struct Code {
    instructions: Vec<Instruction>,
}

impl Code {
    #[cfg(test)]
    pub(super) fn init(instructions: Vec<Instruction>) -> Self {
        Self { instructions }
    }

    pub(super) fn add_instruction(&mut self, instruction: &str, value: i32) -> Result<()> {
        self.instructions.push(match instruction {
            "acc" => Instruction::Acc(value),
            "jmp" => Instruction::Jmp(value),
            "nop" => Instruction::Nop(value),
            _ => {
                return Err(AppError::init(format!(
                    "invalid instruction: {}",
                    instruction
                )));
            }
        });

        Ok(())
    }

    pub(super) fn get_instruction(&self, index: usize) -> Option<&Instruction> {
        self.instructions.get(index)
    }

    pub(super) fn swap_instruction(&mut self, index: usize) {
        match self.instructions.get(index) {
            Some(&Instruction::Nop(value)) => {
                self.instructions[index] = Instruction::Jmp(value);
            }
            Some(&Instruction::Jmp(value)) => {
                self.instructions[index] = Instruction::Nop(value);
            }
            _ => (),
        }
    }
}

#[cfg(test)]
mod tests {
    use hamcrest2::prelude::*;

    use super::*;

    #[test]
    fn code_init_returns_instance() {
        let code = Code::init(vec![Instruction::Acc(5), Instruction::Nop(1)]);

        assert_that!(
            code,
            eq(Code {
                instructions: vec![Instruction::Acc(5), Instruction::Nop(1)]
            })
        )
    }

    #[test]
    fn code_add_instruction_adds_acc() {
        let mut code = Code::default();
        let result = code.add_instruction("acc", 5);

        assert_that!(&result, ok());
        assert_that!(
            code,
            eq(Code {
                instructions: vec![Instruction::Acc(5)]
            })
        );
    }

    #[test]
    fn code_add_instruction_adds_jmp() {
        let mut code = Code::default();
        let result = code.add_instruction("jmp", -4);

        assert_that!(&result, ok());
        assert_that!(
            code,
            eq(Code {
                instructions: vec![Instruction::Jmp(-4)]
            })
        );
    }

    #[test]
    fn code_add_instruction_adds_nop() {
        let mut code = Code::default();
        let result = code.add_instruction("nop", 1);

        assert_that!(&result, ok());
        assert_that!(
            code,
            eq(Code {
                instructions: vec![Instruction::Nop(1)]
            })
        );
    }

    #[test]
    fn code_add_instruction_returns_err_for_invalid_instructions() {
        let mut code = Code::default();
        let result = code.add_instruction("abc", 0);

        assert_that!(&result, err());
        assert_that!(
            format!("{}", result.unwrap_err()),
            eq("app error: invalid instruction: abc")
        );
    }

    #[test]
    fn code_get_instruction_returns_object() {
        let code = Code {
            instructions: vec![
                Instruction::Acc(0),
                Instruction::Acc(1),
                Instruction::Acc(2),
            ],
        };

        let result = code.get_instruction(1);

        assert_that!(result, has(&Instruction::Acc(1)));
    }

    #[test]
    fn code_get_instruction_returns_none_if_not_in_range() {
        let code = Code {
            instructions: vec![
                Instruction::Acc(0),
                Instruction::Acc(1),
                Instruction::Acc(2),
            ],
        };

        let result = code.get_instruction(3);

        assert_that!(result, none());
    }

    #[test]
    fn code_swap_instruction_ignores_acc() {
        let mut code = Code {
            instructions: vec![Instruction::Acc(0), Instruction::Acc(1)],
        };

        code.swap_instruction(1);

        assert_that!(
            code,
            eq(Code {
                instructions: vec![Instruction::Acc(0), Instruction::Acc(1),],
            })
        );
    }

    #[test]
    fn code_swap_instruction_changes_nop() {
        let mut code = Code {
            instructions: vec![Instruction::Acc(0), Instruction::Nop(1)],
        };

        code.swap_instruction(1);

        assert_that!(
            code,
            eq(Code {
                instructions: vec![Instruction::Acc(0), Instruction::Jmp(1),],
            })
        );
    }

    #[test]
    fn code_swap_instruction_changes_jmp() {
        let mut code = Code {
            instructions: vec![Instruction::Acc(0), Instruction::Jmp(1)],
        };

        code.swap_instruction(1);

        assert_that!(
            code,
            eq(Code {
                instructions: vec![Instruction::Acc(0), Instruction::Nop(1),],
            })
        );
    }

    #[test]
    fn code_swap_instruction_ignores_wrong_index() {
        let mut code = Code {
            instructions: vec![Instruction::Acc(0), Instruction::Jmp(1)],
        };

        code.swap_instruction(2);

        assert_that!(
            code,
            eq(Code {
                instructions: vec![Instruction::Acc(0), Instruction::Jmp(1),],
            })
        );
    }
}
