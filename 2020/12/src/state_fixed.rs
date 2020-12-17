use crate::instruction::Instruction;

#[derive(Debug, PartialEq)]
struct Point {
    north: i32,
    east: i32,
}

impl Point {
    fn add(&mut self, point: &Point, factor: i32) {
        self.north += factor * point.north;
        self.east += factor * point.east;
    }

    fn rotate(&mut self, angle: i32) {
        match (angle + 360) % 360 {
            90 => {
                let north = self.north;
                self.north = self.east * -1;
                self.east = north;
            }
            180 => {
                self.north *= -1;
                self.east *= -1;
            }
            270 => {
                let north = self.north;
                self.north = self.east;
                self.east = north * -1;
            }
            _ => (),
        }
    }
}

#[derive(Debug, PartialEq)]
pub(super) struct StateFixed {
    position: Point,
    waypoint: Point,
}

impl StateFixed {
    pub(super) fn new() -> Self {
        Self {
            position: Point { north: 0, east: 0 },
            waypoint: Point { north: 1, east: 10 },
        }
    }

    pub(super) fn apply(&mut self, instruction: &Instruction) {
        match *instruction {
            Instruction::North(value) => self.waypoint.north += value,
            Instruction::East(value) => self.waypoint.east += value,
            Instruction::South(value) => self.waypoint.north -= value,
            Instruction::West(value) => self.waypoint.east -= value,
            Instruction::Left(value) => self.waypoint.rotate(value * -1),
            Instruction::Right(value) => self.waypoint.rotate(value),
            Instruction::Forward(value) => self.position.add(&self.waypoint, value),
        }
    }

    pub(super) fn get_manhattan_distance(&self) -> i32 {
        self.position.north.abs() + self.position.east.abs()
    }
}

#[cfg(test)]
mod tests {
    use hamcrest2::prelude::*;

    use super::*;

    #[test]
    fn point_add_updates_values() {
        let mut point = Point { north: 1, east: -2 };
        point.add(&Point { north: 4, east: 6 }, 3);

        assert_that!(
            point,
            equal_to(Point {
                north: 13,
                east: 16
            })
        );
    }

    #[test]
    fn point_rotate_handles_90_degrees() {
        let mut point = Point { north: 1, east: -2 };
        point.rotate(90);

        assert_that!(point, equal_to(Point { north: 2, east: 1 }));
    }

    #[test]
    fn point_rotate_handles_180_degrees() {
        let mut point = Point { north: 1, east: -2 };
        point.rotate(180);

        assert_that!(point, equal_to(Point { north: -1, east: 2 }));
    }

    #[test]
    fn point_rotate_handles_negative_90_degrees() {
        let mut point = Point { north: 1, east: -2 };
        point.rotate(-90);

        assert_that!(
            point,
            equal_to(Point {
                north: -2,
                east: -1
            })
        );
    }

    #[test]
    fn state_fixed_new_returns_start_position() {
        let state = StateFixed::new();

        assert_that!(
            state,
            equal_to(StateFixed {
                position: Point { north: 0, east: 0 },
                waypoint: Point { north: 1, east: 10 }
            })
        )
    }

    macro_rules! test_state_fixed_apply {
        ($name:ident, $input:expr, $expected:expr) => {
            #[test]
            fn $name() {
                let mut state = StateFixed {
                    position: Point { north: 0, east: 0 },
                    waypoint: Point { north: 1, east: 10 },
                };

                state.apply(&$input);

                assert_that!(state, equal_to($expected));
            }
        };
    }

    test_state_fixed_apply!(
        state_apply_north,
        Instruction::North(5),
        StateFixed {
            position: Point { north: 0, east: 0 },
            waypoint: Point { north: 6, east: 10 },
        }
    );
    test_state_fixed_apply!(
        state_apply_east,
        Instruction::East(7),
        StateFixed {
            position: Point { north: 0, east: 0 },
            waypoint: Point { north: 1, east: 17 },
        }
    );
    test_state_fixed_apply!(
        state_apply_south,
        Instruction::South(9),
        StateFixed {
            position: Point { north: 0, east: 0 },
            waypoint: Point {
                north: -8,
                east: 10
            },
        }
    );
    test_state_fixed_apply!(
        state_apply_west,
        Instruction::West(17),
        StateFixed {
            position: Point { north: 0, east: 0 },
            waypoint: Point { north: 1, east: -7 },
        }
    );
    test_state_fixed_apply!(
        state_apply_left,
        Instruction::Left(180),
        StateFixed {
            position: Point { north: 0, east: 0 },
            waypoint: Point {
                north: -1,
                east: -10
            },
        }
    );
    test_state_fixed_apply!(
        state_apply_right,
        Instruction::Right(90),
        StateFixed {
            position: Point { north: 0, east: 0 },
            waypoint: Point {
                north: -10,
                east: 1
            },
        }
    );
    test_state_fixed_apply!(
        state_apply_forward,
        Instruction::Forward(2),
        StateFixed {
            position: Point { north: 2, east: 20 },
            waypoint: Point { north: 1, east: 10 },
        }
    );

    #[test]
    fn state_get_manhattan_distance() {
        let state = StateFixed {
            position: Point {
                north: 72,
                east: 214,
            },
            waypoint: Point { north: 1, east: 10 },
        };

        assert_that!(state.get_manhattan_distance(), equal_to(286));
    }
}
