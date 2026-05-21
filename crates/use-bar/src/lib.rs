#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::{fmt, str::FromStr};
use std::error::Error;

use use_market_price::{MarketPrice, MarketPriceError};

/// Common bar primitives.
pub mod prelude {
    pub use crate::{
        BarError, BarInterval, BarIntervalParseError, BarTime, BarTimeError, OhlcBar, OhlcvBar,
    };
}

/// A simple bar time label.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct BarTime(String);

impl BarTime {
    /// Creates a bar time label from non-empty text.
    ///
    /// # Errors
    ///
    /// Returns [`BarTimeError::Empty`] when the trimmed label is empty.
    pub fn new(value: impl AsRef<str>) -> Result<Self, BarTimeError> {
        let trimmed = value.as_ref().trim();
        if trimmed.is_empty() {
            Err(BarTimeError::Empty)
        } else {
            Ok(Self(trimmed.to_string()))
        }
    }

    /// Returns the time label.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for BarTime {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for BarTime {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for BarTime {
    type Err = BarTimeError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

/// Errors returned while constructing bar time labels.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BarTimeError {
    /// The label was empty after trimming whitespace.
    Empty,
}

impl fmt::Display for BarTimeError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("bar time cannot be empty"),
        }
    }
}

impl Error for BarTimeError {}

/// Descriptive bar interval vocabulary.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum BarInterval {
    /// Tick-level interval.
    Tick,
    /// Second interval.
    Second,
    /// Minute interval.
    Minute,
    /// Hour interval.
    Hour,
    /// Day interval.
    Day,
    /// Week interval.
    Week,
    /// Month interval.
    Month,
    /// Caller-defined interval.
    Custom(String),
}

impl fmt::Display for BarInterval {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::Tick => "tick",
            Self::Second => "second",
            Self::Minute => "minute",
            Self::Hour => "hour",
            Self::Day => "day",
            Self::Week => "week",
            Self::Month => "month",
            Self::Custom(value) => value.as_str(),
        })
    }
}

impl FromStr for BarInterval {
    type Err = BarIntervalParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let trimmed = value.trim();
        if trimmed.is_empty() {
            return Err(BarIntervalParseError::Empty);
        }

        match normalized_token(trimmed).as_str() {
            "tick" => Ok(Self::Tick),
            "second" => Ok(Self::Second),
            "minute" => Ok(Self::Minute),
            "hour" => Ok(Self::Hour),
            "day" => Ok(Self::Day),
            "week" => Ok(Self::Week),
            "month" => Ok(Self::Month),
            _ => Ok(Self::Custom(trimmed.to_string())),
        }
    }
}

/// Errors returned while parsing bar intervals.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BarIntervalParseError {
    /// The input was empty after trimming whitespace.
    Empty,
}

impl fmt::Display for BarIntervalParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("bar interval cannot be empty"),
        }
    }
}

impl Error for BarIntervalParseError {}

/// A primitive OHLC bar.
#[derive(Clone, Debug, PartialEq)]
pub struct OhlcBar {
    time: BarTime,
    interval: BarInterval,
    open: MarketPrice,
    high: MarketPrice,
    low: MarketPrice,
    close: MarketPrice,
}

impl OhlcBar {
    /// Creates an OHLC bar from validated price values.
    ///
    /// # Errors
    ///
    /// Returns [`BarError::InvalidHigh`] or [`BarError::InvalidLow`] when obvious OHLC
    /// constraints are violated.
    pub fn new(
        time: BarTime,
        interval: BarInterval,
        open: MarketPrice,
        high: MarketPrice,
        low: MarketPrice,
        close: MarketPrice,
    ) -> Result<Self, BarError> {
        validate_ohlc(open, high, low, close)?;

        Ok(Self {
            time,
            interval,
            open,
            high,
            low,
            close,
        })
    }

    /// Creates an OHLC bar from raw `f64` price values.
    ///
    /// # Errors
    ///
    /// Returns [`BarError`] when any price is invalid or OHLC constraints are violated.
    pub fn from_values(
        time: BarTime,
        interval: BarInterval,
        open: f64,
        high: f64,
        low: f64,
        close: f64,
    ) -> Result<Self, BarError> {
        Self::new(
            time,
            interval,
            MarketPrice::new(open)?,
            MarketPrice::new(high)?,
            MarketPrice::new(low)?,
            MarketPrice::new(close)?,
        )
    }

    /// Returns the bar time label.
    #[must_use]
    pub const fn time(&self) -> &BarTime {
        &self.time
    }

    /// Returns the bar interval.
    #[must_use]
    pub const fn interval(&self) -> &BarInterval {
        &self.interval
    }

    /// Returns the open price.
    #[must_use]
    pub const fn open(&self) -> MarketPrice {
        self.open
    }

    /// Returns the high price.
    #[must_use]
    pub const fn high(&self) -> MarketPrice {
        self.high
    }

    /// Returns the low price.
    #[must_use]
    pub const fn low(&self) -> MarketPrice {
        self.low
    }

    /// Returns the close price.
    #[must_use]
    pub const fn close(&self) -> MarketPrice {
        self.close
    }
}

/// A primitive OHLCV bar.
#[derive(Clone, Debug, PartialEq)]
pub struct OhlcvBar {
    bar: OhlcBar,
    volume: f64,
}

