use std::fs;
use std::io;
use std::io::BufRead;
use std::path;
use std::str::FromStr;

use lazy_static::lazy_static;
use regex::Regex;

use crate::error::AppError;

mod error;

#[derive(Debug)]
struct Line {
    min: u8,
    max: u8,
    char: char,
    password: String,
}

fn main() -> Result<(), AppError> {
    let lines = read_lines("./files/passwords.txt")?
        .iter()
        .map(|line| parse_line(line))
        .collect::<Result<Vec<_>, AppError>>()?;

    println!("Result: {:#?}", lines);

    Ok(())
}

fn read_lines<P>(filename: P) -> io::Result<Vec<String>>
where
    P: AsRef<path::Path>,
{
    let file = fs::File::open(filename)?;
    let lines_raw = io::BufReader::new(file).lines();

    lines_raw.into_iter().collect()
}

fn parse_line(line: &str) -> Result<Line, AppError> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^(\d+)-(\d+) ([a-z]): ([a-z]+)$").unwrap();
    }

    let build_option_error = || AppError::invalid_line(&line);
    let captures = RE.captures(line).ok_or_else(build_option_error)?;

    let min = captures.get(1).ok_or_else(build_option_error)?;
    let max = captures.get(2).ok_or_else(build_option_error)?;
    let char = captures.get(3).ok_or_else(build_option_error)?;
    let password = captures.get(4).ok_or_else(build_option_error)?;

    Ok(Line {
        min: min.as_str().parse()?,
        max: max.as_str().parse()?,
        char: char::from_str(char.as_str())?,
        password: password.as_str().to_string(),
    })
}

#[cfg(test)]
mod tests {
    use hamcrest2::prelude::*;

    use super::*;

    #[test]
    fn read_lines_returns_every_line_from_file() {
        let lines = read_lines("./files/example.txt");

        assert_that!(
            lines,
            has(vec![
                "abc".to_string(),
                "bcd".to_string(),
                "cde".to_string()
            ])
        );
    }

    #[test]
    fn read_lines_returns_error_when_file_not_found() {
        let lines = read_lines("./files/example_not_found.txt");

        assert_that!(lines, err());
    }

    #[test]
    fn test_parse_line_returns_populated_line_object() {
        let line1 = parse_line("1-2 z: qlfzd");
        let line2 = parse_line("5-7 a: abcd");

        assert_that!(&line1, ok());
        assert_that!(&line2, ok());

        let line1_unwrapped = line1.unwrap();
        let line2_unwrapped = line2.unwrap();

        assert_that!(line1_unwrapped.min, eq(1));
        assert_that!(line1_unwrapped.max, eq(2));
        assert_that!(line1_unwrapped.char, eq('z'));
        assert_that!(line1_unwrapped.password, eq("qlfzd"));

        assert_that!(line2_unwrapped.min, eq(5));
        assert_that!(line2_unwrapped.max, eq(7));
        assert_that!(line2_unwrapped.char, eq('a'));
        assert_that!(line2_unwrapped.password, eq("abcd"));
    }
}
