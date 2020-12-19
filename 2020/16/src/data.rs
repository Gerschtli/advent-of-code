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
}