impl OhlcvBar {
    /// Creates an OHLCV bar from an OHLC bar and finite non-negative volume.
    ///
    /// # Errors
    ///
    /// Returns [`BarError::NonFiniteVolume`] or [`BarError::NegativeVolume`] when volume is
    /// invalid.
    pub fn new(bar: OhlcBar, volume: f64) -> Result<Self, BarError> {
        validate_volume(volume)?;

        Ok(Self { bar, volume })
    }

    /// Creates an OHLCV bar from raw `f64` price and volume values.
    ///
    /// # Errors
    ///
    /// Returns [`BarError`] when any price, volume, or OHLC relationship is invalid.
    #[allow(clippy::too_many_arguments)]
    pub fn from_values(
        time: BarTime,
        interval: BarInterval,
        open: f64,
        high: f64,
        low: f64,
        close: f64,
        volume: f64,
    ) -> Result<Self, BarError> {
        Self::new(
            OhlcBar::from_values(time, interval, open, high, low, close)?,
            volume,
        )
    }

    /// Returns the OHLC portion of the bar.
    #[must_use]
    pub const fn bar(&self) -> &OhlcBar {
        &self.bar
    }

    /// Returns the volume value.
    #[must_use]
    pub const fn volume(&self) -> f64 {
        self.volume
    }
}

/// Errors returned by bar construction.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BarError {
    /// One of the price values was invalid.
    InvalidPrice(MarketPriceError),
    /// Volume must be finite.
    NonFiniteVolume,
    /// Volume must not be negative.
    NegativeVolume,
    /// The high price was lower than open, low, or close.
    InvalidHigh,
    /// The low price was higher than open, high, or close.
    InvalidLow,
}

impl fmt::Display for BarError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidPrice(error) => write!(formatter, "{error}"),
            Self::NonFiniteVolume => formatter.write_str("bar volume must be finite"),
            Self::NegativeVolume => formatter.write_str("bar volume cannot be negative"),
            Self::InvalidHigh => {
                formatter.write_str("bar high must be at least open, low, and close")
            },
            Self::InvalidLow => {
                formatter.write_str("bar low must be at most open, high, and close")
            },
        }
    }
}

impl Error for BarError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::InvalidPrice(error) => Some(error),
            Self::NonFiniteVolume | Self::NegativeVolume | Self::InvalidHigh | Self::InvalidLow => {
                None
            },
        }
    }
}

impl From<MarketPriceError> for BarError {
    fn from(error: MarketPriceError) -> Self {
        Self::InvalidPrice(error)
    }
}

fn validate_ohlc(
    open: MarketPrice,
    high: MarketPrice,
    low: MarketPrice,
    close: MarketPrice,
) -> Result<(), BarError> {
    if high.value() < open.value() || high.value() < low.value() || high.value() < close.value() {
        return Err(BarError::InvalidHigh);
    }

    if low.value() > open.value() || low.value() > high.value() || low.value() > close.value() {
        return Err(BarError::InvalidLow);
    }

    Ok(())
}

fn validate_volume(volume: f64) -> Result<(), BarError> {
    if !volume.is_finite() {
        return Err(BarError::NonFiniteVolume);
    }

    if volume < 0.0 {
        return Err(BarError::NegativeVolume);
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
    use super::{BarError, BarInterval, BarTime, OhlcBar, OhlcvBar};

    #[test]
    fn constructs_valid_ohlc_bar() {
        let bar = OhlcBar::from_values(
            BarTime::new("2026-05-17").expect("time should be valid"),
            BarInterval::Day,
            100.0,
            102.0,
            99.5,
            101.25,
        )
        .expect("bar should be valid");

        assert!((bar.high().value() - 102.0).abs() < f64::EPSILON);
        assert!((bar.low().value() - 99.5).abs() < f64::EPSILON);
    }

    #[test]
    fn constructs_valid_ohlcv_bar() {
        let bar = OhlcvBar::from_values(
            BarTime::new("2026-05-17").expect("time should be valid"),
            BarInterval::Day,
            100.0,
            102.0,
            99.5,
            101.25,
            42_000.0,
        )
        .expect("bar should be valid");

        assert!((bar.volume() - 42_000.0).abs() < f64::EPSILON);
    }

    #[test]
    fn rejects_invalid_high() {
        assert_eq!(
            OhlcBar::from_values(
                BarTime::new("t").expect("time should be valid"),
                BarInterval::Day,
                100.0,
                99.0,
                98.0,
                100.0,
            ),
            Err(BarError::InvalidHigh)
        );
    }

    #[test]
    fn rejects_invalid_low() {
        assert_eq!(
            OhlcBar::from_values(
                BarTime::new("t").expect("time should be valid"),
                BarInterval::Day,
                100.0,
                102.0,
                100.5,
                101.0,
            ),
            Err(BarError::InvalidLow)
        );
    }

    #[test]
    fn displays_and_parses_interval() {
        let interval: BarInterval = "Minute".parse().expect("interval should parse");

        assert_eq!(interval, BarInterval::Minute);
        assert_eq!(interval.to_string(), "minute");
    }

    #[test]
    fn supports_custom_interval() {
        let interval: BarInterval = "session".parse().expect("interval should parse");

        assert_eq!(interval, BarInterval::Custom("session".to_string()));
    }
}
