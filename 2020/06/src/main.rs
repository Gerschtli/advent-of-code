// necessary for intellij support
#[cfg(test)]
#[macro_use]
extern crate hamcrest2;

use error::Result;
use file::read_lines;

use crate::group::Group;
use crate::person::Person;

mod group;
mod person;

fn main() -> Result<()> {
    let sum = get_sum_of_all_answers()?;

    println!("sum: {}", sum);

    Ok(())
}

fn get_sum_of_all_answers() -> Result<usize> {
    let lines = read_lines("./files/answers.txt")?;
    let groups = parse_lines(&lines)?;

    let sum_of_all_answers = groups.iter().map(|g| g.get_answer_count()).sum();

    Ok(sum_of_all_answers)
}

fn parse_lines(lines: &[String]) -> Result<Vec<Group>> {
    let mut groups = vec![];
    let mut current_group = Group::new();
    let mut needs_push = false;

    for line in lines {
        if line.is_empty() {
            groups.push(current_group);
            current_group = Group::new();
            needs_push = false;
        } else {
            let person = Person::build_from_string(line)?;
            current_group = current_group.with_person(person);
            needs_push = true;
        }
    }

    if needs_push {
        groups.push(current_group);
    }

    Ok(groups)
}

#[cfg(test)]
mod tests {
    use hamcrest2::prelude::*;

    use crate::person::Person;

    use super::*;

    #[test]
    fn get_sum_of_all_answers_returns_sum() {
        let sum = get_sum_of_all_answers();

        assert_that!(sum, has(7110));
    }

    #[test]
    fn parse_lines_returns_list_of_groups() {
        let lines = vec![
            "abc".to_string(),
            "".to_string(),
            "a".to_string(),
            "b".to_string(),
            "c".to_string(),
            "".to_string(),
            "ab".to_string(),
            "ac".to_string(),
        ];
        let groups = parse_lines(&lines);

        let expected = vec![
            Group::new().with_person(Person::init(vec!['a', 'b', 'c'])),
            Group::new()
                .with_person(Person::init(vec!['a']))
                .with_person(Person::init(vec!['b']))
                .with_person(Person::init(vec!['c'])),
            Group::new()
                .with_person(Person::init(vec!['a', 'b']))
                .with_person(Person::init(vec!['a', 'c'])),
        ];

        assert_that!(&groups, ok());
        assert_that!(&groups.unwrap(), contains(expected).exactly());
    }

    #[test]
    fn parse_lines_returns_error_for_invalid_lines() {
        let lines = vec![
            "abc".to_string(),
            "".to_string(),
            "a".to_string(),
            "b2".to_string(),
            "c".to_string(),
        ];
        let groups = parse_lines(&lines);

        assert_that!(&groups, err());
        assert_that!(
            format!("{}", groups.unwrap_err()),
            eq("app error: invalid answer: b2")
        );
    }
}
