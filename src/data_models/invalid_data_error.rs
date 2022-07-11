use std::fmt;

#[derive(Debug, Clone)]
pub enum ErrorReason {
    None,
    Is(String),
}

#[derive(Debug, Clone)]
pub struct InvalidDataError {
    pub reason: ErrorReason,
}

impl InvalidDataError {
    pub fn new(reason: ErrorReason) -> InvalidDataError {
        InvalidDataError { reason: reason }
    }
}

impl fmt::Display for InvalidDataError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.reason {
            ErrorReason::None => write!(f, "Invalid Error with no reason given"),
            ErrorReason::Is(msg) => write!(f, "Invalid Error, {}", msg),
        }
    }
}
