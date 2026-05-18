# use-quant

Facade crate for RustUse quantitative finance primitives.

`use-quant` is a thin re-export layer over focused child crates for market prices, returns, bars, ticks, price series, volatility, drawdowns, portfolio weights, factors, signal scores, and risk vocabulary. Most implementation lives in the focused crates.

This crate is not a trading system, backtesting framework, financial advisor, broker integration, market data provider, charting library, strategy engine, portfolio optimizer, risk platform, finance system, or accounting system.

## Example

```rust
use use_quant::{market_price, price_series, return_};

let start = market_price::MarketPrice::new(100.0)?;
let end = market_price::MarketPrice::new(105.0)?;
let simple = return_::SimpleReturn::from_prices(start.value(), end.value())?;

let mut series = price_series::PriceSeries::new();
series.push(price_series::PricePoint::new(start));
series.push(price_series::PricePoint::new(end));

assert!((simple.value() - 0.05).abs() < 1.0e-12);
assert_eq!(series.len(), 2);
# Ok::<(), Box<dyn std::error::Error>>(())
```

## Modules

- `market_price` re-exports `use-market-price`
- `return_` re-exports `use-return`
- `bar` re-exports `use-bar`
- `tick` re-exports `use-tick`
- `price_series` re-exports `use-price-series`
- `volatility` re-exports `use-volatility`
- `drawdown` re-exports `use-drawdown`
- `portfolio_weight` re-exports `use-portfolio-weight`
- `factor` re-exports `use-factor`
- `signal_score` re-exports `use-signal-score`
- `risk` re-exports `use-risk`

## Scope

Use the facade when one dependency and one import surface are useful. Use focused crates directly when a library only needs one primitive area. Business and accounting concepts belong in a possible future `use-finance` set.
