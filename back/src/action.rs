use std::{fmt, str};

#[derive(Debug, Copy, Clone)]
pub enum Action {
    EndTurn,
}

#[derive(Debug)]
pub enum ParseError {
    Unknown,
}

impl str::FromStr for Action {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "E" {
            Ok(Action::EndTurn)
        } else {
            Err(ParseError::Unknown)
        }
    }
}

impl fmt::Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EndTurn => write!(f, "E"),
        }
    }
}
