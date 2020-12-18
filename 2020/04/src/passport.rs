use lazy_static::lazy_static;
use regex::Regex;

#[derive(Clone, Debug, Eq, PartialEq)]
struct PassportValue {
    is_present: bool,
    is_valid: bool,
}

impl PassportValue {
    fn new() -> Self {
        PassportValue {
            is_present: false,
            is_valid: false,
        }
    }

    fn init_present(is_valid: bool) -> Self {
        PassportValue {
            is_present: true,
            is_valid,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct Passport {
    /// Birth Year
    byr: PassportValue,
    /// Issue Year
    iyr: PassportValue,
    /// Expiration Year
    eyr: PassportValue,
    /// Height
    hgt: PassportValue,
    /// Hair Color
    hcl: PassportValue,
    /// Eye Color
    ecl: PassportValue,
    /// Passport ID
    pid: PassportValue,
    /// Country ID
    cid: PassportValue,
}

impl Passport {
    pub(super) fn new() -> Self {
        Passport {
            byr: PassportValue::new(),
            iyr: PassportValue::new(),
            eyr: PassportValue::new(),
            hgt: PassportValue::new(),
            hcl: PassportValue::new(),
            ecl: PassportValue::new(),
            pid: PassportValue::new(),
            cid: PassportValue::new(),
        }
    }

    pub(super) fn has_necessary_properties(&self) -> bool {
        self.byr.is_present
            && self.iyr.is_present
            && self.eyr.is_present
            && self.hgt.is_present
            && self.hcl.is_present
            && self.ecl.is_present
            && self.pid.is_present
    }

    pub(super) fn is_valid(&self) -> bool {
        self.byr.is_valid
            && self.iyr.is_valid
            && self.eyr.is_valid
            && self.hgt.is_valid
            && self.hcl.is_valid
            && self.ecl.is_valid
            && self.pid.is_valid
    }

    pub(super) fn with_byr(&self, value: &str) -> Self {
        let valid = Passport::is_valid_year(value, 1920, 2002);

        let mut new = self.clone();
        new.byr = PassportValue::init_present(valid);
        new
    }

    pub(super) fn with_iyr(&self, value: &str) -> Self {
        let valid = Passport::is_valid_year(value, 2010, 2020);

        let mut new = self.clone();
        new.iyr = PassportValue::init_present(valid);
        new
    }

    pub(super) fn with_eyr(&self, value: &str) -> Self {
        let valid = Passport::is_valid_year(value, 2020, 2030);

        let mut new = self.clone();
        new.eyr = PassportValue::init_present(valid);
        new
    }

    pub(super) fn with_hgt(&self, value: &str) -> Self {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^(\d+)(cm|in)$").unwrap();
        }

        let valid = match RE.captures(value) {
            Some(captures) => {
                let capture_value = captures.get(1);
                let capture_unit = captures.get(2);

                match (capture_value, capture_unit) {
                    (Some(value), Some(unit)) => {
                        if unit.as_str() == "cm" {
                            Passport::is_in_range(value.as_str(), 150, 193)
                        } else {
                            Passport::is_in_range(value.as_str(), 59, 76)
                        }
                    }
                    _ => false,
                }
            }
            None => false,
        };

        let mut new = self.clone();
        new.hgt = PassportValue::init_present(valid);
        new
    }

    pub(super) fn with_hcl(&self, value: &str) -> Self {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^#[\da-f]{6}$").unwrap();
        }

        let valid = RE.is_match(value);

        let mut new = self.clone();
        new.hcl = PassportValue::init_present(valid);
        new
    }

    pub(super) fn with_ecl(&self, value: &str) -> Self {
        let valid = matches!(value, "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth");

        let mut new = self.clone();
        new.ecl = PassportValue::init_present(valid);
        new
    }

    pub(super) fn with_pid(&self, value: &str) -> Self {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^\d{9}$").unwrap();
        }

        let valid = RE.is_match(value);

        let mut new = self.clone();
        new.pid = PassportValue::init_present(valid);
        new
    }

