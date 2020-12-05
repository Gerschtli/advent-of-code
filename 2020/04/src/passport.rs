#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct Passport {
    /// Birth Year
    pub(super) byr: bool,
    /// Issue Year
    pub(super) iyr: bool,
    /// Expiration Year
    pub(super) eyr: bool,
    /// Height
    pub(super) hgt: bool,
    /// Hair Color
    pub(super) hcl: bool,
    /// Eye Color
    pub(super) ecl: bool,
    /// Passport ID
    pub(super) pid: bool,
    /// Country ID
    pub(super) cid: bool,
}

impl Passport {
    pub(super) fn is_valid(&self) -> bool {
        self.byr && self.iyr && self.eyr && self.hgt && self.hcl && self.ecl && self.pid
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
