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

    fn init_present() -> Self {
        PassportValue {
            is_present: true,
            is_valid: false,
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
        new.byr = PassportValue::init_present();
        new
    }

    pub(super) fn with_iyr(&self, _value: &str) -> Self {
        let mut new = self.clone();
        new.iyr = PassportValue::init_present();
        new
    }

    pub(super) fn with_eyr(&self, _value: &str) -> Self {
        let mut new = self.clone();
        new.eyr = PassportValue::init_present();
        new
    }

    pub(super) fn with_hgt(&self, _value: &str) -> Self {
        let mut new = self.clone();
        new.hgt = PassportValue::init_present();
        new
    }

    pub(super) fn with_hcl(&self, _value: &str) -> Self {
        let mut new = self.clone();
        new.hcl = PassportValue::init_present();
        new
    }

    pub(super) fn with_ecl(&self, _value: &str) -> Self {
        let mut new = self.clone();
        new.ecl = PassportValue::init_present();
        new
    }

    pub(super) fn with_pid(&self, _value: &str) -> Self {
        let mut new = self.clone();
        new.pid = PassportValue::init_present();
        new
    }

    pub(super) fn with_cid(&self, _value: &str) -> Self {
        let mut new = self.clone();
        new.cid = PassportValue::init_present();
        new
    }
}

#[cfg(test)]
mod tests {
    use hamcrest2::prelude::*;

    use super::*;

    #[test]
    fn passport_has_necessary_properties_needs_every_value_except_cid() {
        assert_that!(
            Passport {
                byr: PassportValue::init_present(),
                iyr: PassportValue::init_present(),
                eyr: PassportValue::init_present(),
                hgt: PassportValue::init_present(),
                hcl: PassportValue::init_present(),
                ecl: PassportValue::init_present(),
                pid: PassportValue::init_present(),
                cid: PassportValue::init_present(),
            }
            .has_necessary_properties(),
            eq(true)
        );
        assert_that!(
            Passport {
                byr: PassportValue::init_present(),
                iyr: PassportValue::init_present(),
                eyr: PassportValue::init_present(),
                hgt: PassportValue::new(),
                hcl: PassportValue::init_present(),
                ecl: PassportValue::init_present(),
                pid: PassportValue::init_present(),
                cid: PassportValue::init_present(),
            }
            .has_necessary_properties(),
            eq(false)
        );
        assert_that!(
            Passport {
                byr: PassportValue::init_present(),
                iyr: PassportValue::init_present(),
                eyr: PassportValue::init_present(),
                hgt: PassportValue::init_present(),
                hcl: PassportValue::init_present(),
                ecl: PassportValue::init_present(),
                pid: PassportValue::init_present(),
                cid: PassportValue::new(),
            }
            .has_necessary_properties(),
            eq(true)
        );
        assert_that!(
            Passport {
                byr: PassportValue::new(),
                iyr: PassportValue::init_present(),
                eyr: PassportValue::init_present(),
                hgt: PassportValue::init_present(),
                hcl: PassportValue::init_present(),
                ecl: PassportValue::init_present(),
                pid: PassportValue::init_present(),
                cid: PassportValue::new(),
            }
            .has_necessary_properties(),
            eq(false)
        );
    }
}
