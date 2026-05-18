#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::fmt;
use std::error::Error;

/// Common drawdown primitives.
pub mod prelude {
    pub use crate::{Drawdown, DrawdownError, DrawdownPoint, DrawdownWindow};
}

/// A finite drawdown value using the convention `current / peak - 1.0`, capped at `0.0`.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Drawdown {
    value: f64,
}

impl Drawdown {
    /// Creates a drawdown value.
    ///
    /// # Errors
    ///
    /// Returns [`DrawdownError::NonFinite`] for non-finite values and
    /// [`DrawdownError::Positive`] for positive values.
    pub fn new(value: f64) -> Result<Self, DrawdownError> {
        if !value.is_finite() {
            return Err(DrawdownError::NonFinite);
        }

        if value > 0.0 {
            return Err(DrawdownError::Positive);
        }

        Ok(Self { value })
    }

    /// Computes drawdown from a positive peak and non-negative current value.
    ///
    /// # Errors
    ///
    /// Returns [`DrawdownError`] when inputs are non-finite, the peak is not positive, or the
    /// current value is negative.
    pub fn from_peak_current(peak: f64, current: f64) -> Result<Self, DrawdownError> {
        validate_peak(peak)?;
        validate_current(current)?;

        if current >= peak {
            Self::new(0.0)
        } else {
            Self::new((current / peak) - 1.0)
        }
    }

    /// Computes maximum drawdown over a simple price or equity series.
    ///
    /// # Errors
    ///
    /// Returns [`DrawdownError`] for empty series, invalid first peak, or invalid values.
    pub fn maximum_from_values(values: &[f64]) -> Result<Self, DrawdownError> {
        let Some((&first, rest)) = values.split_first() else {
            return Err(DrawdownError::EmptySeries);
        };

        validate_peak(first)?;
        let mut peak = first;
        let mut maximum = Self::new(0.0)?;

        for &value in rest {
            validate_current(value)?;
            if value > peak {
                peak = value;
            }

            let drawdown = Self::from_peak_current(peak, value)?;
            if drawdown.value() < maximum.value() {
                maximum = drawdown;
            }
        }

        Ok(maximum)
    }

    /// Returns the drawdown value.
    #[must_use]
    pub const fn value(self) -> f64 {
        self.value
    }
}

impl fmt::Display for Drawdown {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.value.fmt(formatter)
    }
}

/// A drawdown point with an optional label.
#[derive(Clone, Debug, PartialEq)]
pub struct DrawdownPoint {
    label: Option<String>,
    drawdown: Drawdown,
}

impl DrawdownPoint {
    /// Creates an unlabeled drawdown point.
    #[must_use]
    pub const fn new(drawdown: Drawdown) -> Self {
        Self {
            label: None,
            drawdown,
        }
    }

    /// Attaches a non-empty label.
    ///
    /// # Errors
    ///
    /// Returns [`DrawdownError::EmptyLabel`] when the trimmed label is empty.
    pub fn with_label(mut self, label: impl AsRef<str>) -> Result<Self, DrawdownError> {
        let trimmed = label.as_ref().trim();
        if trimmed.is_empty() {
            return Err(DrawdownError::EmptyLabel);
        }

        self.label = Some(trimmed.to_string());
        Ok(self)
    }

    /// Returns the optional label.
    #[must_use]
    pub fn label(&self) -> Option<&str> {
        self.label.as_deref()
    }

    /// Returns the drawdown.
    #[must_use]
    pub const fn drawdown(&self) -> Drawdown {
        self.drawdown
    }
}

/// A simple observation-count drawdown window.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct DrawdownWindow {
    length: usize,
}

impl DrawdownWindow {
    /// Creates a non-zero drawdown window length.
    ///
    /// # Errors
    ///
    /// Returns [`DrawdownError::ZeroWindow`] when `length` is zero.
    pub const fn new(length: usize) -> Result<Self, DrawdownError> {
        if length == 0 {
            Err(DrawdownError::ZeroWindow)
        } else {
            Ok(Self { length })
        }
    }

    /// Returns the window length.
    #[must_use]
    pub const fn length(self) -> usize {
        self.length
    }
}

/// Errors returned by drawdown helpers.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DrawdownError {
    /// Drawdown and series values must be finite.
    NonFinite,
    /// Drawdowns use zero-or-negative convention.
    Positive,
    /// Peak values must be finite and strictly positive.
    InvalidPeak,
    /// Current values must not be negative.
    NegativeValue,
    /// Maximum drawdown requires at least one value.
    EmptySeries,
    /// Window lengths must be non-zero.
    ZeroWindow,
    /// Labels must be non-empty after trimming whitespace.
    EmptyLabel,
}

impl fmt::Display for DrawdownError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NonFinite => formatter.write_str("drawdown values must be finite"),
            Self::Positive => {
                formatter.write_str("drawdown cannot be positive with this convention")
            },
            Self::InvalidPeak => formatter.write_str("drawdown peak must be finite and positive"),
            Self::NegativeValue => formatter.write_str("drawdown current value cannot be negative"),
            Self::EmptySeries => {
                formatter.write_str("maximum drawdown requires at least one value")
            },
            Self::ZeroWindow => formatter.write_str("drawdown window length must be non-zero"),
            Self::EmptyLabel => formatter.write_str("drawdown point label cannot be empty"),
        }
    }
}

impl Error for DrawdownError {}

fn validate_peak(value: f64) -> Result<(), DrawdownError> {
    if !value.is_finite() || value <= 0.0 {
        Err(DrawdownError::InvalidPeak)
    } else {
        Ok(())
    }
}

fn validate_current(value: f64) -> Result<(), DrawdownError> {
    if !value.is_finite() {
        return Err(DrawdownError::NonFinite);
    }

    if value < 0.0 {
        return Err(DrawdownError::NegativeValue);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{Drawdown, DrawdownError};

    #[test]
    fn computes_drawdown_from_peak_current() {
        let drawdown = Drawdown::from_peak_current(120.0, 90.0).expect("drawdown should compute");

        assert_eq!(drawdown.value(), -0.25);
    }

    #[test]
    fn returns_zero_drawdown_at_new_high() {
        let drawdown = Drawdown::from_peak_current(120.0, 130.0).expect("drawdown should compute");

        assert_eq!(drawdown.value(), 0.0);
    }

    #[test]
    fn rejects_invalid_peak() {
        assert_eq!(
            Drawdown::from_peak_current(0.0, 90.0),
            Err(DrawdownError::InvalidPeak)
        );
    }

    #[test]
    fn computes_maximum_drawdown() {
        let drawdown = Drawdown::maximum_from_values(&[100.0, 120.0, 90.0, 80.0, 130.0])
            .expect("drawdown should compute");

        assert!((drawdown.value() - (-1.0 / 3.0)).abs() < 1.0e-12);
    }
}
