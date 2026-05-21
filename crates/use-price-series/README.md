# use-price-series

Primitive ordered price-series vocabulary for `RustUse` quantitative crates.

`use-price-series` stores price points in insertion order and exposes lightweight collection helpers. It can compute adjacent simple returns without becoming a dataframe, database, indicator library, or market data store.

## Example

```rust
use use_market_price::MarketPrice;
use use_price_series::{PricePoint, PriceSeries};

let mut series = PriceSeries::new();
series.push(PricePoint::new(MarketPrice::new(100.0)?));
series.push(PricePoint::new(MarketPrice::new(105.0)?));

let returns = series.adjacent_simple_returns()?;

assert_eq!(series.len(), 2);
assert!((returns[0].value() - 0.05).abs() < 1.0e-12);
# Ok::<(), Box<dyn std::error::Error>>(())
```

## Scope

Use this crate for small ordered price collections. It does not implement dataframe behavior, time-series databases, indicators, or data fetching.

## License

Licensed under either MIT or Apache-2.0.
