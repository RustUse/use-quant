use std::{error::Error, fmt};

/// A validated bar volume value.
///
/// `BarVolume` accepts finite, non-negative values. Zero volume is valid because
/// some markets, synthetic bars, or missing-trade intervals may produce bars
/// with no observed volume.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct BarVolume(f64);

impl BarVolume {
    /// Creates a validated bar volume.
    ///
    /// # Errors
    ///
    /// Returns [`BarVolumeError::Invalid`] when the value is negative, `NaN`,
    /// positive infinity, or negative infinity.
    pub fn new(value: f64) -> Result<Self, BarVolumeError> {
        if !value.is_finite() || value.is_sign_negative() {
            return Err(BarVolumeError::Invalid);
        }

        Ok(Self(value))
    }

    /// Returns the raw volume value.
    #[must_use]
    pub const fn value(self) -> f64 {
        self.0
    }

    /// Consumes the volume and returns the wrapped value.
    #[must_use]
    pub const fn into_inner(self) -> f64 {
        self.0
    }

    /// Returns `true` when the volume is greater than zero.
    #[must_use]
    pub fn is_nonzero(self) -> bool {
        self.0 > 0.0
    }
}

impl fmt::Display for BarVolume {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

impl TryFrom<f64> for BarVolume {
    type Error = BarVolumeError;

    fn try_from(value: f64) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl From<BarVolume> for f64 {
    fn from(volume: BarVolume) -> Self {
        volume.value()
    }
}

/// Errors returned when creating a [`BarVolume`].
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BarVolumeError {
    /// The volume was negative or non-finite.
    Invalid,
}

impl fmt::Display for BarVolumeError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Invalid => {
                formatter.write_str("bar volume must be finite and greater than or equal to zero")
            },
        }
    }
}

impl Error for BarVolumeError {}

#[cfg(test)]
mod tests {
    use super::{BarVolume, BarVolumeError};
    use crate::BarError;

    #[test]
    fn constructs_valid_volume() {
        let volume = BarVolume::new(42_000.0).expect("volume should be valid");

        assert!((volume.value() - 42_000.0).abs() < f64::EPSILON);
    }

    #[test]
    fn allows_zero_volume() {
        let volume = BarVolume::new(0.0).expect("zero volume should be valid");

        assert!(volume.value().abs() < f64::EPSILON);
        assert!(!volume.is_nonzero());
    }

    #[test]
    fn detects_nonzero_volume() {
        let volume = BarVolume::new(1.0).expect("volume should be valid");

        assert!(volume.is_nonzero());
    }

    #[test]
    fn rejects_negative_volume() {
        assert_eq!(BarVolume::new(-1.0), Err(BarVolumeError::Invalid));
    }

    #[test]
    fn rejects_nan_volume() {
        assert_eq!(BarVolume::new(f64::NAN), Err(BarVolumeError::Invalid));
    }

    #[test]
    fn rejects_infinite_volume() {
        assert_eq!(BarVolume::new(f64::INFINITY), Err(BarVolumeError::Invalid));

        assert_eq!(
            BarVolume::new(f64::NEG_INFINITY),
            Err(BarVolumeError::Invalid)
        );
    }

    #[test]
    fn converts_from_f64() {
        let volume = BarVolume::try_from(42_000.0).expect("volume should be valid");

        assert!((volume.value() - 42_000.0).abs() < f64::EPSILON);
    }

    #[test]
    fn converts_into_f64() {
        let volume = BarVolume::new(42_000.0).expect("volume should be valid");
        let value = f64::from(volume);

        assert!((value - 42_000.0).abs() < f64::EPSILON);
    }

    #[test]
    fn displays_volume() {
        let volume = BarVolume::new(42_000.0).expect("volume should be valid");

        assert_eq!(volume.to_string(), "42000");
    }

    #[test]
    fn converts_error_into_bar_error() {
        let error = BarError::from(BarVolumeError::Invalid);

        assert_eq!(error, BarError::InvalidVolume(BarVolumeError::Invalid));
    }
}
