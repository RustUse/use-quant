# use-portfolio-weight

Primitive portfolio weight vocabulary for RustUse quantitative crates.

`use-portfolio-weight` stores finite `f64` weights, including negative weights for short exposure vocabulary, and deterministic asset-weight sets.

## Example

```rust
use use_portfolio_weight::{AssetWeight, PortfolioWeight, WeightSet};

let weights = WeightSet::from_asset_weights([
    AssetWeight::new("ABC", PortfolioWeight::new(0.60)?)?,
    AssetWeight::new("XYZ", PortfolioWeight::new(0.40)?)?,
])?;

assert_eq!(weights.sum(), 1.0);
assert!(weights.is_approximately_fully_invested(1.0e-12)?);
# Ok::<(), Box<dyn std::error::Error>>(())
```

## Scope

Use this crate for descriptive asset weights. It does not optimize portfolios, rebalance, trade, or provide allocation advice.

## License

Licensed under either MIT or Apache-2.0.
