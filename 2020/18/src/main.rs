#![deny(warnings)]
#![allow(unused_imports)]

// necessary for intellij support
#[cfg(test)]
#[macro_use]
extern crate hamcrest2;

use error::Result;

use crate::expression::Expression;

mod expression;

fn main() -> Result<()> {
    let sum = run()?;

    println!("sum: {}", sum);

    Ok(())
}

fn run() -> Result<i64> {
    let lines = file::read_lines("./files/math.txt")?;
    let sum = lines
        .iter()
        .map(|line| Expression::parse(line))
        .map(|expression| expression.evaluate())
        .sum();

    Ok(sum)
}

#[cfg(test)]
mod tests {
    use hamcrest2::prelude::*;

    use super::*;

    #[test]
    fn run_returns_result() {
        let result = run();

        assert_that!(result, has(11004703763391));
    }
}