    pub(super) fn with_cid(&self, _value: &str) -> Self {
        let mut new = self.clone();
        new.cid = PassportValue::init_present(true);
        new
    }

    fn is_valid_year(value: &str, min: i32, max: i32) -> bool {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^\d{4}$").unwrap();
        }

        if !RE.is_match(value) {
            return false;
        }

        Passport::is_in_range(value, min, max)
    }

    fn is_in_range(value: &str, min: i32, max: i32) -> bool {
        let v = value.parse::<i32>().unwrap();

        min <= v && v <= max
    }
}

#[cfg(test)]
mod tests {
    use hamcrest2::prelude::*;

    use super::*;

    #[test]
    fn passport_value_new_sets_all_to_false() {
        let value = PassportValue::new();

        assert_that!(value.is_present, eq(false));
        assert_that!(value.is_valid, eq(false));
    }

    #[test]
    fn passport_value_init_present_sets_valid_to_input() {
        let value_valid = PassportValue::init_present(true);
        let value_invalid = PassportValue::init_present(false);

        assert_that!(value_valid.is_present, eq(true));
        assert_that!(value_valid.is_valid, eq(true));
        assert_that!(value_invalid.is_present, eq(true));
        assert_that!(value_invalid.is_valid, eq(false));
    }

    #[test]
    fn passport_new_sets_all_to_false() {
        let passport = Passport::new();

        assert_that!(
            passport.byr,
            eq(PassportValue {
                is_present: false,
                is_valid: false
            })
        );
        assert_that!(
            passport.iyr,
            eq(PassportValue {
                is_present: false,
                is_valid: false
            })
        );
        assert_that!(
            passport.eyr,
            eq(PassportValue {
                is_present: false,
                is_valid: false
            })
        );
        assert_that!(
            passport.hgt,
            eq(PassportValue {
                is_present: false,
                is_valid: false
            })
        );
        assert_that!(
            passport.hcl,
            eq(PassportValue {
                is_present: false,
                is_valid: false
            })
        );
        assert_that!(
            passport.ecl,
            eq(PassportValue {
                is_present: false,
                is_valid: false
            })
        );
        assert_that!(
            passport.pid,
            eq(PassportValue {
                is_present: false,
                is_valid: false
            })
        );
        assert_that!(
            passport.cid,
            eq(PassportValue {
                is_present: false,
                is_valid: false
            })
        );
    }

    #[test]
    fn passport_has_necessary_properties_needs_every_value_except_cid() {
        assert_that!(
            Passport {
                byr: PassportValue::init_present(false),
                iyr: PassportValue::init_present(false),
                eyr: PassportValue::init_present(false),
                hgt: PassportValue::init_present(false),
                hcl: PassportValue::init_present(false),
                ecl: PassportValue::init_present(false),
                pid: PassportValue::init_present(false),
                cid: PassportValue::init_present(false),
            }
            .has_necessary_properties(),
            eq(true)
        );
        assert_that!(
            Passport {
                byr: PassportValue::init_present(false),
                iyr: PassportValue::init_present(false),
                eyr: PassportValue::init_present(false),
                hgt: PassportValue::new(),
                hcl: PassportValue::init_present(false),
                ecl: PassportValue::init_present(false),
                pid: PassportValue::init_present(false),
                cid: PassportValue::init_present(false),
            }
            .has_necessary_properties(),
            eq(false)
        );
        assert_that!(
            Passport {
                byr: PassportValue::init_present(false),
                iyr: PassportValue::init_present(false),
                eyr: PassportValue::init_present(false),
                hgt: PassportValue::init_present(false),
                hcl: PassportValue::init_present(false),
                ecl: PassportValue::init_present(false),
                pid: PassportValue::init_present(false),
                cid: PassportValue::new(),
            }
            .has_necessary_properties(),
            eq(true)
        );
        assert_that!(
            Passport {
                byr: PassportValue::new(),
                iyr: PassportValue::init_present(false),
                eyr: PassportValue::init_present(false),
                hgt: PassportValue::init_present(false),
                hcl: PassportValue::init_present(false),
                ecl: PassportValue::init_present(false),
                pid: PassportValue::init_present(false),
                cid: PassportValue::new(),
            }
            .has_necessary_properties(),
            eq(false)
        );
    }

