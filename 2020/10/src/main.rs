// necessary for intellij support
#[cfg(test)]
#[macro_use]
extern crate hamcrest2;

use std::result;

use error::Result;

fn main() {}

fn parse_lines(lines: &[String]) -> Result<Vec<i32>> {
    let numbers = lines
        .iter()
        .map(|line| line.parse())
        .collect::<result::Result<Vec<_>, _>>()?;

    Ok(numbers)
}

#[cfg(test)]
mod tests {
    use hamcrest2::prelude::*;

    use super::*;

    #[test]
    fn parse_lines_returns_list_of_ints() {
        let numbers = parse_lines(&vec![
            "35".to_string(),
            "20".to_string(),
            "15".to_string(),
            "25".to_string(),
            "47".to_string(),
        ]);

        assert_that!(&numbers, ok());
        assert_that!(
            &numbers.unwrap(),
            contains(vec![35, 20, 15, 25, 47]).exactly().in_order()
        )
    }

    #[test]
    fn parse_lines_returns_error_for_invalid_lines() {
        let numbers = parse_lines(&vec![
            "35".to_string(),
            "20".to_string(),
            "abc".to_string(),
            "25".to_string(),
            "47".to_string(),
        ]);

        assert_that!(&numbers, err());
        assert_that!(
            format!("{}", numbers.unwrap_err()),
            equal_to("app error: invalid digit found in string")
        )
    }
}
