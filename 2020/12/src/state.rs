use crate::instruction::Instruction;

#[derive(Debug, PartialEq)]
pub(super) struct State {
    north: i32,
    east: i32,
    facing: i32,
}

impl State {
    pub(super) fn new() -> Self {
        State {
            north: 0,
            east: 0,
            facing: 90, // start facing east
        }
    }

    pub(super) fn apply(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::North(value) => self.north += value,
            Instruction::East(value) => self.east += value,
            Instruction::South(value) => self.north -= value,
            Instruction::West(value) => self.east -= value,
            Instruction::Left(value) => self.facing = (self.facing - value + 360) % 360,
            Instruction::Right(value) => self.facing = (self.facing + value) % 360,
            Instruction::Forward(value) => match self.facing {
                0 => self.north += value,
                90 => self.east += value,
                180 => self.north -= value,
                270 => self.east -= value,
                _ => (),
            },
        }
    }

    pub(super) fn get_manhattan_distance(&self) -> i32 {
        self.north.abs() + self.east.abs()
    }
}

#[cfg(test)]
mod tests {
    use hamcrest2::prelude::*;

    use super::*;

    #[test]
    fn state_new_returns_start_position() {
        let state = State::new();

        assert_that!(
            state,
            equal_to(State {
                north: 0,
                east: 0,
                facing: 90
            })
        )
    }

    macro_rules! test_state_apply {
        ($name:ident, $input:expr, $expected:expr) => {
            #[test]
            fn $name() {
                let mut state = State {
                    north: 0,
                    east: 0,
                    facing: 0,
                };

                state.apply($input);

                assert_that!(state, equal_to($expected));
            }
        };
    }

    test_state_apply!(
        state_apply_north,
        Instruction::North(5),
        State {
            north: 5,
            east: 0,
            facing: 0
        }
    );
    test_state_apply!(
        state_apply_east,
        Instruction::East(7),
        State {
            north: 0,
            east: 7,
            facing: 0
        }
    );
    test_state_apply!(
        state_apply_south,
        Instruction::South(9),
        State {
            north: -9,
            east: 0,
            facing: 0
        }
    );
    test_state_apply!(
        state_apply_west,
        Instruction::West(17),
        State {
            north: 0,
            east: -17,
            facing: 0
        }
    );
    test_state_apply!(
        state_apply_left,
        Instruction::Left(270),
        State {
            north: 0,
            east: 0,
            facing: 90
        }
    );
    test_state_apply!(
        state_apply_right,
        Instruction::Right(90),
        State {
            north: 0,
            east: 0,
            facing: 90
        }
    );

    #[test]
    fn state_apply_forward_north() {
        let mut state = State {
            north: 0,
            east: 0,
            facing: 0,
        };
        state.apply(Instruction::Forward(5));

        assert_that!(
            state,
            equal_to(State {
                north: 5,
                east: 0,
                facing: 0
            })
        );
    }

    #[test]
    fn state_apply_forward_east() {
        let mut state = State {
            north: 0,
            east: 0,
            facing: 90,
        };
        state.apply(Instruction::Forward(5));

        assert_that!(
            state,
            equal_to(State {
                north: 0,
                east: 5,
                facing: 90
            })
        );
    }

    #[test]
    fn state_apply_forward_south() {
        let mut state = State {
            north: 0,
            east: 0,
            facing: 180,
        };
        state.apply(Instruction::Forward(5));

        assert_that!(
            state,
            equal_to(State {
                north: -5,
                east: 0,
                facing: 180
            })
        );
    }

    #[test]
    fn state_apply_forward_west() {
        let mut state = State {
            north: 0,
            east: 0,
            facing: 270,
        };
        state.apply(Instruction::Forward(5));

        assert_that!(
            state,
            equal_to(State {
                north: 0,
                east: -5,
                facing: 270
            })
        );
    }

    #[test]
    fn state_get_manhattan_distance() {
        let state = State {
            north: -8,
            east: 17,
            facing: 0,
        };

        assert_that!(state.get_manhattan_distance(), equal_to(25));
    }
}
