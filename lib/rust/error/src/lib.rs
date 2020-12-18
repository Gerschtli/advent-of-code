#![deny(warnings)]
#![allow(unused_imports)]

// necessary for intellij support
#[cfg(test)]
#[macro_use]
extern crate hamcrest2;

use std::char::ParseCharError;
use std::error::Error;
use std::fmt;
use std::io;
use std::num::ParseIntError;
use std::result;

pub type Result<T> = result::Result<T, AppError>;

#[derive(Debug)]
pub struct AppError {
    error: Option<Box<dyn Error>>,
    message: Option<String>,
}

impl Error for AppError {}

impl AppError {
    pub fn init<I>(message: I) -> Self
    where
        I: Into<String>,
    {
        AppError {
            error: None,
            message: Some(message.into()),
        }
    }

    pub fn init_err<I, E>(message: I, error: E) -> Self
    where
        I: Into<String>,
        E: Into<Box<dyn Error>>,
    {
        AppError {
            error: Some(error.into()),
            message: Some(message.into()),
        }
    }
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "app error")?;
        if let Some(m) = &self.message {
            write!(f, ": {}", m)?
        }
        if let Some(e) = &self.error {
            write!(f, ": {}", e)?
        }
        Ok(())
    }
}

macro_rules! from_error {
    ($error: ty) => {
        impl From<$error> for AppError {
            fn from(error: $error) -> Self {
                AppError {
                    error: Some(Box::from(error)),
                    message: None,
                }
            }
        }
    };
}

from_error!(io::Error);
from_error!(ParseCharError);
from_error!(ParseIntError);

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use hamcrest2::prelude::*;

    use super::*;

    #[test]
    fn app_error_init_sets_message() {
        assert_that!(format!("{}", AppError::init("test")), eq("app error: test"));
        assert_that!(
            format!("{}", AppError::init("test".to_string())),
            eq("app error: test")
        );
    }

    #[test]
    fn app_error_init_sets_message_with_error() {
        let error = io::Error::new(io::ErrorKind::AlreadyExists, "oops");
        assert_that!(
            format!("{}", AppError::init_err("test", error)),
            eq("app error: test: oops")
        );
    }

    #[test]
    fn app_error_converts_from_io_error() {
        let error = io::Error::new(io::ErrorKind::AlreadyExists, "oops");
        assert_that!(format!("{}", AppError::from(error)), eq("app error: oops"));
    }

    #[test]
    fn app_error_converts_from_parse_char_error() {
        let error = char::from_str("ab").err().unwrap();
        assert_that!(
            format!("{}", AppError::from(error)),
            eq("app error: too many characters in string")
        );
    }

    #[test]
    fn app_error_converts_from_parse_int_error() {
        let error = i32::from_str("ab").err().unwrap();
        assert_that!(
            format!("{}", AppError::from(error)),
            eq("app error: invalid digit found in string")
        );
    }
}
