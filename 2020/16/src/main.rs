#![deny(warnings)]
#![allow(unused_imports)]

// necessary for intellij support
#[cfg(test)]
#[macro_use]
extern crate hamcrest2;

use std::num::ParseIntError;
use std::result;

use error::Result;
use lazy_static::lazy_static;
use regex::Regex;

use crate::data::{Rule, Ticket};

mod data;

fn main() {}

fn parse_lines(lines: &[String]) -> Result<(Vec<Rule>, Ticket, Vec<Ticket>)> {
    let (rules, index) = parse_rules(lines);
    let my_ticket = parse_ticket(&lines[index + 1])?;

    let tickets = lines[index + 4..]
        .iter()
        .map(|line| parse_ticket(line))
        .collect::<Result<Vec<_>>>()?;

    Ok((rules, my_ticket, tickets))
}

fn parse_rules(lines: &[String]) -> (Vec<Rule>, usize) {
    let mut rules = vec![];

    lazy_static! {
        static ref RE_RULE: Regex = Regex::new(r"^([^:]+): (\d+)-(\d+) or (\d+)-(\d+)$").unwrap();
    }

    let mut next_index = 0;
    for (index, line) in lines.iter().enumerate() {
        if line.is_empty() {
            next_index = index + 1;
            break;
        }

        if let Some(captures) = RE_RULE.captures(line) {
            rules.push(Rule::new(
                captures.get(1).unwrap().as_str(),
                vec![
                    (
                        captures.get(2).unwrap().as_str().parse().unwrap(),
                        captures.get(3).unwrap().as_str().parse().unwrap(),
                    ),
                    (
                        captures.get(4).unwrap().as_str().parse().unwrap(),
                        captures.get(5).unwrap().as_str().parse().unwrap(),
                    ),
                ],
            ));
        }
    }

    (rules, next_index)
}

fn parse_ticket(line: &str) -> Result<Ticket> {
    let values = line
        .split(',')
        .map(|value| value.parse::<i32>())
        .collect::<result::Result<Vec<_>, ParseIntError>>()?;

    Ok(Ticket::new(values))
}

fn get_error_rate(rules: &[Rule], tickets: &[Ticket]) -> i32 {
    tickets
        .iter()
        .map(|ticket| match ticket.is_valid_for_any_rule(rules) {
            (true, _) => 0,
            (false, value) => value,
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use hamcrest2::prelude::*;

    use super::*;

    #[test]
    fn parse_lines_returns_all_data() {
        let lines = vec![
            "class: 1-3 or 5-7".to_string(),
            "row: 6-11 or 33-44".to_string(),
            "seat: 13-40 or 45-50".to_string(),
            "".to_string(),
            "your ticket:".to_string(),
            "7,1,14".to_string(),
            "".to_string(),
            "nearby tickets:".to_string(),
            "7,3,47".to_string(),
            "40,4,50".to_string(),
            "55,2,20".to_string(),
            "38,6,12".to_string(),
        ];

        let result = parse_lines(&lines);

        assert_that!(&result, ok());
        assert_that!(
            result,
            has((
                vec![
                    Rule::new("class", vec![(1, 3), (5, 7)]),
                    Rule::new("row", vec![(6, 11), (33, 44)]),
                    Rule::new("seat", vec![(13, 40), (45, 50)]),
                ],
                Ticket::new(vec![7, 1, 14]),
                vec![
                    Ticket::new(vec![7, 3, 47]),
                    Ticket::new(vec![40, 4, 50]),
                    Ticket::new(vec![55, 2, 20]),
                    Ticket::new(vec![38, 6, 12]),
                ],
            ))
        );
    }

    #[test]
    fn get_error_rate_returns_sum_of_errors() {
        let result = get_error_rate(
            &vec![
                Rule::new("class", vec![(1, 3), (5, 7)]),
                Rule::new("row", vec![(6, 11), (33, 44)]),
                Rule::new("seat", vec![(13, 40), (45, 50)]),
            ],
            &vec![
                Ticket::new(vec![7, 3, 47]),
                Ticket::new(vec![40, 4, 50]),
                Ticket::new(vec![55, 2, 20]),
                Ticket::new(vec![38, 6, 12]),
            ],
        );

        assert_that!(result, equal_to(71));
    }
}
