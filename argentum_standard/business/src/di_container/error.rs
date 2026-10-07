use std::{error::Error, fmt};

#[derive(Debug)]
pub enum DiError {
    NotRegistered { type_name: &'static str },

    TypeMismatch { type_name: &'static str },

    Other(String),
}

impl fmt::Display for DiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotRegistered { type_name } => {
                write!(f, "DI service `{type_name}` is not registered")
            }

            Self::TypeMismatch { type_name } => {
                write!(f, "DI service `{type_name}` has wrong type")
            }

            Self::Other(message) => {
                write!(f, "{message}")
            }
        }
    }
}

impl Error for DiError {}
