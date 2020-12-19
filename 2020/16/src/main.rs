#![deny(warnings)]
#![allow(unused_imports)]

// necessary for intellij support
#[cfg(test)]
#[macro_use]
extern crate hamcrest2;

use std::collections::{HashMap, HashSet};
use std::num::ParseIntError;
use std::result;

use error::Result;
use lazy_static::lazy_static;
use regex::Regex;

use crate::data::{Rule, Ticket};

mod data;

fn main() -> Result<()> {
    let (error_rate, departure_values) = run()?;

    println!("error rate: {}", error_rate);
    println!("departure values: {}", departure_values);

    Ok(())
}

fn run() -> Result<(i32, i64)> {
    let lines = file::read_lines("./files/tickets.txt")?;
    let (rules, my_ticket, tickets) = parse_lines(&lines)?;

    let error_rate = get_error_rate(&rules, &tickets);

    let mut filtered_tickets = filter_invalid_tickets(&rules, &tickets);
    filtered_tickets.push(&my_ticket);

    let field_mapping = get_field_mapping(&rules, &filtered_tickets);
    let departure_values = field_mapping
        .iter()
        .filter(|(rule, _)| rule.name().starts_with("departure "))
        .map(|(_, index)| my_ticket.values()[*index])
        .fold(1_i64, |acc, v| acc * v as i64);

    Ok((error_rate, departure_values))
}

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

fn filter_invalid_tickets<'a>(rules: &[Rule], tickets: &'a [Ticket]) -> Vec<&'a Ticket> {
    tickets
        .iter()
        .filter(|ticket| ticket.is_valid_for_any_rule(rules).0)
        .collect()
}

fn get_field_mapping<'a>(rules: &'a [Rule], tickets: &[&Ticket]) -> HashMap<&'a Rule, usize> {
    let index_list = 0..tickets[0].values().len();
    let mut map = rules
        .iter()
        .map(|rule| (rule, index_list.clone().collect::<HashSet<_>>()))
        .collect::<HashMap<_, HashSet<usize>>>();

    for ticket in tickets {
        for (i, value) in ticket.values().iter().enumerate() {
            for rule in rules {
                if !rule.is_valid_value(*value) {
                    map.get_mut(rule).unwrap().remove(&i);
                }
            }
        }
    }

    let mut final_map = HashMap::new();

    loop {
        for (rule, set) in &map {
            if set.len() == 1 {
                final_map.insert(*rule, *set.iter().next().unwrap());
            }
        }

        let taken_indexes = map
            .iter()
            .filter(|(_, set)| set.len() == 1)
            .map(|(rule, set)| {
                let index = *set.iter().next().unwrap();
                final_map.insert(*rule, index);
                index
            })
            .collect::<HashSet<_>>();

        if taken_indexes.is_empty() {
            break;
        }

        map = map
            .iter()
            .filter(|(_, set)| set.len() > 1)
            .map(|(rule, set)| (*rule, set.difference(&taken_indexes).cloned().collect()))
            .collect();
    }

    final_map
}

#[cfg(test)]
mod tests {
    use hamcrest2::prelude::*;

    use super::*;

    #[test]
    fn run_returns_values() {
        assert_that!(run(), has((23954, 453459307723)));
    }

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

    #[test]
    fn filter_invalid_tickets_returns_sliced_tickets() {
        let tickets = vec![
            Ticket::new(vec![7, 3, 47]),
            Ticket::new(vec![40, 4, 50]),
            Ticket::new(vec![55, 2, 20]),
            Ticket::new(vec![38, 6, 12]),
        ];
        let result = filter_invalid_tickets(
            &vec![
                Rule::new("class", vec![(1, 3), (5, 7)]),
                Rule::new("row", vec![(6, 11), (33, 44)]),
                Rule::new("seat", vec![(13, 40), (45, 50)]),
            ],
            &tickets,
        );

        let ticket = Ticket::new(vec![7, 3, 47]);
        assert_that!(result, equal_to(vec![&ticket]));
    }

    #[test]
    fn get_field_mapping_returns_map() {
        let rule_class = Rule::new("class", vec![(0, 1), (4, 19)]);
        let rule_row = Rule::new("row", vec![(0, 5), (8, 19)]);
        let rule_seat = Rule::new("seat", vec![(0, 13), (16, 19)]);
        let rules = vec![rule_class.clone(), rule_row.clone(), rule_seat.clone()];

        let ticket0 = Ticket::new(vec![3, 9, 18]);
        let ticket1 = Ticket::new(vec![15, 1, 5]);
        let ticket2 = Ticket::new(vec![5, 14, 9]);
        let ticket3 = Ticket::new(vec![11, 12, 13]);

        let tickets = vec![&ticket0, &ticket1, &ticket2, &ticket3];

        let result = get_field_mapping(&rules, &tickets);

        assert_that!(result.len(), equal_to(3));
        assert_that!(result[&rule_row], equal_to(0));
        assert_that!(result[&rule_class], equal_to(1));
        assert_that!(result[&rule_seat], equal_to(2));
    }
}
