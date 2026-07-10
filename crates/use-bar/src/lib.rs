#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

mod error;
mod interval;
mod ohlc;
mod ohlcv;
pub mod prelude;
mod time;
mod volume;

pub use error::BarError;
pub use interval::{BarInterval, BarIntervalError};
pub use ohlc::OhlcBar;
pub use ohlcv::OhlcvBar;
pub use time::{BarTime, BarTimeError};
pub use volume::{BarVolume, BarVolumeError};
