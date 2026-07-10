use use_market_price::MarketPrice;

use crate::{BarError, BarInterval, BarTime};

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

    /// Returns the high-low price range.
    #[must_use]
    pub fn range(&self) -> f64 {
        self.high.value() - self.low.value()
    }

    /// Returns the absolute open-close body size.
    #[must_use]
    pub fn body(&self) -> f64 {
        (self.close.value() - self.open.value()).abs()
    }

    /// Returns `true` when close is greater than open.
    #[must_use]
    pub fn is_up(&self) -> bool {
        self.close.value() > self.open.value()
    }

    /// Returns `true` when close is less than open.
    #[must_use]
    pub fn is_down(&self) -> bool {
        self.close.value() < self.open.value()
    }

    /// Returns `true` when close is equal to open.
    #[must_use]
    pub fn is_flat(&self) -> bool {
        self.close.value().total_cmp(&self.open.value()).is_eq()
    }

    /// Returns the midpoint of the high and low prices.
    ///
    /// # Errors
    ///
    /// Returns [`BarError`] if the derived price is invalid.
    pub fn midpoint(&self) -> Result<MarketPrice, BarError> {
        Ok(MarketPrice::new(f64::midpoint(
            self.high.value(),
            self.low.value(),
        ))?)
    }

    /// Returns the typical price: `(high + low + close) / 3`.
    ///
    /// # Errors
    ///
    /// Returns [`BarError`] if the derived price is invalid.
    pub fn typical_price(&self) -> Result<MarketPrice, BarError> {
        Ok(MarketPrice::new(
            (self.high.value() + self.low.value() + self.close.value()) / 3.0,
        )?)
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

#[cfg(test)]
mod tests {
    use super::OhlcBar;
    use crate::{BarError, BarInterval, BarTime};

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
        assert!((bar.close().value() - 101.25).abs() < f64::EPSILON);
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
    fn calculates_range_and_body() {
        let bar = OhlcBar::from_values(
            BarTime::new("t").expect("time should be valid"),
            BarInterval::Day,
            100.0,
            105.0,
            98.0,
            103.0,
        )
        .expect("bar should be valid");

        assert!((bar.range() - 7.0).abs() < f64::EPSILON);
        assert!((bar.body() - 3.0).abs() < f64::EPSILON);
    }

    #[test]
    fn classifies_up_down_and_flat_bars() {
        let up = OhlcBar::from_values(
            BarTime::new("up").expect("time should be valid"),
            BarInterval::Day,
            100.0,
            105.0,
            99.0,
            103.0,
        )
        .expect("bar should be valid");

        let down = OhlcBar::from_values(
            BarTime::new("down").expect("time should be valid"),
            BarInterval::Day,
            100.0,
            101.0,
            95.0,
            96.0,
        )
        .expect("bar should be valid");

        let flat = OhlcBar::from_values(
            BarTime::new("flat").expect("time should be valid"),
            BarInterval::Day,
            100.0,
            101.0,
            99.0,
            100.0,
        )
        .expect("bar should be valid");

        assert!(up.is_up());
        assert!(!up.is_down());
        assert!(!up.is_flat());

        assert!(down.is_down());
        assert!(!down.is_up());
        assert!(!down.is_flat());

        assert!(flat.is_flat());
        assert!(!flat.is_up());
        assert!(!flat.is_down());
    }

    #[test]
    fn calculates_midpoint_and_typical_price() {
        let bar = OhlcBar::from_values(
            BarTime::new("t").expect("time should be valid"),
            BarInterval::Day,
            100.0,
            106.0,
            98.0,
            102.0,
        )
        .expect("bar should be valid");

        let midpoint = bar.midpoint().expect("midpoint should be valid");
        let typical_price = bar.typical_price().expect("typical price should be valid");

        assert!((midpoint.value() - 102.0).abs() < f64::EPSILON);
        assert!((typical_price.value() - 102.0).abs() < f64::EPSILON);
    }
}
