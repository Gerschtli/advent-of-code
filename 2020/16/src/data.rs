#[derive(Debug, PartialEq)]
pub(super) struct Rule {
    name: String,
    ranges: Vec<(i32, i32)>,
}

impl Rule {
    pub(super) fn new<I, J>(name: I, ranges: J) -> Self
    where
        I: Into<String>,
        J: Into<Vec<(i32, i32)>>,
    {
        Self {
            name: name.into(),
            ranges: ranges.into(),
        }
    }

    fn is_valid_value(&self, value: i32) -> bool {
        for (low, high) in &self.ranges {
            if *low <= value && value <= *high {
                return true;
            }
        }

        false
    }
}

#[derive(Debug, PartialEq)]
pub(super) struct Ticket {
    values: Vec<i32>,
}

impl Ticket {
    pub(super) fn new<I>(values: I) -> Self
    where
        I: Into<Vec<i32>>,
    {
        Self {
            values: values.into(),
        }
    }

    pub(super) fn is_valid_for_any_rule(&self, rules: &[Rule]) -> (bool, i32) {
        let result = self
            .values
            .iter()
            .filter(|value| {
                let count_matching_rules = rules
                    .iter()
                    .filter(|rule| rule.is_valid_value(**value))
                    .count();

                count_matching_rules == 0
            })
            .next();

        match result {
            Some(value) => (false, *value),
            None => (true, 0),
        }
    }
}

#[cfg(test)]
mod tests {
    use hamcrest2::prelude::*;

    use super::*;

    #[test]
    fn ticket_is_valid_for_any_rule_returns_true_for_valid() {
        let ticket = Ticket::new(vec![7, 3, 47]);
        let rules = vec![
            Rule::new("class", vec![(1, 3), (5, 7)]),
            Rule::new("row", vec![(6, 11), (33, 44)]),
            Rule::new("seat", vec![(13, 40), (45, 50)]),
        ];

        assert_that!(ticket.is_valid_for_any_rule(&rules), equal_to((true, 0)));
    }

    #[test]
    fn ticket_is_valid_for_any_rule_returns_false_and_error_value_for_invalid() {
        let ticket = Ticket::new(vec![40, 4, 50]);
        let rules = vec![
            Rule::new("class", vec![(1, 3), (5, 7)]),
            Rule::new("row", vec![(6, 11), (33, 44)]),
            Rule::new("seat", vec![(13, 40), (45, 50)]),
        ];

        assert_that!(ticket.is_valid_for_any_rule(&rules), equal_to((false, 4)));
    }
}
