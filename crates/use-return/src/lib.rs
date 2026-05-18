#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::{fmt, str::FromStr};
use std::error::Error;

/// Common return primitives.
pub mod prelude {
    pub use crate::{
        LogReturn, ReturnError, ReturnKind, ReturnKindParseError, ReturnValue, SimpleReturn,
    };
}

/// A finite simple return value.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct SimpleReturn {
    value: f64,
}

impl SimpleReturn {
    /// Creates a simple return from a finite value.
    ///
    /// # Errors
    ///
    /// Returns [`ReturnError::NonFiniteReturn`] when `value` is not finite.
    pub fn new(value: f64) -> Result<Self, ReturnError> {
        validate_return(value).map(|value| Self { value })
    }

    /// Computes a simple return as `end_price / start_price - 1.0`.
    ///
    /// # Errors
    ///
    /// Returns [`ReturnError`] when either price is not finite, the start price is not positive,
    /// or the end price is negative.
    pub fn from_prices(start_price: f64, end_price: f64) -> Result<Self, ReturnError> {
        validate_start_price(start_price)?;
        validate_end_price_for_simple_return(end_price)?;

        Self::new((end_price / start_price) - 1.0)
    }

    /// Returns the stored return value.
    #[must_use]
    pub const fn value(self) -> f64 {
        self.value
    }
}

impl fmt::Display for SimpleReturn {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.value.fmt(formatter)
    }
}

/// A finite log return value.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct LogReturn {
    value: f64,
}

impl LogReturn {
    /// Creates a log return from a finite value.
    ///
    /// # Errors
    ///
    /// Returns [`ReturnError::NonFiniteReturn`] when `value` is not finite.
    pub fn new(value: f64) -> Result<Self, ReturnError> {
        validate_return(value).map(|value| Self { value })
    }

    /// Computes a log return as `ln(end_price / start_price)`.
    ///
    /// # Errors
    ///
    /// Returns [`ReturnError`] when either price is not finite, the start price is not positive,
    /// or the end price is not positive.
    pub fn from_prices(start_price: f64, end_price: f64) -> Result<Self, ReturnError> {
        validate_start_price(start_price)?;
        validate_end_price_for_log_return(end_price)?;

        Self::new((end_price / start_price).ln())
    }

    /// Returns the stored return value.
    #[must_use]
    pub const fn value(self) -> f64 {
        self.value
    }
}

impl fmt::Display for LogReturn {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.value.fmt(formatter)
    }
}

/// Descriptive return kind vocabulary.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ReturnKind {
    /// Simple arithmetic return.
    Simple,
    /// Logarithmic return.
    Log,
    /// Gross return vocabulary.
    Gross,
    /// Net return vocabulary.
    Net,
    /// Excess return vocabulary.
    Excess,
    /// Unknown return kind.
    Unknown,
    /// Caller-defined return kind.
    Custom(String),
}

impl fmt::Display for ReturnKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::Simple => "simple",
            Self::Log => "log",
            Self::Gross => "gross",
            Self::Net => "net",
            Self::Excess => "excess",
            Self::Unknown => "unknown",
            Self::Custom(value) => value.as_str(),
        })
    }
}

impl FromStr for ReturnKind {
    type Err = ReturnKindParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let trimmed = value.trim();
        if trimmed.is_empty() {
            return Err(ReturnKindParseError::Empty);
        }

        match normalized_token(trimmed).as_str() {
            "simple" => Ok(Self::Simple),
            "log" => Ok(Self::Log),
            "gross" => Ok(Self::Gross),
            "net" => Ok(Self::Net),
            "excess" => Ok(Self::Excess),
            "unknown" => Ok(Self::Unknown),
            _ => Ok(Self::Custom(trimmed.to_string())),
        }
    }
}

/// Errors returned while parsing return kinds.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ReturnKindParseError {
    /// The input was empty after trimming whitespace.
    Empty,
}

impl fmt::Display for ReturnKindParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("return kind cannot be empty"),
        }
    }
}

impl Error for ReturnKindParseError {}

/// A return value paired with descriptive return-kind vocabulary.
#[derive(Clone, Debug, PartialEq)]
pub struct ReturnValue {
    kind: ReturnKind,
    value: f64,
}

