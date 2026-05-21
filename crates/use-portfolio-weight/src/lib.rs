#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::fmt;
use std::{collections::BTreeMap, collections::btree_map, error::Error};

/// Common portfolio weight primitives.
pub mod prelude {
    pub use crate::{AssetWeight, PortfolioWeight, PortfolioWeightError, WeightSet};
}

/// A finite portfolio weight value.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct PortfolioWeight {
    value: f64,
}

impl PortfolioWeight {
    /// Creates a portfolio weight from a finite `f64`.
    ///
    /// Negative values are accepted for short-exposure vocabulary.
    ///
    /// # Errors
    ///
    /// Returns [`PortfolioWeightError::NonFiniteWeight`] when `value` is not finite.
    pub const fn new(value: f64) -> Result<Self, PortfolioWeightError> {
        if value.is_finite() {
            Ok(Self { value })
        } else {
            Err(PortfolioWeightError::NonFiniteWeight)
        }
    }

    /// Returns the weight value.
    #[must_use]
    pub const fn value(self) -> f64 {
        self.value
    }
}

impl fmt::Display for PortfolioWeight {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.value.fmt(formatter)
    }
}

/// A non-empty asset identifier paired with a portfolio weight.
#[derive(Clone, Debug, PartialEq)]
pub struct AssetWeight {
    asset_id: String,
    weight: PortfolioWeight,
}

impl AssetWeight {
    /// Creates an asset weight.
    ///
    /// # Errors
    ///
    /// Returns [`PortfolioWeightError::EmptyAssetId`] when the trimmed identifier is empty.
    pub fn new(
        asset_id: impl AsRef<str>,
        weight: PortfolioWeight,
    ) -> Result<Self, PortfolioWeightError> {
        let trimmed = asset_id.as_ref().trim();
        if trimmed.is_empty() {
            return Err(PortfolioWeightError::EmptyAssetId);
        }

        Ok(Self {
            asset_id: trimmed.to_string(),
            weight,
        })
    }

    /// Returns the asset identifier.
    #[must_use]
    pub fn asset_id(&self) -> &str {
        &self.asset_id
    }

    /// Returns the asset weight.
    #[must_use]
    pub const fn weight(&self) -> PortfolioWeight {
        self.weight
    }
}

/// A deterministic set of asset weights keyed by asset identifier.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct WeightSet {
    weights: BTreeMap<String, PortfolioWeight>,
}

impl WeightSet {
    /// Creates an empty weight set.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            weights: BTreeMap::new(),
        }
    }

    /// Creates a weight set from asset weights.
    ///
    /// # Errors
    ///
    /// Returns [`PortfolioWeightError::DuplicateAssetId`] when an asset appears more than once.
    pub fn from_asset_weights(
        weights: impl IntoIterator<Item = AssetWeight>,
    ) -> Result<Self, PortfolioWeightError> {
        let mut set = Self::new();
        for weight in weights {
            set.insert(weight)?;
        }
        Ok(set)
    }

    /// Inserts an asset weight.
    ///
    /// # Errors
    ///
    /// Returns [`PortfolioWeightError::DuplicateAssetId`] when the asset already exists.
    pub fn insert(&mut self, asset_weight: AssetWeight) -> Result<(), PortfolioWeightError> {
        if self.weights.contains_key(asset_weight.asset_id()) {
            return Err(PortfolioWeightError::DuplicateAssetId(
                asset_weight.asset_id().to_string(),
            ));
        }

        self.weights
            .insert(asset_weight.asset_id, asset_weight.weight);
        Ok(())
    }

    /// Returns the sum of all weights.
    #[must_use]
    pub fn sum(&self) -> f64 {
        self.weights.values().map(|weight| weight.value()).sum()
    }

    /// Checks whether weights sum approximately to `1.0` within `tolerance`.
    ///
    /// # Errors
    ///
    /// Returns [`PortfolioWeightError::NonFiniteTolerance`] or
    /// [`PortfolioWeightError::NegativeTolerance`] when tolerance is invalid.
    pub fn is_approximately_fully_invested(
        &self,
        tolerance: f64,
    ) -> Result<bool, PortfolioWeightError> {
        if !tolerance.is_finite() {
            return Err(PortfolioWeightError::NonFiniteTolerance);
        }

        if tolerance < 0.0 {
            return Err(PortfolioWeightError::NegativeTolerance);
        }

        Ok((self.sum() - 1.0).abs() <= tolerance)
    }

    /// Iterates over weights in deterministic asset-id order.
    pub fn iter(&self) -> btree_map::Iter<'_, String, PortfolioWeight> {
        self.weights.iter()
    }
}

