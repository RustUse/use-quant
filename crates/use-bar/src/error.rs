use std::{error::Error, fmt};

use use_market_price::MarketPriceError;

use crate::{BarTimeError, BarVolumeError};

/// Errors returned while constructing or deriving bar values.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum BarError {
    /// The bar time label was invalid.
    InvalidTime(BarTimeError),
    /// One of the price values was invalid.
    InvalidPrice(MarketPriceError),
    /// The high price was lower than open, low, or close.
    InvalidHigh,
    /// The low price was higher than open, high, or close.
    InvalidLow,
    /// The volume value was invalid.
    InvalidVolume(BarVolumeError),
}

impl fmt::Display for BarError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidTime(error) => write!(formatter, "{error}"),
            Self::InvalidPrice(error) => write!(formatter, "{error}"),
            Self::InvalidHigh => {
                formatter.write_str("bar high must be at least open, low, and close")
            },
            Self::InvalidLow => {
                formatter.write_str("bar low must be at most open, high, and close")
            },
            Self::InvalidVolume(error) => write!(formatter, "{error}"),
        }
    }
}

impl Error for BarError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::InvalidTime(error) => Some(error),
            Self::InvalidPrice(error) => Some(error),
            Self::InvalidHigh | Self::InvalidLow => None,
            Self::InvalidVolume(error) => Some(error),
        }
    }
}

impl From<BarTimeError> for BarError {
    fn from(error: BarTimeError) -> Self {
        Self::InvalidTime(error)
    }
}

impl From<MarketPriceError> for BarError {
    fn from(error: MarketPriceError) -> Self {
        Self::InvalidPrice(error)
    }
}

impl From<BarVolumeError> for BarError {
    fn from(error: BarVolumeError) -> Self {
        Self::InvalidVolume(error)
    }
}

#[cfg(test)]
mod tests {
    use std::error::Error;

    use use_market_price::MarketPrice;

    use super::BarError;
    use crate::{BarTimeError, BarVolumeError};

    #[test]
    fn displays_invalid_time_error() {
        let error = BarError::InvalidTime(BarTimeError::Empty);

        assert_eq!(error.to_string(), "bar time label must not be empty");
    }

    #[test]
    fn displays_invalid_high_error() {
        assert_eq!(
            BarError::InvalidHigh.to_string(),
            "bar high must be at least open, low, and close"
        );
    }

    #[test]
    fn displays_invalid_low_error() {
        assert_eq!(
            BarError::InvalidLow.to_string(),
            "bar low must be at most open, high, and close"
        );
    }

    #[test]
    fn displays_invalid_volume_error() {
        let error = BarError::InvalidVolume(BarVolumeError::Invalid);

        assert_eq!(
            error.to_string(),
            "bar volume must be finite and greater than or equal to zero"
        );
    }

    #[test]
    fn converts_time_error() {
        let error = BarError::from(BarTimeError::Empty);

        assert_eq!(error, BarError::InvalidTime(BarTimeError::Empty));
    }

    #[test]
    fn converts_market_price_error() {
        let market_price_error = MarketPrice::new(f64::NAN).expect_err("price should be invalid");
        let error = BarError::from(market_price_error);

        assert_eq!(error, BarError::InvalidPrice(market_price_error));
    }

    #[test]
    fn converts_volume_error() {
        let error = BarError::from(BarVolumeError::Invalid);

        assert_eq!(error, BarError::InvalidVolume(BarVolumeError::Invalid));
    }

    #[test]
    fn exposes_nested_error_sources() {
        let time_error = BarError::InvalidTime(BarTimeError::Empty);
        let volume_error = BarError::InvalidVolume(BarVolumeError::Invalid);

        assert!(time_error.source().is_some());
        assert!(volume_error.source().is_some());
        assert!(BarError::InvalidHigh.source().is_none());
        assert!(BarError::InvalidLow.source().is_none());
    }
}
