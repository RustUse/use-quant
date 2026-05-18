#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::{fmt, str::FromStr};
use std::error::Error;

/// Common volatility primitives.
pub mod prelude {
    pub use crate::{
        Volatility, VolatilityError, VolatilityKind, VolatilityKindParseError, VolatilityWindow,
    };
}

/// A finite non-negative volatility value.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Volatility {
    value: f64,
}

impl Volatility {
    /// Creates a volatility value.
    ///
    /// # Errors
    ///
    /// Returns [`VolatilityError::NonFinite`] or [`VolatilityError::Negative`] when `value` is
    /// invalid.
    pub fn new(value: f64) -> Result<Self, VolatilityError> {
        if !value.is_finite() {
            return Err(VolatilityError::NonFinite);
        }

        if value < 0.0 {
            return Err(VolatilityError::Negative);
        }

        Ok(Self { value })
    }

    /// Computes sample standard-deviation volatility from return values.
    ///
    /// # Errors
    ///
    /// Returns [`VolatilityError::InsufficientReturns`] for fewer than two returns and
    /// [`VolatilityError::NonFinite`] for non-finite inputs.
    pub fn sample_from_returns(returns: &[f64]) -> Result<Self, VolatilityError> {
        if returns.len() < 2 {
            return Err(VolatilityError::InsufficientReturns);
        }

        if returns.iter().any(|value| !value.is_finite()) {
            return Err(VolatilityError::NonFinite);
        }

        let mean = returns.iter().sum::<f64>() / returns.len() as f64;
        let sum_squared_deviation = returns
            .iter()
            .map(|value| {
                let deviation = value - mean;
                deviation * deviation
            })
            .sum::<f64>();
        let variance = sum_squared_deviation / (returns.len() - 1) as f64;

        Self::new(variance.sqrt())
    }

    /// Returns the volatility value.
    #[must_use]
    pub const fn value(self) -> f64 {
        self.value
    }
}

impl fmt::Display for Volatility {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.value.fmt(formatter)
    }
}

/// Descriptive volatility kind vocabulary.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum VolatilityKind {
    /// Historical volatility.
    Historical,
    /// Realized volatility.
    Realized,
    /// Implied volatility.
    Implied,
    /// Forecast volatility.
    Forecast,
    /// Unknown volatility kind.
    Unknown,
    /// Caller-defined volatility kind.
    Custom(String),
}

impl fmt::Display for VolatilityKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::Historical => "historical",
            Self::Realized => "realized",
            Self::Implied => "implied",
            Self::Forecast => "forecast",
            Self::Unknown => "unknown",
            Self::Custom(value) => value.as_str(),
        })
    }
}

impl FromStr for VolatilityKind {
    type Err = VolatilityKindParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let trimmed = value.trim();
        if trimmed.is_empty() {
            return Err(VolatilityKindParseError::Empty);
        }

        match normalized_token(trimmed).as_str() {
            "historical" => Ok(Self::Historical),
            "realized" => Ok(Self::Realized),
            "implied" => Ok(Self::Implied),
            "forecast" => Ok(Self::Forecast),
            "unknown" => Ok(Self::Unknown),
            _ => Ok(Self::Custom(trimmed.to_string())),
        }
    }
}

/// Errors returned while parsing volatility kinds.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VolatilityKindParseError {
    /// The input was empty after trimming whitespace.
    Empty,
}

impl fmt::Display for VolatilityKindParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("volatility kind cannot be empty"),
        }
    }
}

impl Error for VolatilityKindParseError {}

/// A simple observation-count volatility window.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct VolatilityWindow {
    length: usize,
}

impl VolatilityWindow {
    /// Creates a non-zero volatility window length.
    ///
    /// # Errors
    ///
    /// Returns [`VolatilityError::ZeroWindow`] when `length` is zero.
    pub const fn new(length: usize) -> Result<Self, VolatilityError> {
        if length == 0 {
            Err(VolatilityError::ZeroWindow)
        } else {
            Ok(Self { length })
        }
    }

    /// Returns the window length.
    #[must_use]
    pub const fn length(self) -> usize {
        self.length
    }
}

/// Errors returned by volatility helpers.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VolatilityError {
    /// Volatility and return inputs must be finite.
    NonFinite,
    /// Volatility must not be negative.
    Negative,
    /// Sample volatility requires at least two return observations.
    InsufficientReturns,
    /// Window lengths must be non-zero.
    ZeroWindow,
}

impl fmt::Display for VolatilityError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NonFinite => formatter.write_str("volatility values must be finite"),
            Self::Negative => formatter.write_str("volatility cannot be negative"),
            Self::InsufficientReturns => {
                formatter.write_str("sample volatility requires at least two returns")
            }
            Self::ZeroWindow => formatter.write_str("volatility window length must be non-zero"),
        }
    }
}

impl Error for VolatilityError {}

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
    use super::{Volatility, VolatilityError, VolatilityKind};

    #[test]
    fn accepts_valid_volatility() {
        let volatility = Volatility::new(0.20).expect("volatility should be valid");

        assert_eq!(volatility.value(), 0.20);
    }

    #[test]
    fn rejects_negative_volatility() {
        assert_eq!(Volatility::new(-0.01), Err(VolatilityError::Negative));
    }

    #[test]
    fn displays_and_parses_volatility_kind() {
        let kind: VolatilityKind = "realized".parse().expect("kind should parse");

        assert_eq!(kind, VolatilityKind::Realized);
        assert_eq!(kind.to_string(), "realized");
    }

    #[test]
    fn supports_custom_volatility_kind() {
        let kind: VolatilityKind = "intraday".parse().expect("kind should parse");

        assert_eq!(kind, VolatilityKind::Custom("intraday".to_string()));
    }

    #[test]
    fn computes_sample_volatility() {
        let volatility = Volatility::sample_from_returns(&[0.01, -0.02, 0.015])
            .expect("volatility should compute");

        assert!((volatility.value() - 0.018_929_694_486).abs() < 1.0e-12);
    }
}
