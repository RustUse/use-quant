# use-tick

Primitive tick, trade, and quote vocabulary for RustUse quantitative crates.

`use-tick` describes single market ticks, trade ticks, and quote ticks with optional labels and sizes. It rejects obviously crossed quotes without implementing order books or streams.

## Example

```rust
use use_market_price::MarketPrice;
use use_tick::{QuoteTick, TradeTick};

let trade = TradeTick::new(MarketPrice::new(101.25)?).with_size(100.0)?;
let quote = QuoteTick::new(Some(MarketPrice::new(101.20)?), Some(MarketPrice::new(101.30)?))?;

assert_eq!(trade.tick().price().value(), 101.25);
assert!((quote.spread().unwrap() - 0.10).abs() < 1.0e-12);
# Ok::<(), Box<dyn std::error::Error>>(())
```

## Scope

Use this crate for plain market microstructure vocabulary. It does not implement order books, exchange connectivity, streaming APIs, or tick aggregation.

## License

Licensed under either MIT or Apache-2.0.
