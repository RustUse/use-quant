# use-drawdown

Primitive drawdown vocabulary for `RustUse` quantitative crates.

`use-drawdown` uses the convention `current / peak - 1.0`, capped at `0.0` for new highs. Drawdowns are therefore zero or negative finite `f64` values.

## Example

```rust
use use_drawdown::Drawdown;

let drawdown = Drawdown::from_peak_current(120.0, 90.0)?;
let maximum = Drawdown::maximum_from_values(&[100.0, 120.0, 90.0, 130.0])?;

assert_eq!(drawdown.value(), -0.25);
assert_eq!(maximum.value(), -0.25);
# Ok::<(), Box<dyn std::error::Error>>(())
```

## Scope

Use this crate for drawdown values and small peak/current calculations. It does not implement performance analytics, reporting, or charts.

## License

Licensed under either MIT or Apache-2.0.
