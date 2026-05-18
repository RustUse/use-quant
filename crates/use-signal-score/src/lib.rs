#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::{fmt, str::FromStr};
use std::error::Error;

/// Common signal score primitives.
pub mod prelude {
    pub use crate::{
        SignalDirection, SignalDirectionParseError, SignalError, SignalName, SignalScore,
        SignalStrength, SignalStrengthParseError,
    };
}

/// A non-empty signal name.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct SignalName(String);

impl SignalName {
    /// Creates a signal name from non-empty text.
    ///
    /// # Errors
    ///
    /// Returns [`SignalError::EmptyName`] when the trimmed value is empty.
    pub fn new(value: impl AsRef<str>) -> Result<Self, SignalError> {
        let trimmed = value.as_ref().trim();
        if trimmed.is_empty() {
            Err(SignalError::EmptyName)
        } else {
            Ok(Self(trimmed.to_string()))
        }
    }

    /// Returns the signal name.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for SignalName {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for SignalName {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for SignalName {
    type Err = SignalError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

/// Descriptive signal direction vocabulary.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum SignalDirection {
    /// Long direction vocabulary.
    Long,
    /// Short direction vocabulary.
    Short,
    /// Neutral direction vocabulary.
    Neutral,
    /// Unknown direction vocabulary.
    Unknown,
    /// Caller-defined direction vocabulary.
    Custom(String),
}

impl fmt::Display for SignalDirection {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::Long => "long",
            Self::Short => "short",
            Self::Neutral => "neutral",
            Self::Unknown => "unknown",
            Self::Custom(value) => value.as_str(),
        })
    }
}

impl FromStr for SignalDirection {
    type Err = SignalDirectionParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let trimmed = value.trim();
        if trimmed.is_empty() {
            return Err(SignalDirectionParseError::Empty);
        }

        match normalized_token(trimmed).as_str() {
            "long" => Ok(Self::Long),
            "short" => Ok(Self::Short),
            "neutral" => Ok(Self::Neutral),
            "unknown" => Ok(Self::Unknown),
            _ => Ok(Self::Custom(trimmed.to_string())),
        }
    }
}

/// Errors returned while parsing signal directions.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SignalDirectionParseError {
    /// The input was empty after trimming whitespace.
    Empty,
}

impl fmt::Display for SignalDirectionParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("signal direction cannot be empty"),
        }
    }
}

impl Error for SignalDirectionParseError {}

/// Descriptive signal strength vocabulary.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum SignalStrength {
    /// Weak strength vocabulary.
    Weak,
    /// Moderate strength vocabulary.
    Moderate,
    /// Strong strength vocabulary.
    Strong,
    /// Unknown strength vocabulary.
    Unknown,
    /// Caller-defined strength vocabulary.
    Custom(String),
}

impl fmt::Display for SignalStrength {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::Weak => "weak",
            Self::Moderate => "moderate",
            Self::Strong => "strong",
            Self::Unknown => "unknown",
            Self::Custom(value) => value.as_str(),
        })
    }
}

impl FromStr for SignalStrength {
    type Err = SignalStrengthParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let trimmed = value.trim();
        if trimmed.is_empty() {
            return Err(SignalStrengthParseError::Empty);
        }

        match normalized_token(trimmed).as_str() {
            "weak" => Ok(Self::Weak),
            "moderate" => Ok(Self::Moderate),
            "strong" => Ok(Self::Strong),
            "unknown" => Ok(Self::Unknown),
            _ => Ok(Self::Custom(trimmed.to_string())),
        }
    }
}

/// Errors returned while parsing signal strengths.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SignalStrengthParseError {
    /// The input was empty after trimming whitespace.
    Empty,
}

impl fmt::Display for SignalStrengthParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("signal strength cannot be empty"),
        }
    }
}

impl Error for SignalStrengthParseError {}

/// A finite quantitative signal score with descriptive semantics.
#[derive(Clone, Debug, PartialEq)]
pub struct SignalScore {
    name: SignalName,
    score: f64,
    direction: SignalDirection,
    strength: SignalStrength,
}

impl SignalScore {
    /// Creates a signal score with unknown direction and strength.
    ///
    /// # Errors
    ///
    /// Returns [`SignalError::NonFiniteScore`] when `score` is not finite.
    pub fn new(name: SignalName, score: f64) -> Result<Self, SignalError> {
        if !score.is_finite() {
            return Err(SignalError::NonFiniteScore);
        }

        Ok(Self {
            name,
            score,
            direction: SignalDirection::Unknown,
            strength: SignalStrength::Unknown,
        })
    }

    /// Sets descriptive direction vocabulary.
    #[must_use]
    pub fn with_direction(mut self, direction: SignalDirection) -> Self {
        self.direction = direction;
        self
    }

    /// Sets descriptive strength vocabulary.
    #[must_use]
    pub fn with_strength(mut self, strength: SignalStrength) -> Self {
        self.strength = strength;
        self
    }

    /// Returns the signal name.
    #[must_use]
    pub const fn name(&self) -> &SignalName {
        &self.name
    }

    /// Returns the numeric score.
    #[must_use]
    pub const fn score(&self) -> f64 {
        self.score
    }

    /// Returns the descriptive direction.
    #[must_use]
    pub const fn direction(&self) -> &SignalDirection {
        &self.direction
    }

    /// Returns the descriptive strength.
    #[must_use]
    pub const fn strength(&self) -> &SignalStrength {
        &self.strength
    }
}

/// Errors returned by signal score helpers.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SignalError {
    /// Names must be non-empty after trimming whitespace.
    EmptyName,
    /// Scores must be finite.
    NonFiniteScore,
}

impl fmt::Display for SignalError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyName => formatter.write_str("signal name cannot be empty"),
            Self::NonFiniteScore => formatter.write_str("signal score must be finite"),
        }
    }
}

impl Error for SignalError {}

fn normalized_token(value: &str) -> String {
    value
        .trim()
        .chars()
        .map(|character| match character {
            '_' | ' ' => '-',
            other => other.to_ascii_lowercase(),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{SignalDirection, SignalError, SignalName, SignalScore, SignalStrength};

    #[test]
    fn accepts_valid_signal_name() {
        let name = SignalName::new("quality-score").expect("name should be valid");

        assert_eq!(name.as_str(), "quality-score");
    }

    #[test]
    fn rejects_empty_signal_name() {
        assert_eq!(SignalName::new(" "), Err(SignalError::EmptyName));
    }

    #[test]
    fn constructs_score() {
        let score = SignalScore::new(
            SignalName::new("momentum").expect("name should be valid"),
            1.2,
        )
        .expect("score should be valid");

        assert_eq!(score.score(), 1.2);
    }

    #[test]
    fn displays_and_parses_direction() {
        let direction: SignalDirection = "long".parse().expect("direction should parse");

        assert_eq!(direction, SignalDirection::Long);
        assert_eq!(direction.to_string(), "long");
    }

    #[test]
    fn displays_and_parses_strength() {
        let strength: SignalStrength = "moderate".parse().expect("strength should parse");

        assert_eq!(strength, SignalStrength::Moderate);
        assert_eq!(strength.to_string(), "moderate");
    }

    #[test]
    fn supports_custom_values() {
        let direction: SignalDirection = "pair".parse().expect("direction should parse");
        let strength: SignalStrength = "screen-only".parse().expect("strength should parse");

        assert_eq!(direction, SignalDirection::Custom("pair".to_string()));
        assert_eq!(strength, SignalStrength::Custom("screen-only".to_string()));
    }
}
