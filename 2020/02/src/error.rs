use std::char::ParseCharError;
use std::error::Error;
use std::fmt;
use std::io;
use std::num::ParseIntError;

#[derive(Debug)]
pub(super) struct AppError {
    error: Option<Box<dyn Error>>,
    message: Option<String>,
}

impl AppError {
    pub(super) fn invalid_line(line: &str) -> Self {
        AppError {
            error: None,
            message: Some(format!("invalid line: {}", line)),
        }
    }
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "app error:")?;
        if let Some(m) = &self.message {
            write!(f, " {}", m)?
        }
        if let Some(e) = &self.error {
            write!(f, " {}", e)?
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
from_error!(regex::Error);
from_error!(ParseCharError);
from_error!(ParseIntError);