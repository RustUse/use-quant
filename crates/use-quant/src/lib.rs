#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

//! Thin facade for RustUse quantitative finance primitive crates.
//!
//! `use-quant` describes quantitative finance vocabulary. It is not a trading system,
//! backtesting framework, financial advisor, broker integration, market data provider, charting
//! library, portfolio optimizer, risk platform, finance system, or accounting system.

pub use use_bar as bar;
pub use use_drawdown as drawdown;
pub use use_factor as factor;
pub use use_market_price as market_price;
pub use use_portfolio_weight as portfolio_weight;
pub use use_price_series as price_series;
pub use use_return as return_;
pub use use_risk as risk;
pub use use_signal_score as signal_score;
pub use use_tick as tick;
pub use use_volatility as volatility;

/// Common quantitative primitive types from the focused crates.
pub mod prelude {
    pub use crate::bar::{BarInterval, BarTime, OhlcBar, OhlcvBar};
    pub use crate::drawdown::{Drawdown, DrawdownPoint, DrawdownWindow};
    pub use crate::factor::{FactorExposure, FactorLoading, FactorModelName, FactorName};
    pub use crate::market_price::{MarketPrice, PriceKind, PriceQuote};
    pub use crate::portfolio_weight::{AssetWeight, PortfolioWeight, WeightSet};
    pub use crate::price_series::{PricePoint, PriceSeries, SeriesName};
    pub use crate::return_::{LogReturn, ReturnKind, ReturnValue, SimpleReturn};
    pub use crate::risk::{RiskBudget, RiskLevel, RiskLimit, RiskMeasure};
    pub use crate::signal_score::{SignalDirection, SignalName, SignalScore, SignalStrength};
    pub use crate::tick::{QuoteTick, Tick, TickKind, TradeTick};
    pub use crate::volatility::{Volatility, VolatilityKind, VolatilityWindow};
}

#[cfg(test)]
mod tests {
    use super::{
        bar, drawdown, factor, market_price, portfolio_weight, price_series, return_, risk,
        signal_score, tick, volatility,
    };

    #[test]
    fn facade_exposes_composable_quant_primitives() -> Result<(), Box<dyn std::error::Error>> {
        let close = market_price::MarketPrice::new(101.25)?;
        let simple_return = return_::SimpleReturn::from_prices(100.0, close.value())?;
        let bar = bar::OhlcvBar::from_values(
            bar::BarTime::new("2026-05-17")?,
            bar::BarInterval::Day,
            100.0,
            102.0,
            99.5,
            close.value(),
            42_000.0,
        )?;
        let trade = tick::TradeTick::new(close).with_size(100.0)?;
        let mut series = price_series::PriceSeries::new();
        series.push(price_series::PricePoint::new(
            market_price::MarketPrice::new(100.0)?,
        ));
        series.push(price_series::PricePoint::new(close));
        let volatility = volatility::Volatility::new(0.20)?;
        let drawdown = drawdown::Drawdown::from_peak_current(105.0, close.value())?;
        let weight = portfolio_weight::AssetWeight::new(
            "ABC",
            portfolio_weight::PortfolioWeight::new(0.25)?,
        )?;
        let exposure = factor::FactorExposure::new(factor::FactorName::new("momentum")?, 0.7)?;
        let signal =
            signal_score::SignalScore::new(signal_score::SignalName::new("quality-score")?, 1.2)?;
        let limit = risk::RiskLimit::new(risk::RiskMeasure::Volatility, 0.20)?;

        assert!((simple_return.value() - 0.0125).abs() < 1.0e-12);
        assert_eq!(bar.bar().close().value(), 101.25);
        assert_eq!(trade.tick().size(), Some(100.0));
        assert_eq!(series.len(), 2);
        assert_eq!(volatility.value(), 0.20);
        assert!(drawdown.value() <= 0.0);
        assert_eq!(weight.asset_id(), "ABC");
        assert_eq!(exposure.factor().as_str(), "momentum");
        assert_eq!(signal.score(), 1.2);
        assert_eq!(limit.measure(), &risk::RiskMeasure::Volatility);
        Ok(())
    }
}
