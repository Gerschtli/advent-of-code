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
        let person = Person::build_from_string("a").unwrap();
        let group = Group { persons: vec![] };

        let group_new = group.with_person(person.clone());

        assert_that!(
            group_new,
            eq(Group {
                persons: vec![person]
            })
        )
    }
}
