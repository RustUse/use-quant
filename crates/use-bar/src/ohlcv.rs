use use_market_price::MarketPrice;

use crate::{BarError, BarInterval, BarTime, BarVolume, OhlcBar};

/// A primitive OHLCV bar.
#[derive(Clone, Debug, PartialEq)]
pub struct OhlcvBar {
    bar: OhlcBar,
    volume: BarVolume,
}

impl OhlcvBar {
    /// Creates an OHLCV bar from a validated OHLC bar and volume.
    #[must_use]
    pub const fn new(bar: OhlcBar, volume: BarVolume) -> Self {
        Self { bar, volume }
    }

    /// Creates an OHLCV bar from raw `f64` price and volume values.
    ///
    /// # Errors
    ///
    /// Returns [`BarError`] when any price, volume, or OHLC relationship is invalid.
    pub fn from_values(
        time: BarTime,
        interval: BarInterval,
        open: f64,
        high: f64,
        low: f64,
        close: f64,
        volume: f64,
    ) -> Result<Self, BarError> {
        Ok(Self {
            bar: OhlcBar::from_values(time, interval, open, high, low, close)?,
            volume: BarVolume::new(volume)?,
        })
    }

    /// Returns the wrapped OHLC bar.
    #[must_use]
    pub const fn bar(&self) -> &OhlcBar {
        &self.bar
    }

    /// Returns the bar volume.
    #[must_use]
    pub const fn volume(&self) -> BarVolume {
        self.volume
    }

    /// Returns the raw volume value.
    #[must_use]
    pub const fn volume_value(&self) -> f64 {
        self.volume.value()
    }

    /// Returns the bar time label.
    #[must_use]
    pub const fn time(&self) -> &BarTime {
        self.bar.time()
    }

    /// Returns the bar interval.
    #[must_use]
    pub const fn interval(&self) -> &BarInterval {
        self.bar.interval()
    }

    /// Returns the open price.
    #[must_use]
    pub const fn open(&self) -> MarketPrice {
        self.bar.open()
    }

    /// Returns the high price.
    #[must_use]
    pub const fn high(&self) -> MarketPrice {
        self.bar.high()
    }

    /// Returns the low price.
    #[must_use]
    pub const fn low(&self) -> MarketPrice {
        self.bar.low()
    }

    /// Returns the close price.
    #[must_use]
    pub const fn close(&self) -> MarketPrice {
        self.bar.close()
    }

    /// Returns the high-low price range.
    #[must_use]
    pub fn range(&self) -> f64 {
        self.bar.range()
    }

    /// Returns the absolute open-close body size.
    #[must_use]
    pub fn body(&self) -> f64 {
        self.bar.body()
    }

    /// Returns `true` when close is greater than open.
    #[must_use]
    pub fn is_up(&self) -> bool {
        self.bar.is_up()
    }

    /// Returns `true` when close is less than open.
    #[must_use]
    pub fn is_down(&self) -> bool {
        self.bar.is_down()
    }

    /// Returns `true` when close is equal to open.
    #[must_use]
    pub fn is_flat(&self) -> bool {
        self.bar.is_flat()
    }

    /// Returns the midpoint of the high and low prices.
    ///
    /// # Errors
    ///
    /// Returns [`BarError`] if the derived price is invalid.
    pub fn midpoint(&self) -> Result<MarketPrice, BarError> {
        self.bar.midpoint()
    }

    /// Returns the typical price: `(high + low + close) / 3`.
    ///
    /// # Errors
    ///
    /// Returns [`BarError`] if the derived price is invalid.
    pub fn typical_price(&self) -> Result<MarketPrice, BarError> {
        self.bar.typical_price()
    }
}

#[cfg(test)]
mod tests {
    use super::OhlcvBar;
    use crate::{BarError, BarInterval, BarTime, BarVolume, BarVolumeError, OhlcBar};

    #[test]
    fn constructs_valid_ohlcv_bar_from_values() {
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

        assert!((bar.open().value() - 100.0).abs() < f64::EPSILON);
        assert!((bar.high().value() - 102.0).abs() < f64::EPSILON);
        assert!((bar.low().value() - 99.5).abs() < f64::EPSILON);
        assert!((bar.close().value() - 101.25).abs() < f64::EPSILON);
        assert!((bar.volume().value() - 42_000.0).abs() < f64::EPSILON);
        assert!((bar.volume_value() - 42_000.0).abs() < f64::EPSILON);
    }

