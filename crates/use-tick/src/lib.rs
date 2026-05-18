#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::{fmt, str::FromStr};
use std::error::Error;

use use_market_price::MarketPrice;

/// Common tick primitives.
pub mod prelude {
    pub use crate::{QuoteTick, Tick, TickError, TickKind, TickKindParseError, TradeTick};
}

/// Descriptive tick kind vocabulary.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum TickKind {
    /// Trade tick.
    Trade,
    /// Bid tick.
    Bid,
    /// Ask tick.
    Ask,
    /// Quote tick.
    Quote,
    /// Unknown tick kind.
    Unknown,
    /// Caller-defined tick kind.
    Custom(String),
}

impl fmt::Display for TickKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::Trade => "trade",
            Self::Bid => "bid",
            Self::Ask => "ask",
            Self::Quote => "quote",
            Self::Unknown => "unknown",
            Self::Custom(value) => value.as_str(),
        })
    }
}

impl FromStr for TickKind {
    type Err = TickKindParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let trimmed = value.trim();
        if trimmed.is_empty() {
            return Err(TickKindParseError::Empty);
        }

        match normalized_token(trimmed).as_str() {
            "trade" => Ok(Self::Trade),
            "bid" => Ok(Self::Bid),
            "ask" => Ok(Self::Ask),
            "quote" => Ok(Self::Quote),
            "unknown" => Ok(Self::Unknown),
            _ => Ok(Self::Custom(trimmed.to_string())),
        }
    }
}

/// Errors returned while parsing tick kinds.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TickKindParseError {
    /// The input was empty after trimming whitespace.
    Empty,
}

impl fmt::Display for TickKindParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("tick kind cannot be empty"),
        }
    }
}

impl Error for TickKindParseError {}

/// A single price tick with optional timestamp label and size.
#[derive(Clone, Debug, PartialEq)]
pub struct Tick {
    kind: TickKind,
    timestamp: Option<String>,
    price: MarketPrice,
    size: Option<f64>,
}

impl Tick {
    /// Creates a tick from a kind and price.
    #[must_use]
    pub const fn new(kind: TickKind, price: MarketPrice) -> Self {
        Self {
            kind,
            timestamp: None,
            price,
            size: None,
        }
    }

    /// Attaches a non-empty timestamp label.
    ///
    /// # Errors
    ///
    /// Returns [`TickError::EmptyTimestamp`] when the trimmed label is empty.
    pub fn with_timestamp(mut self, timestamp: impl AsRef<str>) -> Result<Self, TickError> {
        let trimmed = timestamp.as_ref().trim();
        if trimmed.is_empty() {
            return Err(TickError::EmptyTimestamp);
        }

        self.timestamp = Some(trimmed.to_string());
        Ok(self)
    }

    /// Attaches a finite non-negative size.
    ///
    /// # Errors
    ///
    /// Returns [`TickError::NonFiniteSize`] or [`TickError::NegativeSize`] when `size` is invalid.
    pub fn with_size(mut self, size: f64) -> Result<Self, TickError> {
        validate_size(size)?;
        self.size = Some(size);
        Ok(self)
    }

    /// Returns the tick kind.
    #[must_use]
    pub const fn kind(&self) -> &TickKind {
        &self.kind
    }

    /// Returns the optional timestamp label.
    #[must_use]
    pub fn timestamp(&self) -> Option<&str> {
        self.timestamp.as_deref()
    }

    /// Returns the tick price.
    #[must_use]
    pub const fn price(&self) -> MarketPrice {
        self.price
    }

    /// Returns the optional size.
    #[must_use]
    pub const fn size(&self) -> Option<f64> {
        self.size
    }
}

/// A trade tick wrapper.
#[derive(Clone, Debug, PartialEq)]
pub struct TradeTick {
    tick: Tick,
}

impl TradeTick {
    /// Creates a trade tick.
    #[must_use]
    pub const fn new(price: MarketPrice) -> Self {
        Self {
            tick: Tick::new(TickKind::Trade, price),
        }
    }

    /// Attaches a timestamp label.
    ///
    /// # Errors
    ///
    /// Returns [`TickError::EmptyTimestamp`] when the trimmed label is empty.
    pub fn with_timestamp(mut self, timestamp: impl AsRef<str>) -> Result<Self, TickError> {
        self.tick = self.tick.with_timestamp(timestamp)?;
        Ok(self)
    }

    /// Attaches a finite non-negative size.
    ///
    /// # Errors
    ///
    /// Returns [`TickError::NonFiniteSize`] or [`TickError::NegativeSize`] when `size` is invalid.
    pub fn with_size(mut self, size: f64) -> Result<Self, TickError> {
        self.tick = self.tick.with_size(size)?;
        Ok(self)
    }

    /// Returns the underlying tick.
    #[must_use]
    pub const fn tick(&self) -> &Tick {
        &self.tick
    }
}

