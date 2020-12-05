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
    let count = count_passports()?;

    println!("Count of valid passports: {}", count);

    Ok(())
}

fn count_passports() -> Result<usize> {
    let lines = read_lines("./files/passports.txt")?;
    let passports = parse_lines(&lines)?;

    Ok(passports.iter().filter(|p| p.is_valid()).count())
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
    let mut passport = Passport {
        byr: false,
        iyr: false,
        eyr: false,
        hgt: false,
        hcl: false,
        ecl: false,
        pid: false,
        cid: false,
    };

    for datum in data {
        let key = datum.split(':').next();

        if key.is_none() {
            return Err(AppError::init(&format!("invalid password data: {}", datum)));
        }

        match key.unwrap() {
            "byr" => passport.byr = true,
            "iyr" => passport.iyr = true,
            "eyr" => passport.eyr = true,
            "hgt" => passport.hgt = true,
            "hcl" => passport.hcl = true,
            "ecl" => passport.ecl = true,
            "pid" => passport.pid = true,
            "cid" => passport.cid = true,
            _ => return Err(AppError::init(&format!("invalid password data: {}", datum))),
        }
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
            Passport {
                byr: true,
                iyr: true,
                eyr: true,
                hgt: true,
                hcl: true,
                ecl: true,
                pid: true,
                cid: true,
            },
            Passport {
                byr: true,
                iyr: true,
                eyr: true,
                hgt: false,
                hcl: true,
                ecl: true,
                pid: true,
                cid: true,
            },
        ];

        assert_that!(&passports.unwrap(), contains(expected).exactly());
    }
}
