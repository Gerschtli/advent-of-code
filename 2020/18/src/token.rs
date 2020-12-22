use std::collections::HashMap;
use std::io::{stdout, Write};
use std::{thread, time};

use crate::token::Token::{Addition, Multiplication, Number, ParentheseClose, ParentheseOpen};

#[derive(Debug, PartialEq, Eq, Hash)]
pub(super) enum Token {
    Number(i64),
    Addition,
    Multiplication,
    ParentheseClose,
    ParentheseOpen,
}

#[derive(Debug, PartialEq)]
pub(super) struct PostfixNotation {
    tokens: Vec<Token>,
}

impl PostfixNotation {
    pub(super) fn precedences_1(token: &Token) -> u8 {
        match token {
            ParentheseClose | ParentheseOpen => 2,
            Addition | Multiplication => 1,
            _ => 0,
        }
    }

    pub(super) fn precedences_2(token: &Token) -> u8 {
        match token {
            ParentheseClose | ParentheseOpen => 3,
            Addition => 2,
            Multiplication => 1,
            _ => 0,
        }
    }

    pub(super) fn parse<F>(input: &str, precedences: F) -> Self
    where
        F: Fn(&Token) -> u8,
    {
        let mut tokens = vec![];
        let mut ops = vec![];
        for c in input.chars() {
            let operator = match c {
                '0'..='9' => {
                    tokens.push(Number(c as i64 - '0' as i64));
                    continue;
                }
                '*' => Multiplication,
                '+' => Addition,
                '(' => ParentheseOpen,
                ')' => ParentheseClose,
                _ => continue,
            };

            if !ops.is_empty() && precedences(&operator) <= precedences(ops.last().unwrap()) {
                while let Some(op) = ops.last() {
                    if op == &ParentheseOpen || precedences(op) < precedences(&operator) {
                        break;
                    }
                    tokens.push(ops.pop().unwrap());
                }
            }

            if operator == ParentheseClose {
                while let Some(op) = ops.pop() {
                    if op == ParentheseOpen {
                        break;
                    }
                    tokens.push(op);
                }
            } else {
                ops.push(operator);
            }
        }
        while let Some(op) = ops.pop() {
            tokens.push(op);
        }

        PostfixNotation { tokens }
    }

    pub(super) fn evaluate(&self) -> i64 {
        let mut operands = vec![];

        for t in &self.tokens {
            let value = match t {
                Number(n) => *n,
                Addition => operands.pop().unwrap() + operands.pop().unwrap(),
                Multiplication => operands.pop().unwrap() * operands.pop().unwrap(),
                _ => continue,
            };

            operands.push(value);
        }

        operands[0]
    }
}

#[cfg(test)]
mod tests {
    use hamcrest2::prelude::*;

    use super::*;

    #[test]
    fn expression_parse() {
        let expr = PostfixNotation::parse("1 + 2 * 3", PostfixNotation::precedences_2);

        assert_that!(
            expr,
            equal_to(PostfixNotation {
                tokens: vec![Number(1), Number(2), Addition, Number(3), Multiplication]
            })
        );
    }

    #[test]
    fn expression_parse_with_parentheses() {
        let expr = PostfixNotation::parse(
            "1 + (2 * 3) + (4 * (5 + 6))",
            PostfixNotation::precedences_2,
        );

        assert_that!(
            expr,
            equal_to(PostfixNotation {
                tokens: vec![
                    Number(1),
                    Number(2),
                    Number(3),
                    Multiplication,
                    Addition,
                    Number(4),
                    Number(5),
                    Number(6),
                    Addition,
                    Multiplication,
                    Addition,
                ]
            })
        );
    }
}
