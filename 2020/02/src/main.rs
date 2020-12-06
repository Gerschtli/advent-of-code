// necessary for intellij support
#[cfg(test)]
#[macro_use]
extern crate hamcrest2;

use std::str::FromStr;

use lazy_static::lazy_static;
use regex::Regex;

use error::{AppError, Result};
use file::read_lines;

#[derive(Debug)]
struct Line {
    index_left: u8,
    index_right: u8,
    char: char,
    password: String,
}

fn main() -> Result<()> {
    let (count_sled_rental, count_otca) = count_valid_lines()?;

    println!("Result Sled Rental: {:#?}", count_sled_rental);
    println!("Result OTCA: {:#?}", count_otca);

    Ok(())
}

fn count_valid_lines() -> Result<(usize, usize)> {
    let lines = read_lines("./files/passwords.txt")?;

    let line_vec = lines
        .iter()
        .map(|line| parse_line(line))
        .collect::<Result<Vec<_>>>()?;

    let count_sled_rental = line_vec
        .iter()
        .map(|line| is_valid_for_sled_rental(line))
        .filter(|is_valid| *is_valid)
        .count();

    let count_otca = line_vec
        .iter()
        .map(|line| is_valid_for_otca(line))
        .filter(|is_valid| *is_valid)
        .count();

    Ok((count_sled_rental, count_otca))
}

fn parse_line(line: &str) -> Result<Line> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^(\d+)-(\d+) ([a-z]): ([a-z]+)$").unwrap();
    }

    let build_option_error = || AppError::init(format!("invalid line: {}", line));
    let captures = RE.captures(line).ok_or_else(build_option_error)?;

    let index_left = captures.get(1).ok_or_else(build_option_error)?;
    let index_right = captures.get(2).ok_or_else(build_option_error)?;
    let char = captures.get(3).ok_or_else(build_option_error)?;
    let password = captures.get(4).ok_or_else(build_option_error)?;

    Ok(Line {
        index_left: index_left.as_str().parse()?,
        index_right: index_right.as_str().parse()?,
        char: char::from_str(char.as_str())?,
        password: password.as_str().to_string(),
    })
}

fn is_valid_for_sled_rental(line: &Line) -> bool {
    let count = line.password.chars().filter(|c| &line.char == c).count() as u8;

    line.index_left <= count && count <= line.index_right
}

fn is_char_at_position(string: &str, index: u8, char: char) -> bool {
    match string.chars().nth((index - 1) as usize) {
        Some(c) => c == char,
        None => false,
    }
}

fn is_valid_for_otca(line: &Line) -> bool {
    is_char_at_position(&line.password, line.index_left, line.char)
        ^ is_char_at_position(&line.password, line.index_right, line.char)
}

#[cfg(test)]
mod tests {
    use hamcrest2::prelude::*;

    use super::*;

    #[test]
    fn count_valid_lines_returns_600() {
        let count = count_valid_lines();

        assert_that!(count, has((600, 245)))
    }

    #[test]
    fn test_parse_line_returns_populated_line_object() {
        let line1 = parse_line("1-2 z: qlfzd");
        let line2 = parse_line("5-7 a: abcd");

        assert_that!(&line1, ok());
        assert_that!(&line2, ok());

        let line1_unwrapped = line1.unwrap();
        let line2_unwrapped = line2.unwrap();

        assert_that!(line1_unwrapped.index_left, eq(1));
        assert_that!(line1_unwrapped.index_right, eq(2));
        assert_that!(line1_unwrapped.char, eq('z'));
        assert_that!(line1_unwrapped.password, eq("qlfzd"));

        assert_that!(line2_unwrapped.index_left, eq(5));
        assert_that!(line2_unwrapped.index_right, eq(7));
        assert_that!(line2_unwrapped.char, eq('a'));
        assert_that!(line2_unwrapped.password, eq("abcd"));
    }

    #[test]
    fn test_parse_line_returns_error_when_not_valid() {
        let line = parse_line("1-2: qlfzd");

        assert_that!(&line, err());
    }

    #[test]
    fn test_is_valid_for_sled_rental_returns_true_for_valid_lines() {
        assert_that!(
            is_valid_for_sled_rental(&Line {
                index_left: 1,
                index_right: 2,
                char: 'c',
                password: "acde".to_string(),
            }),
            eq(true)
        );
        assert_that!(
            is_valid_for_sled_rental(&Line {
                index_left: 4,
                index_right: 8,
                char: 'a',
                password: "aaaaaa".to_string(),
            }),
            eq(true)
        );
    }

    #[test]
    fn test_is_valid_for_sled_rental_returns_false_for_invalid_lines() {
        assert_that!(
            is_valid_for_sled_rental(&Line {
                index_left: 1,
                index_right: 2,
                char: 'c',
                password: "acdecc".to_string(),
            }),
            eq(false)
        );
        assert_that!(
            is_valid_for_sled_rental(&Line {
                index_left: 4,
                index_right: 8,
                char: 'a',
                password: "aaabbbb".to_string(),
            }),
            eq(false)
        );
    }

    #[test]
    fn test_is_valid_for_otca_returns_true_for_valid_lines() {
        assert_that!(
            is_valid_for_otca(&Line {
                index_left: 1,
                index_right: 3,
                char: 'a',
                password: "abcde".to_string(),
            }),
            eq(true)
        );
        assert_that!(
            is_valid_for_otca(&Line {
                index_left: 4,
                index_right: 8,
                char: 'a',
                password: "wwea".to_string(),
            }),
            eq(true)
        );
    }

    #[test]
    fn test_is_valid_for_otca_returns_false_for_invalid_lines() {
        assert_that!(
            is_valid_for_otca(&Line {
                index_left: 1,
                index_right: 3,
                char: 'b',
                password: "cdefg".to_string(),
            }),
            eq(false)
        );
        assert_that!(
            is_valid_for_otca(&Line {
                index_left: 2,
                index_right: 9,
                char: 'c',
                password: "ccccccccc".to_string(),
            }),
            eq(false)
        );
    }
}
