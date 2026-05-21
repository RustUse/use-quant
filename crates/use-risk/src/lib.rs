#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::{fmt, str::FromStr};
use std::error::Error;

/// Common risk primitives.
pub mod prelude {
    pub use crate::{
        RiskBudget, RiskError, RiskLevel, RiskLevelParseError, RiskLimit, RiskMeasure,
        RiskMeasureParseError,
    };
}

/// Descriptive risk measure vocabulary.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum RiskMeasure {
    /// Volatility measure.
    Volatility,
    /// Value-at-risk measure value.
    ValueAtRisk,
    /// Expected shortfall measure value.
    ExpectedShortfall,
    /// Beta measure.
    Beta,
    /// Tracking error measure.
    TrackingError,
    /// Drawdown measure.
    Drawdown,
    /// Exposure measure.
    Exposure,
    /// Leverage measure.
    Leverage,
    /// Unknown measure.
    Unknown,
    /// Caller-defined risk measure.
    Custom(String),
}

impl fmt::Display for RiskMeasure {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::Volatility => "volatility",
            Self::ValueAtRisk => "value-at-risk",
            Self::ExpectedShortfall => "expected-shortfall",
            Self::Beta => "beta",
            Self::TrackingError => "tracking-error",
            Self::Drawdown => "drawdown",
            Self::Exposure => "exposure",
            Self::Leverage => "leverage",
            Self::Unknown => "unknown",
            Self::Custom(value) => value.as_str(),
        })
    }
}

impl FromStr for RiskMeasure {
    type Err = RiskMeasureParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let trimmed = value.trim();
        if trimmed.is_empty() {
            return Err(RiskMeasureParseError::Empty);
        }

        match normalized_token(trimmed).as_str() {
            "volatility" => Ok(Self::Volatility),
            "value-at-risk" | "var" => Ok(Self::ValueAtRisk),
            "expected-shortfall" | "es" => Ok(Self::ExpectedShortfall),
            "beta" => Ok(Self::Beta),
            "tracking-error" => Ok(Self::TrackingError),
            "drawdown" => Ok(Self::Drawdown),
            "exposure" => Ok(Self::Exposure),
            "leverage" => Ok(Self::Leverage),
            "unknown" => Ok(Self::Unknown),
            _ => Ok(Self::Custom(trimmed.to_string())),
        }
    }
}

/// Errors returned while parsing risk measures.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RiskMeasureParseError {
    /// The input was empty after trimming whitespace.
    Empty,
}

impl fmt::Display for RiskMeasureParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("risk measure cannot be empty"),
        }
    }
}

impl Error for RiskMeasureParseError {}

/// Descriptive risk level vocabulary.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum RiskLevel {
    /// Low risk level.
    Low,
    /// Medium risk level.
    Medium,
    /// High risk level.
    High,
    /// Critical risk level.
    Critical,
    /// Unknown risk level.
    Unknown,
    /// Caller-defined risk level.
    Custom(String),
}

impl fmt::Display for RiskLevel {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::Low => "low",
            Self::Medium => "medium",
            Self::High => "high",
            Self::Critical => "critical",
            Self::Unknown => "unknown",
            Self::Custom(value) => value.as_str(),
        })
    }
}

impl FromStr for RiskLevel {
    type Err = RiskLevelParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let trimmed = value.trim();
        if trimmed.is_empty() {
            return Err(RiskLevelParseError::Empty);
        }

        match normalized_token(trimmed).as_str() {
            "low" => Ok(Self::Low),
            "medium" => Ok(Self::Medium),
            "high" => Ok(Self::High),
            "critical" => Ok(Self::Critical),
            "unknown" => Ok(Self::Unknown),
            _ => Ok(Self::Custom(trimmed.to_string())),
        }
    }
}

/// Errors returned while parsing risk levels.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RiskLevelParseError {
    /// The input was empty after trimming whitespace.
    Empty,
}

impl fmt::Display for RiskLevelParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("risk level cannot be empty"),
        }
    }
}

impl Error for RiskLevelParseError {}

/// A simple numeric risk limit threshold.
#[derive(Clone, Debug, PartialEq)]
pub struct RiskLimit {
    measure: RiskMeasure,
    threshold: f64,
    level: RiskLevel,
}

