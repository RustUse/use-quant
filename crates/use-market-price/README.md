# use-market-price

Primitive market price vocabulary for `RustUse` quantitative crates.

`use-market-price` stores finite non-negative `f64` market price values and descriptive price kinds without fetching prices, modeling money, or connecting to market data APIs.

## Example

```rust
use use_market_price::{MarketPrice, PriceKind, PriceQuote};

let price = MarketPrice::new(101.25)?;
let kind: PriceKind = "adjusted-close".parse()?;
let quote = PriceQuote::new(kind, price);

assert!(quote.price().is_positive());
assert_eq!(quote.kind().to_string(), "adjusted-close");
# Ok::<(), Box<dyn std::error::Error>>(())
```

## Scope

Use this crate for small market price values, price-kind vocabulary, and simple quotes. It does not model money, currencies, decimal arithmetic, market data clients, or broker connections.

## License

Licensed under either MIT or Apache-2.0.