impl<'a> IntoIterator for &'a WeightSet {
    type Item = (&'a String, &'a PortfolioWeight);
    type IntoIter = btree_map::Iter<'a, String, PortfolioWeight>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

/// Errors returned by portfolio weight helpers.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PortfolioWeightError {
    /// Weight values must be finite.
    NonFiniteWeight,
    /// Asset identifiers must be non-empty after trimming whitespace.
    EmptyAssetId,
    /// Asset identifiers must be unique in a weight set.
    DuplicateAssetId(String),
    /// Tolerances must be finite.
    NonFiniteTolerance,
    /// Tolerances must not be negative.
    NegativeTolerance,
}

impl fmt::Display for PortfolioWeightError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NonFiniteWeight => formatter.write_str("portfolio weight must be finite"),
            Self::EmptyAssetId => formatter.write_str("asset identifier cannot be empty"),
            Self::DuplicateAssetId(asset_id) => {
                write!(formatter, "duplicate asset identifier: {asset_id}")
            },
            Self::NonFiniteTolerance => formatter.write_str("weight tolerance must be finite"),
            Self::NegativeTolerance => formatter.write_str("weight tolerance cannot be negative"),
        }
    }
}

impl Error for PortfolioWeightError {}

#[cfg(test)]
mod tests {
    use super::{AssetWeight, PortfolioWeight, WeightSet};

    #[test]
    fn accepts_valid_weight() {
        let weight = PortfolioWeight::new(0.25).expect("weight should be valid");

        assert!((weight.value() - 0.25).abs() < f64::EPSILON);
    }

    #[test]
    fn constructs_weight_set() {
        let weights = WeightSet::from_asset_weights([
            AssetWeight::new(
                "XYZ",
                PortfolioWeight::new(0.40).expect("weight should be valid"),
            )
            .expect("asset should be valid"),
            AssetWeight::new(
                "ABC",
                PortfolioWeight::new(0.60).expect("weight should be valid"),
            )
            .expect("asset should be valid"),
        ])
        .expect("set should be valid");

        let ids: Vec<&str> = (&weights)
            .into_iter()
            .map(|(asset_id, _)| asset_id.as_str())
            .collect();
        assert_eq!(ids, vec!["ABC", "XYZ"]);
    }

    #[test]
    fn sums_weights() {
        let weights = WeightSet::from_asset_weights([
            AssetWeight::new(
                "ABC",
                PortfolioWeight::new(0.60).expect("weight should be valid"),
            )
            .expect("asset should be valid"),
            AssetWeight::new(
                "XYZ",
                PortfolioWeight::new(0.40).expect("weight should be valid"),
            )
            .expect("asset should be valid"),
        ])
        .expect("set should be valid");

        assert!((weights.sum() - 1.0).abs() < f64::EPSILON);
    }

    #[test]
    fn checks_approximate_fully_invested() {
        let weights = WeightSet::from_asset_weights([
            AssetWeight::new(
                "ABC",
                PortfolioWeight::new(0.60).expect("weight should be valid"),
            )
            .expect("asset should be valid"),
            AssetWeight::new(
                "XYZ",
                PortfolioWeight::new(0.400_000_000_1).expect("weight should be valid"),
            )
            .expect("asset should be valid"),
        ])
        .expect("set should be valid");

        assert!(
            weights
                .is_approximately_fully_invested(1.0e-9)
                .expect("check should succeed")
        );
    }

    #[test]
    fn documents_negative_weight_behavior() {
        let weight = PortfolioWeight::new(-0.10).expect("negative weight should be valid");

        assert!((weight.value() + 0.10).abs() < f64::EPSILON);
    }
}
