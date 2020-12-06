// necessary for intellij support
#[cfg(test)]
#[macro_use]
extern crate hamcrest2;

use std::io::BufRead;
use std::{fs, io, path};

use crate::error::AppError;
use crate::error::Result;
use crate::passport::Passport;

mod error;
mod passport;

fn main() -> Result<()> {
    let count_with_necessary_properties = count_passports()?;

    println!(
        "Count of passports with necessary properties: {}",
        count_with_necessary_properties
    );

    Ok(())
}

fn count_passports() -> Result<usize> {
    let lines = read_lines("./files/passports.txt")?;
    let passports = parse_lines(&lines)?;

    let count_with_necessary_properties = passports
        .iter()
        .filter(|p| p.has_necessary_properties())
        .count();

    Ok(count_with_necessary_properties)
}

fn read_lines<P>(filename: P) -> io::Result<Vec<String>>
where
    P: AsRef<path::Path>,
{
    let file = fs::File::open(filename)?;
    let lines_raw = io::BufReader::new(file).lines();

    lines_raw.into_iter().collect()
}

fn parse_lines(lines: &[String]) -> Result<Vec<Passport>> {
    let mut passports = Vec::<Passport>::new();
    let mut current_passport_data = Vec::<&str>::new();

    for line in lines {
        if line.is_empty() {
            passports.push(build_passport(&current_passport_data)?);
            current_passport_data.clear();
        } else {
            current_passport_data.extend_from_slice(&line.split(' ').collect::<Vec<_>>());
        }
    }

    passports.push(build_passport(&current_passport_data)?);

    Ok(passports)
}

fn build_passport(data: &[&str]) -> Result<Passport> {
    let mut passport = Passport::new();

    for datum in data {
        let mut split = datum.split(':');
        let key = split.next();
        let value = split.next();

        if key.is_none() || value.is_none() {
            return Err(AppError::init(&format!("invalid password data: {}", datum)));
        }

        passport = match key.unwrap() {
            "byr" => passport.with_byr(value.unwrap()),
            "iyr" => passport.with_iyr(value.unwrap()),
            "eyr" => passport.with_eyr(value.unwrap()),
            "hgt" => passport.with_hgt(value.unwrap()),
            "hcl" => passport.with_hcl(value.unwrap()),
            "ecl" => passport.with_ecl(value.unwrap()),
            "pid" => passport.with_pid(value.unwrap()),
            "cid" => passport.with_cid(value.unwrap()),
            _ => return Err(AppError::init(&format!("invalid password data: {}", datum))),
        };
    }

    Ok(passport)
}

#[cfg(test)]
mod tests {
    use hamcrest2::prelude::*;

    use super::*;

    #[test]
    fn count_passports_returns_202() {
        assert_that!(count_passports(), has(202));
    }

    #[test]
    fn read_lines_returns_every_line_from_file() {
        let lines = read_lines("./files/example.txt");

        assert_that!(
            lines,
            has(vec![
                "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd".to_string(),
                "".to_string(),
                "byr:1937 iyr:2017 cid:147 hgt:183cm".to_string(),
            ])
        );
    }

    #[test]
    fn read_lines_returns_error_when_file_not_found() {
        let lines = read_lines("./files/example_not_found.txt");

        assert_that!(lines, err());
    }

    #[test]
    fn parse_lines_returns_vec_of_passports() {
        let lines = vec![
            "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd".to_string(),
            "byr:1937 iyr:2017 cid:147 hgt:183cm".to_string(),
            "".to_string(),
            "iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884".to_string(),
            "hcl:#cfa07d byr:1929".to_string(),
        ];

        let passports = parse_lines(&lines);

        assert_that!(&passports, ok());

        let expected = vec![
            Passport::new()
                .with_byr("1937")
                .with_iyr("2017")
                .with_eyr("2020")
                .with_hgt("183cm")
                .with_hcl("#fffffd")
                .with_ecl("gry")
                .with_pid("860033327")
                .with_cid("147"),
            Passport::new()
                .with_byr("1929")
                .with_iyr("2013")
                .with_eyr("2023")
                .with_hcl("#cfa07d")
                .with_ecl("amb")
                .with_pid("028048884")
                .with_cid("350"),
        ];

        assert_that!(&passports.unwrap(), contains(expected).exactly());
    }
}
