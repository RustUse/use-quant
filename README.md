# use-quant

RustUse is "Composable sets of primitive Rust utility crates for fellow crustaceans."

`use-quant` is a primitive quantitative finance vocabulary set. It provides small, composable Rust primitives for market prices, returns, bars, ticks, price series, volatility, drawdowns, portfolio weights, factors, signal scores, and risk vocabulary.

`use-quant` is not a trading bot, broker API, backtesting framework, charting library, market data provider, financial advisor, portfolio manager, strategy engine, risk platform, finance system, or accounting system.

## Boundary

`use-quant` owns quantitative finance and market-analysis primitives: prices, returns, bars, ticks, price series, volatility, drawdowns, portfolio weights, factors, signal scores, and risk-measure vocabulary.

`use-finance`, if created later, should own business and accounting concepts such as money, accounts, transactions, invoices, payments, ledgers, and balances.

This workspace describes quantitative finance concepts. It does not trade, fetch live data, advise, backtest, optimize, execute orders, or manage accounts.

## Crates

| Crate                  | Purpose                                                       |
| ---------------------- | ------------------------------------------------------------- |
| `use-quant`            | Thin facade over the focused quantitative vocabulary crates.  |
| `use-market-price`     | Market price values, price kinds, and price quotes.           |
| `use-return`           | Simple, log, and descriptive return values.                   |
| `use-bar`              | OHLC and OHLCV bar vocabulary.                                |
| `use-tick`             | Trade, quote, bid, ask, and tick vocabulary.                  |
| `use-price-series`     | Ordered price points and lightweight price series.            |
| `use-volatility`       | Volatility values, kinds, windows, and small realized helper. |
| `use-drawdown`         | Drawdown values, points, windows, and small max helper.       |
| `use-portfolio-weight` | Asset weights and deterministic weight sets.                  |
| `use-factor`           | Factor names, exposures, loadings, and model names.           |
| `use-signal-score`     | Signal names, scores, directions, and strengths.              |
| `use-risk`             | Risk measure, level, limit, and budget vocabulary.            |

## Example

```rust
use use_quant::{
    bar, drawdown, factor, market_price, portfolio_weight, price_series, return_, signal_score,
    volatility,
};

let close = market_price::MarketPrice::new(101.25)?;
let simple_return = return_::SimpleReturn::from_prices(100.0, close.value())?;

let ohlcv = bar::OhlcvBar::from_values(
    bar::BarTime::new("2026-05-17")?,
    bar::BarInterval::Day,
    100.0,
    102.0,
    99.5,
    close.value(),
    42_000.0,
)?;

let mut series = price_series::PriceSeries::new();
series.push(price_series::PricePoint::new(market_price::MarketPrice::new(100.0)?));
series.push(price_series::PricePoint::new(close));

let realized = volatility::Volatility::sample_from_returns(&[simple_return.value()])
    .unwrap_or(volatility::Volatility::new(0.0)?);
let drawdown = drawdown::Drawdown::from_peak_current(105.0, close.value())?;
let weight = portfolio_weight::AssetWeight::new("ABC", portfolio_weight::PortfolioWeight::new(0.25)? )?;
let exposure = factor::FactorExposure::new(factor::FactorName::new("momentum")?, 0.7)?;
let signal = signal_score::SignalScore::new(signal_score::SignalName::new("quality-score")?, 1.2)?;

assert_eq!(ohlcv.bar().close().value(), 101.25);
assert_eq!(series.len(), 2);
assert_eq!(realized.value(), 0.0);
assert!(drawdown.value() <= 0.0);
assert_eq!(weight.asset_id(), "ABC");
assert_eq!(exposure.factor().as_str(), "momentum");
assert_eq!(signal.score(), 1.2);
# Ok::<(), Box<dyn std::error::Error>>(())
```

The example composes primitives that downstream crates can store, compare, serialize, or transform. It does not trade, fetch market data, advise, backtest, optimize, or execute anything.

## Related Sets

- `use-math`
- `use-stats`
- `use-time`
- `use-data`
- `use-measure`
- `use-validate`

## License

Licensed under either of the following, at your option:

- Apache License, Version 2.0, in `LICENSE-APACHE`
- MIT license, in `LICENSE-MIT`
