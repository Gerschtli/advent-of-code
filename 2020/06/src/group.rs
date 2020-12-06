use crate::person::Person;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct Group {
    persons: Vec<Person>,
}

impl Group {
    pub(super) fn new() -> Self {
        Group { persons: vec![] }
    }

    pub(super) fn with_person(mut self, person: Person) -> Self {
        self.persons.push(person);
        self
    }

    pub(super) fn get_answer_count(&self) -> usize {
        Person::get_answer_count(&self.persons)
    }
}

#[cfg(test)]
mod tests {
    use hamcrest2::prelude::*;

    use super::*;

    #[test]
    fn group_new_returns_empty_persons_list() {
        assert_that!(Group::new(), eq(Group { persons: vec![] }))
    }

    #[test]
    fn group_with_person_adds_person_to_list() {
        let person = Person::init(vec!['a']);
        let group = Group { persons: vec![] };

        let group_new = group.with_person(person.clone());

        assert_that!(
            group_new,
            eq(Group {
                persons: vec![person]
            })
        )
    }

    #[test]
    fn group_get_answer_count_returns_count_of_all_answers() {
        let count = Person::get_answer_count(&vec![
            Person::init(vec!['a', 'b']),
            Person::init(vec!['b', 'c']),
        ]);

        assert_that!(count, eq(3))
    }
}
