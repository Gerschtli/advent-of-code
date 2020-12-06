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
}
