#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct Passport {
    /// Birth Year
    byr: bool,
    /// Issue Year
    iyr: bool,
    /// Expiration Year
    eyr: bool,
    /// Height
    hgt: bool,
    /// Hair Color
    hcl: bool,
    /// Eye Color
    ecl: bool,
    /// Passport ID
    pid: bool,
    /// Country ID
    cid: bool,
}

impl Passport {
    pub(super) fn new() -> Self {
        Passport {
            byr: false,
            iyr: false,
            eyr: false,
            hgt: false,
            hcl: false,
            ecl: false,
            pid: false,
            cid: false,
        }
    }

    pub(super) fn is_valid(&self) -> bool {
        self.byr && self.iyr && self.eyr && self.hgt && self.hcl && self.ecl && self.pid
    }

    pub(super) fn with_byr(&self, value: &str) -> Self {
        let mut new = self.clone();
        new.byr = !value.is_empty();
        new
    }

    pub(super) fn with_iyr(&self, value: &str) -> Self {
        let mut new = self.clone();
        new.iyr = !value.is_empty();
        new
    }

    pub(super) fn with_eyr(&self, value: &str) -> Self {
        let mut new = self.clone();
        new.eyr = !value.is_empty();
        new
    }

    pub(super) fn with_hgt(&self, value: &str) -> Self {
        let mut new = self.clone();
        new.hgt = !value.is_empty();
        new
    }

    pub(super) fn with_hcl(&self, value: &str) -> Self {
        let mut new = self.clone();
        new.hcl = !value.is_empty();
        new
    }

    pub(super) fn with_ecl(&self, value: &str) -> Self {
        let mut new = self.clone();
        new.ecl = !value.is_empty();
        new
    }

    pub(super) fn with_pid(&self, value: &str) -> Self {
        let mut new = self.clone();
        new.pid = !value.is_empty();
        new
    }

    pub(super) fn with_cid(&self, value: &str) -> Self {
        let mut new = self.clone();
        new.cid = !value.is_empty();
        new
    }
}

#[cfg(test)]
mod tests {
    use hamcrest2::prelude::*;

    use super::*;

    #[test]
    fn passport_is_valid_needs_every_value_except_cid() {
        assert_that!(
            Passport {
                byr: true,
                iyr: true,
                eyr: true,
                hgt: true,
                hcl: true,
                ecl: true,
                pid: true,
                cid: true,
            }
            .is_valid(),
            eq(true)
        );
        assert_that!(
            Passport {
                byr: true,
                iyr: true,
                eyr: true,
                hgt: false,
                hcl: true,
                ecl: true,
                pid: true,
                cid: true,
            }
            .is_valid(),
            eq(false)
        );
        assert_that!(
            Passport {
                byr: true,
                iyr: true,
                eyr: true,
                hgt: true,
                hcl: true,
                ecl: true,
                pid: true,
                cid: false,
            }
            .is_valid(),
            eq(true)
        );
        assert_that!(
            Passport {
                byr: false,
                iyr: true,
                eyr: true,
                hgt: true,
                hcl: true,
                ecl: true,
                pid: true,
                cid: false,
            }
            .is_valid(),
            eq(false)
        );
    }
}