/// A quote tick with optional bid and ask prices.
#[derive(Clone, Debug, PartialEq)]
pub struct QuoteTick {
    timestamp: Option<String>,
    bid: Option<MarketPrice>,
    ask: Option<MarketPrice>,
}

impl QuoteTick {
    /// Creates a quote tick and rejects crossed bid/ask values when both sides are present.
    ///
    /// # Errors
    ///
    /// Returns [`TickError::CrossedQuote`] when `ask < bid`.
    pub fn new(bid: Option<MarketPrice>, ask: Option<MarketPrice>) -> Result<Self, TickError> {
        validate_quote(bid, ask)?;

        Ok(Self {
            timestamp: None,
            bid,
            ask,
        })
    }

    /// Attaches a non-empty timestamp label.
    ///
    /// # Errors
    ///
    /// Returns [`TickError::EmptyTimestamp`] when the trimmed label is empty.
    pub fn with_timestamp(mut self, timestamp: impl AsRef<str>) -> Result<Self, TickError> {
        let trimmed = timestamp.as_ref().trim();
        if trimmed.is_empty() {
            return Err(TickError::EmptyTimestamp);
        }

        self.timestamp = Some(trimmed.to_string());
        Ok(self)
    }

    /// Returns the optional timestamp label.
    #[must_use]
    pub fn timestamp(&self) -> Option<&str> {
        self.timestamp.as_deref()
    }

    /// Returns the optional bid price.
    #[must_use]
    pub const fn bid(&self) -> Option<MarketPrice> {
        self.bid
    }

    /// Returns the optional ask price.
    #[must_use]
    pub const fn ask(&self) -> Option<MarketPrice> {
        self.ask
    }

    /// Returns the ask-bid spread when both sides are present.
    #[must_use]
    pub fn spread(&self) -> Option<f64> {
        Some(self.ask?.value() - self.bid?.value())
    }
}

/// Errors returned by tick construction.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TickError {
    /// Timestamp labels must be non-empty after trimming whitespace.
    EmptyTimestamp,
    /// Size values must be finite.
    NonFiniteSize,
    /// Size values must not be negative.
    NegativeSize,
    /// Quote ask must be greater than or equal to bid.
    CrossedQuote,
}

impl fmt::Display for TickError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyTimestamp => formatter.write_str("tick timestamp cannot be empty"),
            Self::NonFiniteSize => formatter.write_str("tick size must be finite"),
            Self::NegativeSize => formatter.write_str("tick size cannot be negative"),
            Self::CrossedQuote => {
                formatter.write_str("quote ask must be greater than or equal to bid")
            }
        }
    }
}

impl Error for TickError {}

fn validate_size(size: f64) -> Result<(), TickError> {
    if !size.is_finite() {
        return Err(TickError::NonFiniteSize);
    }

    if size < 0.0 {
        return Err(TickError::NegativeSize);
    }

    Ok(())
}

fn validate_quote(bid: Option<MarketPrice>, ask: Option<MarketPrice>) -> Result<(), TickError> {
    if let (Some(bid), Some(ask)) = (bid, ask)
        && ask.value() < bid.value()
    {
        return Err(TickError::CrossedQuote);
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
    use super::{QuoteTick, TickError, TickKind, TradeTick};
    use use_market_price::MarketPrice;

    #[test]
    fn constructs_valid_trade_tick() {
        let tick = TradeTick::new(MarketPrice::new(101.25).expect("price should be valid"))
            .with_timestamp("2026-05-17T10:00:00Z")
            .expect("timestamp should be valid")
            .with_size(100.0)
            .expect("size should be valid");

        assert_eq!(tick.tick().price().value(), 101.25);
        assert_eq!(tick.tick().size(), Some(100.0));
    }

    #[test]
    fn constructs_valid_quote_tick() {
        let quote = QuoteTick::new(
            Some(MarketPrice::new(101.20).expect("price should be valid")),
            Some(MarketPrice::new(101.30).expect("price should be valid")),
        )
        .expect("quote should be valid");

        assert!((quote.spread().expect("spread should exist") - 0.10).abs() < 1.0e-12);
    }

    #[test]
    fn rejects_crossed_quote() {
        assert_eq!(
            QuoteTick::new(
                Some(MarketPrice::new(101.30).expect("price should be valid")),
                Some(MarketPrice::new(101.20).expect("price should be valid")),
            ),
            Err(TickError::CrossedQuote)
        );
    }

    #[test]
    fn displays_and_parses_tick_kind() {
        let kind: TickKind = "trade".parse().expect("kind should parse");

        assert_eq!(kind, TickKind::Trade);
        assert_eq!(kind.to_string(), "trade");
    }

    #[test]
    fn supports_custom_tick_kind() {
        let kind: TickKind = "auction".parse().expect("kind should parse");

        assert_eq!(kind, TickKind::Custom("auction".to_string()));
    }
}
