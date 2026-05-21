#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::{fmt, str::FromStr};
use std::error::Error;

/// Common market price primitives.
pub mod prelude {
    pub use crate::{MarketPrice, MarketPriceError, PriceKind, PriceKindParseError, PriceQuote};
}

/// A finite non-negative market price value.
///
/// Zero is accepted for descriptive use cases such as missing, halted, or placeholder prices.
/// Positive prices can be checked with [`MarketPrice::is_positive`].
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct MarketPrice {
    value: f64,
}

impl MarketPrice {
    /// Creates a market price from a finite non-negative `f64`.
    ///
    /// # Errors
    ///
    /// Returns [`MarketPriceError::NonFinite`] for `NaN` or infinite values and
    /// [`MarketPriceError::Negative`] for negative values.
    pub fn new(value: f64) -> Result<Self, MarketPriceError> {
        if !value.is_finite() {
            return Err(MarketPriceError::NonFinite);
        }

        if value < 0.0 {
            return Err(MarketPriceError::Negative);
        }

        Ok(Self { value })
    }

    /// Returns the stored price value.
    #[must_use]
    pub const fn value(self) -> f64 {
        self.value
    }

    /// Returns whether the price is strictly positive.
    #[must_use]
    pub fn is_positive(self) -> bool {
        self.value > 0.0
    }
}

impl TryFrom<f64> for MarketPrice {
    type Error = MarketPriceError;

    fn try_from(value: f64) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl fmt::Display for MarketPrice {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.value.fmt(formatter)
    }
}

/// Errors returned while constructing market price values.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MarketPriceError {
    /// Price values must be finite.
    NonFinite,
    /// Price values must not be negative.
    Negative,
}

impl fmt::Display for MarketPriceError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NonFinite => formatter.write_str("market price must be finite"),
            Self::Negative => formatter.write_str("market price cannot be negative"),
        }
    }
}

impl Error for MarketPriceError {}

/// Descriptive market price kind vocabulary.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum PriceKind {
    /// Opening price.
    Open,
    /// High price.
    High,
    /// Low price.
    Low,
    /// Closing price.
    Close,
    /// Last traded price.
    Last,
    /// Bid price.
    Bid,
    /// Ask price.
    Ask,
    /// Mid price.
    Mid,
    /// Settlement price.
    Settlement,
    /// Adjusted close price.
    AdjustedClose,
    /// Unknown price kind.
    Unknown,
    /// Caller-defined price kind.
    Custom(String),
}

impl fmt::Display for PriceKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::Open => "open",
            Self::High => "high",
            Self::Low => "low",
            Self::Close => "close",
            Self::Last => "last",
            Self::Bid => "bid",
            Self::Ask => "ask",
            Self::Mid => "mid",
            Self::Settlement => "settlement",
            Self::AdjustedClose => "adjusted-close",
            Self::Unknown => "unknown",
            Self::Custom(value) => value.as_str(),
        })
    }
}

impl FromStr for PriceKind {
    type Err = PriceKindParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let trimmed = value.trim();
        if trimmed.is_empty() {
            return Err(PriceKindParseError::Empty);
        }

        match normalized_token(trimmed).as_str() {
            "open" => Ok(Self::Open),
            "high" => Ok(Self::High),
            "low" => Ok(Self::Low),
            "close" => Ok(Self::Close),
            "last" => Ok(Self::Last),
            "bid" => Ok(Self::Bid),
            "ask" => Ok(Self::Ask),
            "mid" => Ok(Self::Mid),
            "settlement" => Ok(Self::Settlement),
            "adjusted-close" => Ok(Self::AdjustedClose),
            "unknown" => Ok(Self::Unknown),
            _ => Ok(Self::Custom(trimmed.to_string())),
        }
    }
}

/// Errors returned while parsing price kinds.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PriceKindParseError {
    /// The input was empty after trimming whitespace.
    Empty,
}

impl fmt::Display for PriceKindParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("price kind cannot be empty"),
        }
    }
}

impl Error for PriceKindParseError {}

/// A market price paired with descriptive price-kind vocabulary.
#[derive(Clone, Debug, PartialEq)]
pub struct PriceQuote {
    kind: PriceKind,
    price: MarketPrice,
}

impl PriceQuote {
    /// Creates a price quote from a kind and already validated price value.
    #[must_use]
    pub const fn new(kind: PriceKind, price: MarketPrice) -> Self {
        Self { kind, price }
    }

    /// Returns the quote kind.
    #[must_use]
    pub const fn kind(&self) -> &PriceKind {
        &self.kind
    }

    /// Returns the quote price.
    #[must_use]
    pub const fn price(&self) -> MarketPrice {
        self.price
    }
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
    use super::{MarketPrice, MarketPriceError, PriceKind, PriceQuote};

    #[test]
    fn accepts_valid_positive_price() {
        let price = MarketPrice::new(101.25).expect("price should be valid");

        assert!((price.value() - 101.25).abs() < f64::EPSILON);
        assert!(price.is_positive());
        assert_eq!(price.to_string(), "101.25");
    }

    #[test]
    fn rejects_negative_price() {
        assert_eq!(MarketPrice::new(-0.01), Err(MarketPriceError::Negative));
    }

    #[test]
    fn displays_and_parses_price_kind() {
        let kind: PriceKind = "Adjusted Close".parse().expect("kind should parse");

        assert_eq!(kind, PriceKind::AdjustedClose);
        assert_eq!(kind.to_string(), "adjusted-close");
    }

    #[test]
    fn supports_custom_price_kind() {
        let kind: PriceKind = "auction-price".parse().expect("kind should parse");

        assert_eq!(kind, PriceKind::Custom("auction-price".to_string()));
        assert_eq!(kind.to_string(), "auction-price");
    }

    #[test]
    fn constructs_quote() {
        let price = MarketPrice::new(100.0).expect("price should be valid");
        let quote = PriceQuote::new(PriceKind::Close, price);

        assert_eq!(quote.kind(), &PriceKind::Close);
        assert_eq!(quote.price(), price);
    }
}