impl RiskLimit {
    /// Creates a risk limit with unknown level.
    ///
    /// # Errors
    ///
    /// Returns [`RiskError`] when `threshold` is not finite or is negative.
    pub fn new(measure: RiskMeasure, threshold: f64) -> Result<Self, RiskError> {
        Ok(Self {
            measure,
            threshold: validate_non_negative(
                threshold,
                RiskError::NonFiniteThreshold,
                RiskError::NegativeThreshold,
            )?,
            level: RiskLevel::Unknown,
        })
    }

    /// Sets descriptive risk level vocabulary.
    #[must_use]
    pub fn with_level(mut self, level: RiskLevel) -> Self {
        self.level = level;
        self
    }

    /// Returns the risk measure.
    #[must_use]
    pub const fn measure(&self) -> &RiskMeasure {
        &self.measure
    }

    /// Returns the numeric threshold.
    #[must_use]
    pub const fn threshold(&self) -> f64 {
        self.threshold
    }

    /// Returns the risk level.
    #[must_use]
    pub const fn level(&self) -> &RiskLevel {
        &self.level
    }
}

/// A simple numeric risk budget value.
#[derive(Clone, Debug, PartialEq)]
pub struct RiskBudget {
    measure: RiskMeasure,
    amount: f64,
}

impl RiskBudget {
    /// Creates a risk budget.
    ///
    /// # Errors
    ///
    /// Returns [`RiskError`] when `amount` is not finite or is negative.
    pub fn new(measure: RiskMeasure, amount: f64) -> Result<Self, RiskError> {
        Ok(Self {
            measure,
            amount: validate_non_negative(
                amount,
                RiskError::NonFiniteAmount,
                RiskError::NegativeAmount,
            )?,
        })
    }

    /// Returns the risk measure.
    #[must_use]
    pub const fn measure(&self) -> &RiskMeasure {
        &self.measure
    }

    /// Returns the budget amount.
    #[must_use]
    pub const fn amount(&self) -> f64 {
        self.amount
    }
}

/// Errors returned by risk wrappers.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RiskError {
    /// Risk limit thresholds must be finite.
    NonFiniteThreshold,
    /// Risk limit thresholds must not be negative.
    NegativeThreshold,
    /// Risk budget amounts must be finite.
    NonFiniteAmount,
    /// Risk budget amounts must not be negative.
    NegativeAmount,
}

impl fmt::Display for RiskError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NonFiniteThreshold => formatter.write_str("risk limit threshold must be finite"),
            Self::NegativeThreshold => {
                formatter.write_str("risk limit threshold cannot be negative")
            },
            Self::NonFiniteAmount => formatter.write_str("risk budget amount must be finite"),
            Self::NegativeAmount => formatter.write_str("risk budget amount cannot be negative"),
        }
    }
}

impl Error for RiskError {}

fn validate_non_negative(
    value: f64,
    non_finite: RiskError,
    negative: RiskError,
) -> Result<f64, RiskError> {
    if !value.is_finite() {
        return Err(non_finite);
    }

    if value < 0.0 {
        return Err(negative);
    }

    Ok(value)
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
    use super::{RiskBudget, RiskLevel, RiskLimit, RiskMeasure};

    #[test]
    fn displays_and_parses_risk_measure() {
        let measure: RiskMeasure = "value at risk".parse().expect("measure should parse");

        assert_eq!(measure, RiskMeasure::ValueAtRisk);
        assert_eq!(measure.to_string(), "value-at-risk");
    }

    #[test]
    fn supports_custom_risk_measure() {
        let measure: RiskMeasure = "liquidity".parse().expect("measure should parse");

        assert_eq!(measure, RiskMeasure::Custom("liquidity".to_string()));
    }

    #[test]
    fn displays_and_parses_risk_level() {
        let level: RiskLevel = "critical".parse().expect("level should parse");

        assert_eq!(level, RiskLevel::Critical);
        assert_eq!(level.to_string(), "critical");
    }

    #[test]
    fn constructs_risk_limit() {
        let limit = RiskLimit::new(RiskMeasure::Volatility, 0.20)
            .expect("limit should be valid")
            .with_level(RiskLevel::Medium);

        assert!((limit.threshold() - 0.20).abs() < f64::EPSILON);
        assert_eq!(limit.level(), &RiskLevel::Medium);
    }

    #[test]
    fn constructs_risk_budget() {
        let budget = RiskBudget::new(RiskMeasure::Drawdown, 0.10).expect("budget should be valid");

        assert!((budget.amount() - 0.10).abs() < f64::EPSILON);
    }
}
