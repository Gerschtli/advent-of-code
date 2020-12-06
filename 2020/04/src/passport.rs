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

    pub(super) fn with_byr(&self, _value: &str) -> Self {
        let mut new = self.clone();
        new.byr = PassportValue::init_present(false);
        new
    }

    pub(super) fn with_iyr(&self, _value: &str) -> Self {
        let mut new = self.clone();
        new.iyr = PassportValue::init_present(false);
        new
    }

    pub(super) fn with_eyr(&self, _value: &str) -> Self {
        let mut new = self.clone();
        new.eyr = PassportValue::init_present(false);
        new
    }

    pub(super) fn with_hgt(&self, _value: &str) -> Self {
        let mut new = self.clone();
        new.hgt = PassportValue::init_present(false);
        new
    }

    pub(super) fn with_hcl(&self, _value: &str) -> Self {
        let mut new = self.clone();
        new.hcl = PassportValue::init_present(false);
        new
    }

    pub(super) fn with_ecl(&self, _value: &str) -> Self {
        let mut new = self.clone();
        new.ecl = PassportValue::init_present(false);
        new
    }

    pub(super) fn with_pid(&self, _value: &str) -> Self {
        let mut new = self.clone();
        new.pid = PassportValue::init_present(false);
        new
    }

    pub(super) fn with_cid(&self, _value: &str) -> Self {
        let mut new = self.clone();
        new.cid = PassportValue::init_present(true);
        new
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
}
