#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

pub mod prelude;

#[cfg(feature = "bar")]
pub use use_bar as bar;

#[cfg(feature = "drawdown")]
pub use use_drawdown as drawdown;

#[cfg(feature = "factor")]
pub use use_factor as factor;

#[cfg(feature = "market-price")]
pub use use_market_price as market_price;

#[cfg(feature = "portfolio-weight")]
pub use use_portfolio_weight as portfolio_weight;

#[cfg(feature = "price-series")]
pub use use_price_series as price_series;

#[cfg(feature = "return")]
pub use use_return as return_;

#[cfg(feature = "risk")]
pub use use_risk as risk;

#[cfg(feature = "signal-score")]
pub use use_signal_score as signal_score;

#[cfg(feature = "tick")]
pub use use_tick as tick;

#[cfg(feature = "volatility")]
pub use use_volatility as volatility;
