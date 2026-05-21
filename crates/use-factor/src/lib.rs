#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::{fmt, str::FromStr};
use std::error::Error;

/// Common factor primitives.
pub mod prelude {
    pub use crate::{FactorError, FactorExposure, FactorLoading, FactorModelName, FactorName};
}

/// A non-empty factor name.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct FactorName(String);

impl FactorName {
    /// Creates a factor name from non-empty text.
    ///
    /// # Errors
    ///
    /// Returns [`FactorError::EmptyName`] when the trimmed value is empty.
    pub fn new(value: impl AsRef<str>) -> Result<Self, FactorError> {
        non_empty_text(value).map(Self)
    }

    /// Returns the factor name.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for FactorName {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for FactorName {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for FactorName {
    type Err = FactorError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

/// A non-empty factor model name.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct FactorModelName(String);

impl FactorModelName {
    /// Creates a factor model name from non-empty text.
    ///
    /// # Errors
    ///
    /// Returns [`FactorError::EmptyName`] when the trimmed value is empty.
    pub fn new(value: impl AsRef<str>) -> Result<Self, FactorError> {
        non_empty_text(value).map(Self)
    }

    /// Returns the model name.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for FactorModelName {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for FactorModelName {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for FactorModelName {
    type Err = FactorError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

/// A factor exposure value.
#[derive(Clone, Debug, PartialEq)]
pub struct FactorExposure {
    factor: FactorName,
    value: f64,
}

impl FactorExposure {
    /// Creates a factor exposure from a factor and finite value.
    ///
    /// # Errors
    ///
    /// Returns [`FactorError::NonFiniteValue`] when `value` is not finite.
    pub fn new(factor: FactorName, value: f64) -> Result<Self, FactorError> {
        Ok(Self {
            factor,
            value: finite_value(value)?,
        })
    }

    /// Returns the factor name.
    #[must_use]
    pub const fn factor(&self) -> &FactorName {
        &self.factor
    }

    /// Returns the exposure value.
    #[must_use]
    pub const fn value(&self) -> f64 {
        self.value
    }
}

/// A factor loading value.
#[derive(Clone, Debug, PartialEq)]
pub struct FactorLoading {
    factor: FactorName,
    value: f64,
}

impl FactorLoading {
    /// Creates a factor loading from a factor and finite value.
    ///
    /// # Errors
    ///
    /// Returns [`FactorError::NonFiniteValue`] when `value` is not finite.
    pub fn new(factor: FactorName, value: f64) -> Result<Self, FactorError> {
        Ok(Self {
            factor,
            value: finite_value(value)?,
        })
    }

    /// Returns the factor name.
    #[must_use]
    pub const fn factor(&self) -> &FactorName {
        &self.factor
    }

    /// Returns the loading value.
    #[must_use]
    pub const fn value(&self) -> f64 {
        self.value
    }
}

/// Errors returned by factor helpers.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FactorError {
    /// Names must be non-empty after trimming whitespace.
    EmptyName,
    /// Exposure and loading values must be finite.
    NonFiniteValue,
}

impl fmt::Display for FactorError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyName => formatter.write_str("factor name cannot be empty"),
            Self::NonFiniteValue => formatter.write_str("factor value must be finite"),
        }
    }
}

impl Error for FactorError {}

fn non_empty_text(value: impl AsRef<str>) -> Result<String, FactorError> {
    let trimmed = value.as_ref().trim();
    if trimmed.is_empty() {
        Err(FactorError::EmptyName)
    } else {
        Ok(trimmed.to_string())
    }
}

const fn finite_value(value: f64) -> Result<f64, FactorError> {
    if value.is_finite() {
        Ok(value)
    } else {
        Err(FactorError::NonFiniteValue)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use super::{FactorError, FactorExposure, FactorLoading, FactorName};

    #[test]
    fn accepts_valid_factor_name() {
        let name = FactorName::new("momentum").expect("name should be valid");

        assert_eq!(name.as_str(), "momentum");
    }

    #[test]
    fn rejects_empty_factor_name() {
        assert_eq!(FactorName::new(" \t"), Err(FactorError::EmptyName));
    }

    #[test]
    fn constructs_exposure() {
        let exposure = FactorExposure::new(
            FactorName::new("quality").expect("name should be valid"),
            0.7,
        )
        .expect("exposure should be valid");

        assert!((exposure.value() - 0.7).abs() < f64::EPSILON);
    }

    #[test]
    fn constructs_loading() {
        let loading = FactorLoading::new(
            FactorName::new("market").expect("name should be valid"),
            1.2,
        )
        .expect("loading should be valid");

        assert!((loading.value() - 1.2).abs() < f64::EPSILON);
    }

    #[test]
    fn factor_names_sort_deterministically() {
        let mut exposures = BTreeMap::new();
        exposures.insert(FactorName::new("value").expect("name should be valid"), 0.1);
        exposures.insert(
            FactorName::new("market").expect("name should be valid"),
            0.2,
        );

        let names: Vec<&str> = exposures.keys().map(FactorName::as_str).collect();
        assert_eq!(names, vec!["market", "value"]);
    }
}
