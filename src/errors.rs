use std::{fmt::Display, error::Error};

#[derive(Debug, PartialEq)]
pub enum Errors {
    // I should add some more errors
    GenericError(String),
}

impl Display for Errors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let desc = match self {
            Errors::GenericError(msg) => msg,
        };

        write!(f, "PNG error: {:?}", desc)
    }
}

impl Error for Errors {}
