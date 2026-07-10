# use-quant

RustUse is "Composable sets of primitive Rust utility crates for fellow crustaceans."

`use-quant` is a primitive quantitative finance vocabulary set. It provides small, composable Rust primitives for market prices, returns, bars, ticks, price series, volatility, drawdowns, portfolio weights, factors, signal scores, and risk vocabulary.

`use-quant` is a thin facade over focused child crates. Most implementation lives in the focused crates under `crates/*`; the root package provides one dependency and one import surface for users who want the whole quantitative vocabulary set available through feature-gated modules.

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

## Features

The facade modules are feature-gated and re-export the focused child crates.

| Feature            | Module             | Re-exported crate      |
| ------------------ | ------------------ | ---------------------- |
| `market-price`     | `market_price`     | `use-market-price`     |
| `return`           | `return_`          | `use-return`           |
| `bar`              | `bar`              | `use-bar`              |
| `tick`             | `tick`             | `use-tick`             |
| `price-series`     | `price_series`     | `use-price-series`     |
| `volatility`       | `volatility`       | `use-volatility`       |
| `drawdown`         | `drawdown`         | `use-drawdown`         |
| `portfolio-weight` | `portfolio_weight` | `use-portfolio-weight` |
| `factor`           | `factor`           | `use-factor`           |
| `signal-score`     | `signal_score`     | `use-signal-score`     |
| `risk`             | `risk`             | `use-risk`             |

Enable everything with:

```toml
[dependencies]
use-quant = { version = "0.2.0", features = ["full"] }
```

Or enable only the primitive areas needed by your crate:

```toml
[dependencies]
use-quant = { version = "0.2.0", features = ["market-price", "return", "price-series"] }
```

## Example

```rust
#[cfg(all(
    feature = "bar",
    feature = "drawdown",
    feature = "factor",
    feature = "market-price",
    feature = "portfolio-weight",
    feature = "price-series",
    feature = "return",
    feature = "signal-score",
    feature = "volatility",
))]
{
    use use_quant::{
        bar, drawdown, factor, market_price, portfolio_weight, price_series, return_,
        signal_score, volatility,
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
    series.push(price_series::PricePoint::new(
        market_price::MarketPrice::new(100.0)?,
    ));
    series.push(price_series::PricePoint::new(close));

    let realized = volatility::Volatility::sample_from_returns(&[simple_return.value()])
        .unwrap_or(volatility::Volatility::new(0.0)?);
    let drawdown = drawdown::Drawdown::from_peak_current(105.0, close.value())?;
    let weight = portfolio_weight::AssetWeight::new(
        "ABC",
        portfolio_weight::PortfolioWeight::new(0.25)?,
    )?;
    let exposure = factor::FactorExposure::new(factor::FactorName::new("momentum")?, 0.7)?;
    let signal =
        signal_score::SignalScore::new(signal_score::SignalName::new("quality-score")?, 1.2)?;

    assert_eq!(ohlcv.bar().close().value(), 101.25);
    assert_eq!(series.len(), 2);
    assert_eq!(realized.value(), 0.0);
    assert!(drawdown.value() <= 0.0);
    assert_eq!(weight.asset_id(), "ABC");
    assert_eq!(exposure.factor().as_str(), "momentum");
    assert_eq!(signal.score(), 1.2);
}

# Ok::<(), Box<dyn std::error::Error>>(())
```

The example composes primitives that downstream crates can store, compare, serialize, or transform. It does not trade, fetch market data, advise, backtest, optimize, or execute anything.

## Scope

Use the facade when one dependency and one import surface are useful. Use focused crates directly when a library only needs one primitive area.

Business and accounting concepts belong in a possible future `use-finance` set.

## Repository Layout

```text
use-quant/
  Cargo.toml
  README.md
  src/
    lib.rs
    prelude.rs
  examples/
  crates/
    use-bar/
    use-drawdown/
    use-factor/
    use-market-price/
    use-portfolio-weight/
    use-price-series/
    use-return/
    use-risk/
    use-signal-score/
    use-tick/
    use-volatility/
```

The repository root is the `use-quant` facade package and workspace root. Child crates live under `crates/*`.

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