    #[test]
    fn passport_is_valid_needs_every_value_to_be_valid_except_cid() {
        assert_that!(
            Passport {
                byr: PassportValue::init_present(true),
                iyr: PassportValue::init_present(true),
                eyr: PassportValue::init_present(true),
                hgt: PassportValue::init_present(true),
                hcl: PassportValue::init_present(true),
                ecl: PassportValue::init_present(true),
                pid: PassportValue::init_present(true),
                cid: PassportValue::init_present(true),
            }
            .is_valid(),
            eq(true)
        );
        assert_that!(
            Passport {
                byr: PassportValue::init_present(true),
                iyr: PassportValue::init_present(true),
                eyr: PassportValue::init_present(false),
                hgt: PassportValue::init_present(true),
                hcl: PassportValue::init_present(true),
                ecl: PassportValue::init_present(true),
                pid: PassportValue::init_present(true),
                cid: PassportValue::init_present(true),
            }
            .is_valid(),
            eq(false)
        );
        assert_that!(
            Passport {
                byr: PassportValue::init_present(true),
                iyr: PassportValue::init_present(true),
                eyr: PassportValue::init_present(true),
                hgt: PassportValue::init_present(true),
                hcl: PassportValue::init_present(true),
                ecl: PassportValue::init_present(true),
                pid: PassportValue::init_present(true),
                cid: PassportValue::init_present(false),
            }
            .is_valid(),
            eq(true)
        );
        assert_that!(
            Passport {
                byr: PassportValue::init_present(false),
                iyr: PassportValue::init_present(true),
                eyr: PassportValue::init_present(true),
                hgt: PassportValue::init_present(true),
                hcl: PassportValue::init_present(true),
                ecl: PassportValue::init_present(true),
                pid: PassportValue::init_present(true),
                cid: PassportValue::init_present(false),
            }
            .is_valid(),
            eq(false)
        );
    }

    #[test]
    fn passport_with_byr_validates_value() {
        let passport = Passport::new();
        let value_invalid = PassportValue::init_present(false);
        let value_valid = PassportValue::init_present(true);

        assert_that!(&passport.with_byr("1919").byr, eq(&value_invalid));
        assert_that!(&passport.with_byr("1920").byr, eq(&value_valid));
        assert_that!(&passport.with_byr("2002").byr, eq(&value_valid));
        assert_that!(&passport.with_byr("2003").byr, eq(&value_invalid));
        assert_that!(&passport.with_byr("abcd").byr, eq(&value_invalid));
    }

    #[test]
    fn passport_with_iyr_validates_value() {
        let passport = Passport::new();
        let value_invalid = PassportValue::init_present(false);
        let value_valid = PassportValue::init_present(true);

        assert_that!(&passport.with_iyr("2009").iyr, eq(&value_invalid));
        assert_that!(&passport.with_iyr("2010").iyr, eq(&value_valid));
        assert_that!(&passport.with_iyr("2020").iyr, eq(&value_valid));
        assert_that!(&passport.with_iyr("2021").iyr, eq(&value_invalid));
        assert_that!(&passport.with_iyr("abcd").iyr, eq(&value_invalid));
    }

