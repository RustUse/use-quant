use std::{error::Error, fmt};

/// A bar time label.
///
/// `BarTime` intentionally stores a string label instead of requiring a
/// concrete date/time dependency. This keeps `use-bar` lightweight and allows
/// callers to use timestamps, trading-session labels, vendor IDs, or other
/// domain-specific time keys.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct BarTime(String);

impl BarTime {
    /// Creates a bar time label.
    ///
    /// # Errors
    ///
    /// Returns [`BarTimeError::Empty`] when the label is empty or whitespace only.
    pub fn new(value: impl Into<String>) -> Result<Self, BarTimeError> {
        let value = value.into();

        if value.trim().is_empty() {
            return Err(BarTimeError::Empty);
        }

        Ok(Self(value))
    }

    /// Returns the underlying time label.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consumes the time label and returns the wrapped string.
    #[must_use]
    pub fn into_inner(self) -> String {
        self.0
    }
}

impl fmt::Display for BarTime {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl TryFrom<String> for BarTime {
    type Error = BarTimeError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl TryFrom<&str> for BarTime {
    type Error = BarTimeError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

/// Errors returned when creating a [`BarTime`].
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BarTimeError {
    /// The time label was empty or whitespace only.
    Empty,
}

impl fmt::Display for BarTimeError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("bar time label must not be empty"),
        }
    }
}

impl Error for BarTimeError {}

#[cfg(test)]
mod tests {
    use super::{BarTime, BarTimeError};
    use crate::BarError;

    #[test]
    fn constructs_valid_bar_time() {
        let time = BarTime::new("2026-05-17").expect("time should be valid");

        assert_eq!(time.as_str(), "2026-05-17");
        assert_eq!(time.to_string(), "2026-05-17");
    }

    #[test]
    fn preserves_original_label() {
        let time = BarTime::new("  session-open  ").expect("time should be valid");

        assert_eq!(time.as_str(), "  session-open  ");
    }

    #[test]
    fn rejects_empty_time() {
        assert_eq!(BarTime::new(""), Err(BarTimeError::Empty));
    }

    #[test]
    fn rejects_whitespace_only_time() {
        assert_eq!(BarTime::new("   "), Err(BarTimeError::Empty));
        assert_eq!(BarTime::new("\n\t"), Err(BarTimeError::Empty));
    }

    #[test]
    fn converts_from_string() {
        let time = BarTime::try_from(String::from("2026-05-17")).expect("time should be valid");

        assert_eq!(time.as_str(), "2026-05-17");
    }

    #[test]
    fn converts_from_str() {
        let time = BarTime::try_from("2026-05-17").expect("time should be valid");

        assert_eq!(time.as_str(), "2026-05-17");
    }

    #[test]
    fn converts_error_into_bar_error() {
        let error = BarError::from(BarTimeError::Empty);

        assert_eq!(error, BarError::InvalidTime(BarTimeError::Empty));
    }

    #[test]
    fn into_inner_returns_wrapped_string() {
        let time = BarTime::new("2026-05-17").expect("time should be valid");

        assert_eq!(time.into_inner(), "2026-05-17");
    }
}