    #[test]
    fn constructs_valid_ohlcv_bar_from_valid_parts() {
        let ohlc = OhlcBar::from_values(
            BarTime::new("2026-05-17").expect("time should be valid"),
            BarInterval::Day,
            100.0,
            102.0,
            99.5,
            101.25,
        )
        .expect("ohlc bar should be valid");

        let volume = BarVolume::new(42_000.0).expect("volume should be valid");
        let bar = OhlcvBar::new(ohlc, volume);

        assert!((bar.close().value() - 101.25).abs() < f64::EPSILON);
        assert!((bar.volume().value() - 42_000.0).abs() < f64::EPSILON);
    }

    #[test]
    fn allows_zero_volume() {
        let bar = OhlcvBar::from_values(
            BarTime::new("2026-05-17").expect("time should be valid"),
            BarInterval::Day,
            100.0,
            102.0,
            99.5,
            101.25,
            0.0,
        )
        .expect("zero volume should be valid");

        assert!(bar.volume().value().abs() < f64::EPSILON);
    }

    #[test]
    fn rejects_negative_volume() {
        assert_eq!(
            OhlcvBar::from_values(
                BarTime::new("t").expect("time should be valid"),
                BarInterval::Day,
                100.0,
                102.0,
                99.5,
                101.25,
                -1.0,
            ),
            Err(BarError::InvalidVolume(BarVolumeError::Invalid))
        );
    }

    #[test]
    fn rejects_non_finite_volume() {
        assert_eq!(
            OhlcvBar::from_values(
                BarTime::new("t").expect("time should be valid"),
                BarInterval::Day,
                100.0,
                102.0,
                99.5,
                101.25,
                f64::NAN,
            ),
            Err(BarError::InvalidVolume(BarVolumeError::Invalid))
        );

        assert_eq!(
            OhlcvBar::from_values(
                BarTime::new("t").expect("time should be valid"),
                BarInterval::Day,
                100.0,
                102.0,
                99.5,
                101.25,
                f64::INFINITY,
            ),
            Err(BarError::InvalidVolume(BarVolumeError::Invalid))
        );
    }

    #[test]
    fn rejects_invalid_ohlc_relationships() {
        assert_eq!(
            OhlcvBar::from_values(
                BarTime::new("t").expect("time should be valid"),
                BarInterval::Day,
                100.0,
                99.0,
                98.0,
                100.0,
                42_000.0,
            ),
            Err(BarError::InvalidHigh)
        );

        assert_eq!(
            OhlcvBar::from_values(
                BarTime::new("t").expect("time should be valid"),
                BarInterval::Day,
                100.0,
                102.0,
                100.5,
                101.0,
                42_000.0,
            ),
            Err(BarError::InvalidLow)
        );
    }

    #[test]
    fn forwards_ohlc_helpers() {
        let bar = OhlcvBar::from_values(
            BarTime::new("t").expect("time should be valid"),
            BarInterval::Day,
            100.0,
            105.0,
            98.0,
            103.0,
            42_000.0,
        )
        .expect("bar should be valid");

        assert!((bar.range() - 7.0).abs() < f64::EPSILON);
        assert!((bar.body() - 3.0).abs() < f64::EPSILON);
        assert!(bar.is_up());
        assert!(!bar.is_down());
        assert!(!bar.is_flat());
    }

    #[test]
    fn calculates_midpoint_and_typical_price() {
        let bar = OhlcvBar::from_values(
            BarTime::new("t").expect("time should be valid"),
            BarInterval::Day,
            100.0,
            106.0,
            98.0,
            102.0,
            42_000.0,
        )
        .expect("bar should be valid");

        let midpoint = bar.midpoint().expect("midpoint should be valid");
        let typical_price = bar.typical_price().expect("typical price should be valid");

        assert!((midpoint.value() - 102.0).abs() < f64::EPSILON);
        assert!((typical_price.value() - 102.0).abs() < f64::EPSILON);
    }
}
