#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::{fmt, str::FromStr};
use std::{error::Error, slice};

use use_market_price::MarketPrice;
use use_return::{ReturnError, SimpleReturn};

/// Common price-series primitives.
pub mod prelude {
    pub use crate::{PricePoint, PriceSeries, PriceSeriesError, SeriesName, SeriesNameError};
}

/// A non-empty price series name.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct SeriesName(String);

impl SeriesName {
    /// Creates a series name from non-empty text.
    ///
    /// # Errors
    ///
    /// Returns [`SeriesNameError::Empty`] when the trimmed name is empty.
    pub fn new(value: impl AsRef<str>) -> Result<Self, SeriesNameError> {
        let trimmed = value.as_ref().trim();
        if trimmed.is_empty() {
            Err(SeriesNameError::Empty)
        } else {
            Ok(Self(trimmed.to_string()))
        }
    }

    /// Returns the series name.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for SeriesName {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for SeriesName {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for SeriesName {
    type Err = SeriesNameError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

/// Errors returned while constructing series names.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SeriesNameError {
    /// The name was empty after trimming whitespace.
    Empty,
}

impl fmt::Display for SeriesNameError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("series name cannot be empty"),
        }
    }
}

impl Error for SeriesNameError {}

/// A price point with an optional label.
#[derive(Clone, Debug, PartialEq)]
pub struct PricePoint {
    label: Option<String>,
    price: MarketPrice,
}

impl PricePoint {
    /// Creates an unlabeled price point.
    #[must_use]
    pub const fn new(price: MarketPrice) -> Self {
        Self { label: None, price }
    }

    /// Attaches a non-empty point label.
    ///
    /// # Errors
    ///
    /// Returns [`PriceSeriesError::EmptyLabel`] when the trimmed label is empty.
    pub fn with_label(mut self, label: impl AsRef<str>) -> Result<Self, PriceSeriesError> {
        let trimmed = label.as_ref().trim();
        if trimmed.is_empty() {
            return Err(PriceSeriesError::EmptyLabel);
        }

        self.label = Some(trimmed.to_string());
        Ok(self)
    }

    /// Returns the optional point label.
    #[must_use]
    pub fn label(&self) -> Option<&str> {
        self.label.as_deref()
    }

    /// Returns the price value.
    #[must_use]
    pub const fn price(&self) -> MarketPrice {
        self.price
    }
}

/// A lightweight insertion-ordered price series.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PriceSeries {
    name: Option<SeriesName>,
    points: Vec<PricePoint>,
}

impl PriceSeries {
    /// Creates an empty unnamed price series.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            name: None,
            points: Vec::new(),
        }
    }

    /// Creates an empty named price series.
    #[must_use]
    pub const fn named(name: SeriesName) -> Self {
        Self {
            name: Some(name),
            points: Vec::new(),
        }
    }

    /// Creates a series from insertion-ordered points.
    #[must_use]
    pub fn from_points(points: impl IntoIterator<Item = PricePoint>) -> Self {
        Self {
            name: None,
            points: points.into_iter().collect(),
        }
    }

    /// Returns the optional series name.
    #[must_use]
    pub const fn name(&self) -> Option<&SeriesName> {
        self.name.as_ref()
    }

    /// Appends a point, preserving insertion order.
    pub fn push(&mut self, point: PricePoint) {
        self.points.push(point);
    }

    /// Returns the number of points.
    #[must_use]
    pub fn len(&self) -> usize {
        self.points.len()
    }

    /// Returns whether the series contains no points.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.points.is_empty()
    }

    /// Returns the first point.
    #[must_use]
    pub fn first(&self) -> Option<&PricePoint> {
        self.points.first()
    }

    /// Returns the last point.
    #[must_use]
    pub fn last(&self) -> Option<&PricePoint> {
        self.points.last()
    }

    /// Iterates over points in insertion order.
    pub fn iter(&self) -> slice::Iter<'_, PricePoint> {
        self.points.iter()
    }

    /// Computes adjacent simple returns in insertion order.
    ///
    /// # Errors
    ///
    /// Returns [`PriceSeriesError::Return`] when a price pair cannot produce a simple return.
    pub fn adjacent_simple_returns(&self) -> Result<Vec<SimpleReturn>, PriceSeriesError> {
        self.points
            .windows(2)
            .map(|pair| {
                SimpleReturn::from_prices(pair[0].price().value(), pair[1].price().value())
                    .map_err(PriceSeriesError::Return)
            })
            .collect()
    }
}

impl IntoIterator for PriceSeries {
    type Item = PricePoint;
    type IntoIter = std::vec::IntoIter<PricePoint>;

    fn into_iter(self) -> Self::IntoIter {
        self.points.into_iter()
    }
}

impl<'a> IntoIterator for &'a PriceSeries {
    type Item = &'a PricePoint;
    type IntoIter = slice::Iter<'a, PricePoint>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

/// Errors returned by price-series helpers.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PriceSeriesError {
    /// Labels must be non-empty after trimming whitespace.
    EmptyLabel,
    /// A return calculation failed.
    Return(ReturnError),
}

impl fmt::Display for PriceSeriesError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyLabel => formatter.write_str("price point label cannot be empty"),
            Self::Return(error) => write!(formatter, "{error}"),
        }
    }
}

impl Error for PriceSeriesError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::EmptyLabel => None,
            Self::Return(error) => Some(error),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{PricePoint, PriceSeries, SeriesName};
    use use_market_price::MarketPrice;

    fn price(value: f64) -> PricePoint {
        PricePoint::new(MarketPrice::new(value).expect("price should be valid"))
    }

    #[test]
    fn creates_empty_series() {
        let series = PriceSeries::new();

        assert!(series.is_empty());
        assert_eq!(series.len(), 0);
        assert!(series.first().is_none());
    }

    #[test]
    fn appends_price_point() {
        let mut series = PriceSeries::named(SeriesName::new("ABC").expect("name should be valid"));
        series.push(price(100.0));

        assert_eq!(series.name().expect("name should exist").as_str(), "ABC");
        assert_eq!(series.len(), 1);
    }

    #[test]
    fn exposes_first_and_last() {
        let mut series = PriceSeries::new();
        series.push(price(100.0));
        series.push(price(105.0));

        assert_eq!(
            series.first().expect("first should exist").price().value(),
            100.0
        );
        assert_eq!(
            series.last().expect("last should exist").price().value(),
            105.0
        );
    }

    #[test]
    fn preserves_iteration_order() {
        let series = PriceSeries::from_points([price(100.0), price(101.0), price(99.0)]);
        let values: Vec<f64> = series.iter().map(|point| point.price().value()).collect();

        assert_eq!(values, vec![100.0, 101.0, 99.0]);
    }

    #[test]
    fn computes_adjacent_returns() {
        let series = PriceSeries::from_points([price(100.0), price(105.0), price(103.0)]);
        let returns = series
            .adjacent_simple_returns()
            .expect("returns should compute");

        assert_eq!(returns.len(), 2);
        assert!((returns[0].value() - 0.05).abs() < 1.0e-12);
    }
}
