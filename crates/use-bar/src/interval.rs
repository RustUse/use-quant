use std::{error::Error, fmt, str::FromStr};

/// A market bar interval label.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum BarInterval {
    /// Tick-level bar.
    Tick,
    /// One-second bar.
    Second,
    /// One-minute bar.
    Minute,
    /// One-hour bar.
    Hour,
    /// One-day bar.
    Day,
    /// One-week bar.
    Week,
    /// One-month bar.
    Month,
    /// Custom interval label.
    Custom(String),
}

impl BarInterval {
    /// Creates a custom bar interval label.
    ///
    /// # Errors
    ///
    /// Returns [`BarIntervalError::Empty`] when the label is empty or whitespace only.
    pub fn custom(value: impl Into<String>) -> Result<Self, BarIntervalError> {
        let value = value.into();

        if value.trim().is_empty() {
            return Err(BarIntervalError::Empty);
        }

        Ok(Self::Custom(value))
    }

    /// Returns the interval as a stable string label.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::Tick => "tick",
            Self::Second => "second",
            Self::Minute => "minute",
            Self::Hour => "hour",
            Self::Day => "day",
            Self::Week => "week",
            Self::Month => "month",
            Self::Custom(value) => value,
        }
    }
}

impl fmt::Display for BarInterval {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for BarInterval {
    type Err = BarIntervalError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value.trim().to_ascii_lowercase().as_str() {
            "" => Err(BarIntervalError::Empty),
            "tick" => Ok(Self::Tick),
            "second" | "1s" => Ok(Self::Second),
            "minute" | "1m" => Ok(Self::Minute),
            "hour" | "1h" => Ok(Self::Hour),
            "day" | "1d" => Ok(Self::Day),
            "week" | "1w" => Ok(Self::Week),
            "month" | "1mo" => Ok(Self::Month),
            _ => Self::custom(value),
        }
    }
}

impl TryFrom<&str> for BarInterval {
    type Error = BarIntervalError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        value.parse()
    }
}

impl TryFrom<String> for BarInterval {
    type Error = BarIntervalError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.parse()
    }
}

/// Errors returned when creating or parsing a [`BarInterval`].
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BarIntervalError {
    /// The interval label was empty or whitespace only.
    Empty,
}

impl fmt::Display for BarIntervalError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("bar interval label must not be empty"),
        }
    }
}

impl Error for BarIntervalError {}

#[cfg(test)]
mod tests {
    use super::{BarInterval, BarIntervalError};

    #[test]
    fn displays_standard_intervals() {
        assert_eq!(BarInterval::Tick.to_string(), "tick");
        assert_eq!(BarInterval::Second.to_string(), "second");
        assert_eq!(BarInterval::Minute.to_string(), "minute");
        assert_eq!(BarInterval::Hour.to_string(), "hour");
        assert_eq!(BarInterval::Day.to_string(), "day");
        assert_eq!(BarInterval::Week.to_string(), "week");
        assert_eq!(BarInterval::Month.to_string(), "month");
    }

    #[test]
    fn creates_custom_interval() {
        let interval = BarInterval::custom("5m").expect("interval should be valid");

        assert_eq!(interval.as_str(), "5m");
    }

    #[test]
    fn rejects_empty_custom_interval() {
        assert_eq!(BarInterval::custom(""), Err(BarIntervalError::Empty));
        assert_eq!(BarInterval::custom("   "), Err(BarIntervalError::Empty));
    }

    #[test]
    fn parses_standard_intervals() {
        assert_eq!("tick".parse::<BarInterval>(), Ok(BarInterval::Tick));
        assert_eq!("1s".parse::<BarInterval>(), Ok(BarInterval::Second));
        assert_eq!("1m".parse::<BarInterval>(), Ok(BarInterval::Minute));
        assert_eq!("1h".parse::<BarInterval>(), Ok(BarInterval::Hour));
        assert_eq!("1d".parse::<BarInterval>(), Ok(BarInterval::Day));
        assert_eq!("1w".parse::<BarInterval>(), Ok(BarInterval::Week));
        assert_eq!("1mo".parse::<BarInterval>(), Ok(BarInterval::Month));
    }

    #[test]
    fn parses_unknown_interval_as_custom() {
        assert_eq!(
            "15m".parse::<BarInterval>(),
            Ok(BarInterval::Custom(String::from("15m")))
        );
    }

    #[test]
    fn rejects_empty_parse_value() {
        assert_eq!("".parse::<BarInterval>(), Err(BarIntervalError::Empty));
        assert_eq!("   ".parse::<BarInterval>(), Err(BarIntervalError::Empty));
    }
}
