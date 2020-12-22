#![deny(warnings)]
#![allow(unused_imports)]

// necessary for intellij support
#[cfg(test)]
#[macro_use]
extern crate hamcrest2;

use error::Result;

use crate::token::PostfixNotation;

mod token;

fn main() -> Result<()> {
    let (sum1, sum2) = run()?;

    println!("sum part 1: {}", sum1);
    println!("sum part 2: {}", sum2);

    Ok(())
}

fn run() -> Result<(i64, i64)> {
    let lines = file::read_lines("./files/math.txt")?;
    let sum1 = lines
        .iter()
        .map(|line| PostfixNotation::parse(line, PostfixNotation::precedences_1))
        .map(|expression| expression.evaluate())
        .sum();
    let sum2 = lines
        .iter()
        .map(|line| PostfixNotation::parse(line, PostfixNotation::precedences_2))
        .map(|expression| expression.evaluate())
        .sum();

    Ok((sum1, sum2))
}

#[cfg(test)]
mod tests {
    use hamcrest2::prelude::*;

    use super::*;

    #[test]
    fn run_returns_result() {
        let result = run();

        assert_that!(result, has((11004703763391, 290726428573651)));
    }
}