impl ReturnValue {
    /// Creates a return value from a kind and finite numeric value.
    ///
    /// # Errors
    ///
    /// Returns [`ReturnError::NonFiniteReturn`] when `value` is not finite.
    pub fn new(kind: ReturnKind, value: f64) -> Result<Self, ReturnError> {
        Ok(Self {
            kind,
            value: validate_return(value)?,
        })
    }

    /// Returns the return kind.
    #[must_use]
    pub const fn kind(&self) -> &ReturnKind {
        &self.kind
    }

    /// Returns the numeric return value.
    #[must_use]
    pub const fn value(&self) -> f64 {
        self.value
    }
}

/// Errors returned by return value construction and price-to-return helpers.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ReturnError {
    /// Return values must be finite.
    NonFiniteReturn,
    /// Price inputs must be finite.
    NonFinitePrice { name: &'static str },
    /// Start prices must be strictly positive.
    NonPositiveStartPrice,
    /// End prices must not be negative for simple returns.
    NegativeEndPrice,
    /// End prices must be strictly positive for log returns.
    NonPositiveEndPrice,
}

impl fmt::Display for ReturnError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NonFiniteReturn => formatter.write_str("return value must be finite"),
            Self::NonFinitePrice { name } => write!(formatter, "{name} price must be finite"),
            Self::NonPositiveStartPrice => formatter.write_str("start price must be positive"),
            Self::NegativeEndPrice => {
                formatter.write_str("end price cannot be negative for simple return")
            }
            Self::NonPositiveEndPrice => {
                formatter.write_str("end price must be positive for log return")
            }
        }
    }
}

impl Error for ReturnError {}

fn validate_return(value: f64) -> Result<f64, ReturnError> {
    if value.is_finite() {
        Ok(value)
    } else {
        Err(ReturnError::NonFiniteReturn)
    }
}

fn validate_start_price(value: f64) -> Result<(), ReturnError> {
    if !value.is_finite() {
        return Err(ReturnError::NonFinitePrice { name: "start" });
    }

    if value <= 0.0 {
        return Err(ReturnError::NonPositiveStartPrice);
    }

    Ok(())
}

fn validate_end_price_for_simple_return(value: f64) -> Result<(), ReturnError> {
    if !value.is_finite() {
        return Err(ReturnError::NonFinitePrice { name: "end" });
    }

    if value < 0.0 {
        return Err(ReturnError::NegativeEndPrice);
    }

    Ok(())
}

fn validate_end_price_for_log_return(value: f64) -> Result<(), ReturnError> {
    if !value.is_finite() {
        return Err(ReturnError::NonFinitePrice { name: "end" });
    }

    if value <= 0.0 {
        return Err(ReturnError::NonPositiveEndPrice);
    }

    Ok(())
}

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
    use super::{LogReturn, ReturnError, ReturnKind, SimpleReturn};

    fn assert_close(left: f64, right: f64) {
        assert!((left - right).abs() < 1.0e-12, "left={left}, right={right}");
    }

    #[test]
    fn computes_simple_return() {
        let value = SimpleReturn::from_prices(100.0, 105.0).expect("return should compute");

        assert_close(value.value(), 0.05);
    }

    #[test]
    fn computes_log_return() {
        let value = LogReturn::from_prices(100.0, 105.0).expect("return should compute");

        assert_close(value.value(), 1.05_f64.ln());
    }

    #[test]
    fn rejects_zero_start_price() {
        assert_eq!(
            SimpleReturn::from_prices(0.0, 105.0),
            Err(ReturnError::NonPositiveStartPrice)
        );
    }

    #[test]
    fn displays_and_parses_return_kind() {
        let kind: ReturnKind = "gross".parse().expect("kind should parse");

        assert_eq!(kind, ReturnKind::Gross);
        assert_eq!(kind.to_string(), "gross");
    }

    #[test]
    fn supports_custom_return_kind() {
        let kind: ReturnKind = "after-fee".parse().expect("kind should parse");

        assert_eq!(kind, ReturnKind::Custom("after-fee".to_string()));
    }
}
