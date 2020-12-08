use std::collections::HashSet;

use crate::code::{Code, Instruction};

#[derive(Debug, Eq, PartialEq)]
pub(super) enum RunResult {
    Success,
    End,
    InfiniteLoopReached,
    InvalidInstruction,
}

#[derive(Debug, Eq, PartialEq)]
pub(super) struct State<'a> {
    code: &'a Code,
    accumulator: i32,
    executed_indexes: HashSet<usize>,
    next_index: usize,
}

impl<'a> State<'a> {
    pub(super) fn init(code: &'a Code) -> Self {
        State {
            code,
            accumulator: 0,
            executed_indexes: HashSet::new(),
            next_index: 0,
        }
    }

    pub(super) fn run(&mut self) -> RunResult {
        if self.executed_indexes.contains(&self.next_index) {
            return RunResult::InfiniteLoopReached;
        }

        self.executed_indexes.insert(self.next_index);

        match self.code.get_instruction(self.next_index) {
            Some(&Instruction::Acc(value)) => {
                self.accumulator += value;
                self.next_index += 1;
            }
            Some(&Instruction::Jmp(value)) => {
                let new_index = self.next_index as i32 + value;
                if new_index < 0 {
                    return RunResult::InvalidInstruction;
                }

                self.next_index = new_index as usize;
            }
            Some(&Instruction::Nop) => {
                self.next_index += 1;
            }
            None => return RunResult::End,
        };

        RunResult::Success
    }
}

#[cfg(test)]
mod tests {
    use std::iter::FromIterator;

    use hamcrest2::prelude::*;

    use crate::code::Instruction;

    use super::*;

    #[test]
    fn state_init_returns_empty_state() {
        let code = Code::init(vec![Instruction::Nop]);

        let state = State::init(&code);

        assert_that!(
            state,
            eq(State {
                code: &code,
                accumulator: 0,
                executed_indexes: HashSet::new(),
                next_index: 0
            })
        );
    }

    #[test]
    fn state_run_executes_step_acc() {
        let code = Code::init(vec![Instruction::Acc(1)]);
        let mut state = State::init(&code);

        let result = state.run();

        assert_that!(result, eq(RunResult::Success));
        assert_that!(
            state,
            eq(State {
                code: &code,
                accumulator: 1,
                executed_indexes: HashSet::from_iter(vec![0].into_iter()),
                next_index: 1
            })
        );
    }

    #[test]
    fn state_run_executes_step_jmp() {
        let code = Code::init(vec![Instruction::Jmp(2)]);
        let mut state = State::init(&code);

        let result = state.run();

        assert_that!(result, eq(RunResult::Success));
        assert_that!(
            state,
            eq(State {
                code: &code,
                accumulator: 0,
                executed_indexes: HashSet::from_iter(vec![0].into_iter()),
                next_index: 2
            })
        );
    }

    #[test]
    fn state_run_executes_step_nop() {
        let code = Code::init(vec![Instruction::Nop]);
        let mut state = State::init(&code);

        let result = state.run();

        assert_that!(result, eq(RunResult::Success));
        assert_that!(
            state,
            eq(State {
                code: &code,
                accumulator: 0,
                executed_indexes: HashSet::from_iter(vec![0].into_iter()),
                next_index: 1
            })
        );
    }

    #[test]
    fn state_run_executes_step_on_saved_index() {
        let code = Code::init(vec![
            Instruction::Acc(5),
            Instruction::Nop,
            Instruction::Jmp(-2),
        ]);
        let mut state = State::init(&code);

        assert_that!(state.run(), eq(RunResult::Success));
        assert_that!(state.run(), eq(RunResult::Success));

        assert_that!(
            state,
            eq(State {
                code: &code,
                accumulator: 5,
                executed_indexes: HashSet::from_iter(vec![0, 1].into_iter()),
                next_index: 2
            })
        );
    }

    #[test]
    fn state_run_detects_infinite_loop() {
        let code = Code::init(vec![
            Instruction::Acc(5),
            Instruction::Nop,
            Instruction::Jmp(-2),
        ]);
        let mut state = State::init(&code);

        assert_that!(state.run(), eq(RunResult::Success));
        assert_that!(state.run(), eq(RunResult::Success));
        assert_that!(state.run(), eq(RunResult::Success));
        assert_that!(state.run(), eq(RunResult::InfiniteLoopReached));

        assert_that!(
            state,
            eq(State {
                code: &code,
                accumulator: 5,
                executed_indexes: HashSet::from_iter(vec![0, 1, 2].into_iter()),
                next_index: 0
            })
        );
    }

    #[test]
    fn state_run_returns_error_for_invalid_jmp() {
        let code = Code::init(vec![
            Instruction::Acc(5),
            Instruction::Nop,
            Instruction::Jmp(-3),
        ]);
        let mut state = State::init(&code);

        assert_that!(state.run(), eq(RunResult::Success));
        assert_that!(state.run(), eq(RunResult::Success));
        assert_that!(state.run(), eq(RunResult::InvalidInstruction));
    }
}
