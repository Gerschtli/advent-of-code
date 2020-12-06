use std::collections::HashSet;

use error::{AppError, Result};

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct Person {
    answers: HashSet<char>,
}

impl Person {
    pub(super) fn build_from_string(line: &str) -> Result<Self> {
        if line.is_empty() {
            return Err(AppError::init("empty answers entry"));
        }

        let mut answers = HashSet::new();
        for char in line.chars() {
            if 'a' <= char && char <= 'z' {
                answers.insert(char);
            } else {
                return Err(AppError::init(format!("invalid answer: {}", line)));
            }
        }

        Ok(Person { answers })
    }

    #[cfg(test)]
    pub(super) fn init(answers: Vec<char>) -> Self {
        Person {
            answers: answers.into_iter().collect(),
        }
    }

    pub(super) fn get_answer_count(persons: &[Person]) -> usize {
        let answers = persons
            .iter()
            .map(|p| &p.answers)
            .flatten()
            .collect::<HashSet<_>>();

        answers.len()
    }

    pub(super) fn get_answer_intersection_count(persons: &[Person]) -> usize {
        if persons.len() == 0 {
            return 0;
        }

        let f = |acc: HashSet<_>, p: &Person| acc.intersection(&p.answers).cloned().collect();
        let answers = persons.iter().fold(persons[0].answers.clone(), f);

        answers.len()
    }
}

#[cfg(test)]
mod tests {
    use hamcrest2::prelude::*;

    use super::*;

    #[test]
    fn person_build_from_string_returns_object_with_answers() {
        let person = Person::build_from_string("abr");
        let mut answers = HashSet::new();
        answers.insert('a');
        answers.insert('b');
        answers.insert('r');

        assert_that!(person, has(Person { answers }))
    }

    #[test]
    fn person_build_from_string_returns_error_for_invalid_answer() {
        let person = Person::build_from_string("a2b");

        assert_that!(&person, err());
        assert_that!(
            format!("{}", person.unwrap_err()),
            eq("app error: invalid answer: a2b")
        );
    }

    #[test]
    fn person_build_from_string_returns_error_for_empty_line() {
        let person = Person::build_from_string("");

        assert_that!(&person, err());
        assert_that!(
            format!("{}", person.unwrap_err()),
            eq("app error: empty answers entry")
        );
    }

    #[test]
    fn person_init_builds_person_with_answers() {
        let person = Person::init(vec!['a', 'b']);
        let mut answers = HashSet::new();
        answers.insert('a');
        answers.insert('b');

        assert_that!(person, eq(Person { answers }))
    }

    #[test]
    fn get_answer_count_returns_count_of_all_answers() {
        let count = Person::get_answer_count(&vec![
            Person {
                answers: vec!['a', 'b'].into_iter().collect(),
            },
            Person {
                answers: vec!['b', 'c'].into_iter().collect(),
            },
        ]);

        assert_that!(count, eq(3))
    }

    #[test]
    fn get_answer_intersection_count_returns_count_of_all_answer_intersections() {
        let count = Person::get_answer_intersection_count(&vec![
            Person {
                answers: vec!['a', 'b'].into_iter().collect(),
            },
            Person {
                answers: vec!['b', 'c'].into_iter().collect(),
            },
        ]);

        assert_that!(count, eq(1))
    }

    #[test]
    fn get_answer_intersection_count_returns_count_answers_when_only_one_person() {
        let count = Person::get_answer_intersection_count(&vec![Person {
            answers: vec!['a', 'b'].into_iter().collect(),
        }]);

        assert_that!(count, eq(2))
    }

    #[test]
    fn get_answer_intersection_count_returns_0_with_empty_input() {
        let count = Person::get_answer_intersection_count(&vec![]);

        assert_that!(count, eq(0))
    }
}
