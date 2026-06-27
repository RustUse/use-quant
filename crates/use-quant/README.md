# use-quant

Facade crate for `RustUse` quantitative finance primitives.

`use-quant` is a thin re-export layer over focused child crates for market prices, returns, bars, ticks, price series, volatility, drawdowns, portfolio weights, factors, signal scores, and risk vocabulary. Most implementation lives in the focused crates.

This crate is not a trading system, backtesting framework, financial advisor, broker integration, market data provider, charting library, strategy engine, portfolio optimizer, risk platform, finance system, or accounting system.

## Example

```rust
#[cfg(all(
    feature = "market-price",
    feature = "price-series",
    feature = "return",
))]
{
    use use_quant::{market_price, price_series, return_};

    let start = market_price::MarketPrice::new(100.0)?;
    let end = market_price::MarketPrice::new(105.0)?;
    let simple = return_::SimpleReturn::from_prices(start.value(), end.value())?;

    let mut series = price_series::PriceSeries::new();
    series.push(price_series::PricePoint::new(start));
    series.push(price_series::PricePoint::new(end));

    assert!((simple.value() - 0.05).abs() < 1.0e-12);
    assert_eq!(series.len(), 2);
}

# Ok::<(), Box<dyn std::error::Error>>(())
```

## Modules

The facade modules are feature-gated and re-export the focused child crates:

- `market_price` re-exports `use-market-price` with the `market-price` feature
- `return_` re-exports `use-return` with the `return` feature
- `bar` re-exports `use-bar` with the `bar` feature
- `tick` re-exports `use-tick` with the `tick` feature
- `price_series` re-exports `use-price-series` with the `price-series` feature
- `volatility` re-exports `use-volatility` with the `volatility` feature
- `drawdown` re-exports `use-drawdown` with the `drawdown` feature
- `portfolio_weight` re-exports `use-portfolio-weight` with the `portfolio-weight` feature
- `factor` re-exports `use-factor` with the `factor` feature
- `signal_score` re-exports `use-signal-score` with the `signal-score` feature
- `risk` re-exports `use-risk` with the `risk` feature

## Scope

Use the facade when one dependency and one import surface are useful. Use focused crates directly when a library only needs one primitive area. Business and accounting concepts belong in a possible future `use-finance` set.
