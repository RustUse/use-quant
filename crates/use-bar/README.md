# use-bar

Primitive OHLC and OHLCV bar vocabulary for `RustUse` quantitative crates.

`use-bar` describes bars with simple labels, intervals, prices, and optional volume. It validates obvious OHLC relationships without fetching, resampling, charting, or storing market data externally.

## Example

```rust
use use_bar::{BarInterval, BarTime, OhlcvBar};

let bar = OhlcvBar::from_values(
    BarTime::new("2026-05-17")?,
    BarInterval::Day,
    100.0,
    102.0,
    99.5,
    101.25,
    42_000.0,
)?;

assert_eq!(bar.bar().close().value(), 101.25);
assert_eq!(bar.volume(), 42_000.0);
# Ok::<(), Box<dyn std::error::Error>>(())
```

## Scope

Use this crate for descriptive OHLC/OHLCV values. It does not fetch data, resample bars, build candlestick charts, or implement vendor formats.

## License

Licensed under either MIT or Apache-2.0.
