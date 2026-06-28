# use-bar

Composable RustUse utility for OHLC and OHLCV market bars.

`use-bar` provides small, validated bar types for quantitative finance code: bar time labels, bar intervals, OHLC bars, OHLCV bars, and bar volume. It validates obvious OHLC and volume relationships without fetching market data, resampling bars, charting candles, storing market data, or implementing vendor-specific formats.

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

assert_eq!(bar.close().value(), 101.25);
assert_eq!(bar.volume().value(), 42_000.0);
assert_eq!(bar.range(), 2.5);
assert_eq!(bar.body(), 1.25);
assert!(bar.is_up());

# Ok::<(), Box<dyn std::error::Error>>(())
```

## Scope

Use this crate for descriptive OHLC and OHLCV values.

This crate does not:

- fetch market data
- resample bars
- build candlestick charts
- store market data externally
- implement vendor-specific formats

## License

Licensed under either MIT or Apache-2.0.
