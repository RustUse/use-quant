# use-volatility

Primitive volatility vocabulary for RustUse quantitative crates.

`use-volatility` stores finite non-negative volatility values and descriptive volatility kinds. Its only calculation helper is a small sample standard-deviation helper over return values.

## Example

```rust
use use_volatility::{Volatility, VolatilityKind};

let volatility = Volatility::sample_from_returns(&[0.01, -0.02, 0.015])?;
let kind: VolatilityKind = "realized".parse()?;

assert!(volatility.value() > 0.0);
assert_eq!(kind.to_string(), "realized");
# Ok::<(), Box<dyn std::error::Error>>(())
```

## Scope

Use this crate for volatility values and small descriptive helpers. It does not implement options pricing, GARCH models, annualization policy, or risk-engine behavior.

## License

Licensed under either MIT or Apache-2.0.
