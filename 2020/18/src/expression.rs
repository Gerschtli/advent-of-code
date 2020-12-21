use crate::expression::Expression::{Addition, Multiplication, Value};

#[derive(Debug, PartialEq)]
pub(super) enum Expression {
    Value(i64),
    Addition(Box<Expression>, Box<Expression>),
    Multiplication(Box<Expression>, Box<Expression>),
}

impl Expression {
    pub(super) fn parse(input: &str) -> Self {
        let mut lhs = None;
        let mut op = None;
        let mut nested_str: Option<String> = None;
        let mut counter = 0;
        for c in input.chars() {
            if c == '(' {
                counter += 1
            }
            if c == ')' {
                counter -= 1
            }

            if counter > 0 && nested_str.is_none() {
                nested_str = Some(String::new());
            }

            match nested_str {
                Some(ref mut n) if counter > 0 => {
                    if counter != 1 || c != '(' {
                        n.push(c)
                    }
                }
                _ => match c {
                    '0'..='9' | ')' => {
                        let v = if c == ')' {
                            let string = nested_str.unwrap();
                            nested_str = None;
                            Self::parse(&string)
                        } else {
                            Value(c as i64 - '0' as i64)
                        };

                        lhs = match lhs {
                            Some(lhs_value) => Some(match op {
                                Some(op_char) => {
                                    let l = Box::new(lhs_value);
                                    let r = Box::new(v);
                                    op = None;

                                    if op_char == '*' {
                                        Multiplication(l, r)
                                    } else {
                                        Addition(l, r)
                                    }
                                }
                                _ => v,
                            }),
                            None => Some(v),
                        };
                    }
                    '*' | '+' => op = Some(c),
                    _ => continue,
                },
            }
        }

        lhs.unwrap()
    }

    pub(super) fn evaluate(&self) -> i64 {
        match self {
            Expression::Addition(lhs, rhs) => lhs.evaluate() + rhs.evaluate(),
            Expression::Multiplication(lhs, rhs) => lhs.evaluate() * rhs.evaluate(),
            Value(value) => *value,
        }
    }
}

#[cfg(test)]
mod tests {
    use hamcrest2::prelude::*;

    use super::*;

    #[test]
    fn expression_parse() {
        let expr = Expression::parse("1 + 2 * 3");

        #[rustfmt::skip]
        assert_that!(
            expr,
            equal_to(
                Multiplication(
                    Box::new(Addition(
                        Box::new(Value(1)),
                        Box::new(Value(2)),
                    )),
                    Box::new(Value(3)),
                )
            )
        );
    }

    #[test]
    fn expression_parse_with_parentheses() {
        let expr = Expression::parse("1 + (2 * 3) + (4 * (5 + 6))");

        #[rustfmt::skip]
        assert_that!(
            expr,
            equal_to(
                Addition(
                    Box::new(Addition(
                        Box::new(Value(1)),
                        Box::new(Multiplication(
                            Box::new(Value(2)),
                            Box::new(Value(3)),
                        )),
                    )),
                    Box::new(Multiplication(
                        Box::new(Value(4)),
                        Box::new(Addition(
                            Box::new(Value(5)),
                            Box::new(Value(6)),
                        )),
                    )),
                )
            )
        );
    }
}
