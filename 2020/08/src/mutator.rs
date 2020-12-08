use crate::code::{Code, Instruction};

#[derive(Debug, PartialEq)]
pub(super) struct Mutator {
    code: Code,
    current_index: usize,
}

impl Mutator {
    pub(super) fn init(code: &Code) -> Self {
        Self {
            code: code.clone(),
            current_index: 0,
        }
    }

    pub(super) fn get_next_mutation(&mut self) -> Option<Code> {
        loop {
            let index = self.current_index;
            self.current_index += 1;

            match self.code.get_instruction(index) {
                Some(&Instruction::Jmp(0)) => (),
                Some(&Instruction::Jmp(_)) | Some(&Instruction::Nop(_)) => {
                    let mut code = self.code.clone();
                    code.swap_instruction(index);

                    return Some(code);
                }
                Some(_) => (),
                None => return None,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use hamcrest2::prelude::*;

    use crate::code::Instruction;

    use super::*;

    #[test]
    fn mutator_init_returns_object() {
        let code = Code::init(vec![Instruction::Jmp(1)]);
        let mutator = Mutator::init(&code);

        assert_that!(
            mutator,
            eq(Mutator {
                code,
                current_index: 0
            })
        );
    }

    #[test]
    fn mutator_get_next_mutation_returns_modified_code_with_replaced_jmp() {
        let mut mutator = Mutator {
            code: Code::init(vec![Instruction::Jmp(1), Instruction::Acc(1)]),
            current_index: 0,
        };

        let code = mutator.get_next_mutation();

        assert_that!(
            code,
            has(Code::init(vec![Instruction::Nop(1), Instruction::Acc(1)]))
        );
    }

    #[test]
    fn mutator_get_next_mutation_returns_modified_code_with_replaced_nop() {
        let mut mutator = Mutator {
            code: Code::init(vec![Instruction::Nop(1), Instruction::Acc(1)]),
            current_index: 0,
        };

        let code = mutator.get_next_mutation();

        assert_that!(
            code,
            has(Code::init(vec![Instruction::Jmp(1), Instruction::Acc(1)]))
        );
    }

    #[test]
    fn mutator_get_next_mutation_returns_modified_code_with_next_available_nop() {
        let mut mutator = Mutator {
            code: Code::init(vec![Instruction::Acc(1), Instruction::Nop(1)]),
            current_index: 0,
        };

        let code = mutator.get_next_mutation();

        assert_that!(
            code,
            has(Code::init(vec![Instruction::Acc(1), Instruction::Jmp(1)]))
        );
    }

    #[test]
    fn mutator_get_next_mutation_returns_modified_code_on_consecutive_calls() {
        let mut mutator = Mutator {
            code: Code::init(vec![Instruction::Jmp(1), Instruction::Nop(1)]),
            current_index: 0,
        };

        let code1 = mutator.get_next_mutation();
        let code2 = mutator.get_next_mutation();

        assert_that!(
            code1,
            has(Code::init(vec![Instruction::Nop(1), Instruction::Nop(1)]))
        );
        assert_that!(
            code2,
            has(Code::init(vec![Instruction::Jmp(1), Instruction::Jmp(1)]))
        );
    }

    #[test]
    fn mutator_get_next_mutation_returns_none_when_no_mutation_possible() {
        let mut mutator = Mutator {
            code: Code::init(vec![Instruction::Acc(1), Instruction::Acc(1)]),
            current_index: 0,
        };

        let code = mutator.get_next_mutation();

        assert_that!(code, none());
    }

    #[test]
    fn mutator_get_next_mutation_skips_jmp_0() {
        let mut mutator = Mutator {
            code: Code::init(vec![Instruction::Jmp(0), Instruction::Acc(1)]),
            current_index: 0,
        };

        let code = mutator.get_next_mutation();

        assert_that!(code, none());
    }
}
