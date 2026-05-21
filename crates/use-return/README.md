# use-return

Primitive quantitative return vocabulary for `RustUse`.

`use-return` stores finite `f64` return values and provides tiny helpers for simple and log returns from start/end prices.

## Example

```rust
use use_return::{LogReturn, ReturnKind, SimpleReturn};

let simple = SimpleReturn::from_prices(100.0, 105.0)?;
let log = LogReturn::from_prices(100.0, 105.0)?;
let kind: ReturnKind = "simple".parse()?;

assert!((simple.value() - 0.05).abs() < 1.0e-12);
assert!(log.value() > 0.0);
assert_eq!(kind.to_string(), "simple");
# Ok::<(), Box<dyn std::error::Error>>(())
```

## Scope

Use this crate for return values and obvious price-to-return calculations. It does not implement performance analytics, fee/tax accounting, advice, or strategy logic.

## License

Licensed under either MIT or Apache-2.0.
