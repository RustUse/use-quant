#![forbid(unsafe_code)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc = include_str!("../README.md")]

pub mod prelude;

#[cfg(feature = "bar")]
#[cfg_attr(docsrs, doc(cfg(feature = "bar")))]
pub use use_bar as bar;

#[cfg(feature = "drawdown")]
#[cfg_attr(docsrs, doc(cfg(feature = "drawdown")))]
pub use use_drawdown as drawdown;

#[cfg(feature = "factor")]
#[cfg_attr(docsrs, doc(cfg(feature = "factor")))]
pub use use_factor as factor;

#[cfg(feature = "market-price")]
#[cfg_attr(docsrs, doc(cfg(feature = "market-price")))]
pub use use_market_price as market_price;

#[cfg(feature = "portfolio-weight")]
#[cfg_attr(docsrs, doc(cfg(feature = "portfolio-weight")))]
pub use use_portfolio_weight as portfolio_weight;

#[cfg(feature = "price-series")]
#[cfg_attr(docsrs, doc(cfg(feature = "price-series")))]
pub use use_price_series as price_series;

#[cfg(feature = "return")]
#[cfg_attr(docsrs, doc(cfg(feature = "return")))]
pub use use_return as return_;

#[cfg(feature = "risk")]
#[cfg_attr(docsrs, doc(cfg(feature = "risk")))]
pub use use_risk as risk;

#[cfg(feature = "signal-score")]
#[cfg_attr(docsrs, doc(cfg(feature = "signal-score")))]
pub use use_signal_score as signal_score;

#[cfg(feature = "tick")]
#[cfg_attr(docsrs, doc(cfg(feature = "tick")))]
pub use use_tick as tick;

#[cfg(feature = "volatility")]
#[cfg_attr(docsrs, doc(cfg(feature = "volatility")))]
pub use use_volatility as volatility;
