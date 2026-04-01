use std::error::Error;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GeometryError {
    ZeroDimension { field: &'static str },
}

impl fmt::Display for GeometryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GeometryError::ZeroDimension { field } => {
                write!(f, "{field} must be greater than zero")
            }
        }
    }
}

impl Error for GeometryError {}