    #[test]
    fn passport_with_eyr_validates_value() {
        let passport = Passport::new();
        let value_invalid = PassportValue::init_present(false);
        let value_valid = PassportValue::init_present(true);

        assert_that!(&passport.with_eyr("2019").eyr, eq(&value_invalid));
        assert_that!(&passport.with_eyr("2020").eyr, eq(&value_valid));
        assert_that!(&passport.with_eyr("2030").eyr, eq(&value_valid));
        assert_that!(&passport.with_eyr("2031").eyr, eq(&value_invalid));
        assert_that!(&passport.with_eyr("abcd").eyr, eq(&value_invalid));
    }

    #[test]
    fn passport_with_hgt_validates_value() {
        let passport = Passport::new();
        let value_invalid = PassportValue::init_present(false);
        let value_valid = PassportValue::init_present(true);

        assert_that!(&passport.with_hgt("149cm").hgt, eq(&value_invalid));
        assert_that!(&passport.with_hgt("150cm").hgt, eq(&value_valid));
        assert_that!(&passport.with_hgt("193cm").hgt, eq(&value_valid));
        assert_that!(&passport.with_hgt("194cm").hgt, eq(&value_invalid));
        assert_that!(&passport.with_hgt("58in").hgt, eq(&value_invalid));
        assert_that!(&passport.with_hgt("59in").hgt, eq(&value_valid));
        assert_that!(&passport.with_hgt("76in").hgt, eq(&value_valid));
        assert_that!(&passport.with_hgt("77in").hgt, eq(&value_invalid));
        assert_that!(&passport.with_hgt("abcd").hgt, eq(&value_invalid));
    }

    #[test]
    fn passport_with_hcl_validates_value() {
        let passport = Passport::new();
        let value_invalid = PassportValue::init_present(false);
        let value_valid = PassportValue::init_present(true);

        assert_that!(&passport.with_hcl("#000000").hcl, eq(&value_valid));
        assert_that!(&passport.with_hcl("#abcdef").hcl, eq(&value_valid));
        assert_that!(&passport.with_hcl("#18e6d7").hcl, eq(&value_valid));
        assert_that!(&passport.with_hcl("#123g45").hcl, eq(&value_invalid));
        assert_that!(&passport.with_hcl("000000").hcl, eq(&value_invalid));
    }

    #[test]
    fn passport_with_ecl_validates_value() {
        let passport = Passport::new();
        let value_invalid = PassportValue::init_present(false);
        let value_valid = PassportValue::init_present(true);

        assert_that!(&passport.with_ecl("amb").ecl, eq(&value_valid));
        assert_that!(&passport.with_ecl("blu").ecl, eq(&value_valid));
        assert_that!(&passport.with_ecl("brn").ecl, eq(&value_valid));
        assert_that!(&passport.with_ecl("gry").ecl, eq(&value_valid));
        assert_that!(&passport.with_ecl("grn").ecl, eq(&value_valid));
        assert_that!(&passport.with_ecl("hzl").ecl, eq(&value_valid));
        assert_that!(&passport.with_ecl("oth").ecl, eq(&value_valid));

        assert_that!(&passport.with_ecl("abc").ecl, eq(&value_invalid));
    }

    #[test]
    fn passport_with_pid_validates_value() {
        let passport = Passport::new();
        let value_invalid = PassportValue::init_present(false);
        let value_valid = PassportValue::init_present(true);

        assert_that!(&passport.with_pid("000000000").pid, eq(&value_valid));
        assert_that!(&passport.with_pid("123456789").pid, eq(&value_valid));
        assert_that!(&passport.with_pid("000123456").pid, eq(&value_valid));
        assert_that!(&passport.with_pid("4567").pid, eq(&value_invalid));
    }

    #[test]
    fn passport_with_cid_returns_always_valid() {
        let passport = Passport::new();
        let value_valid = PassportValue::init_present(true);

        assert_that!(&passport.with_cid("123").cid, eq(&value_valid));
        assert_that!(&passport.with_cid("").cid, eq(&value_valid));
    }
}
