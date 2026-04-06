use crate::KiotaError;
use std::fmt;

/// ISO 8601 duration string wrapper.
///
/// chrono::Duration doesn't support ISO 8601 string parsing/formatting,
/// so we store the raw string and validate it on construction.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IsoDuration {
    raw: String,
}

impl IsoDuration {
    pub fn parse(s: &str) -> Result<Self, KiotaError> {
        // Basic validation: must start with P
        if !s.starts_with('P') {
            return Err(KiotaError::Deserialization(format!(
                "invalid ISO 8601 duration: {s}"
            )));
        }
        Ok(Self {
            raw: s.to_string(),
        })
    }

    pub fn as_str(&self) -> &str {
        &self.raw
    }
}

impl fmt::Display for IsoDuration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.raw)
    }
}
