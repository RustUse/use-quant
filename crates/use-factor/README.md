# use-factor

Primitive quantitative factor vocabulary for RustUse.

`use-factor` stores non-empty factor names, model names, exposures, and loadings as descriptive primitives.

## Example

```rust
use use_factor::{FactorExposure, FactorLoading, FactorName};

let factor = FactorName::new("momentum")?;
let exposure = FactorExposure::new(factor.clone(), 0.7)?;
let loading = FactorLoading::new(factor, 1.2)?;

assert_eq!(exposure.factor().as_str(), "momentum");
assert_eq!(loading.value(), 1.2);
# Ok::<(), Box<dyn std::error::Error>>(())
```

## Scope

Use this crate for descriptive factor vocabulary. It does not estimate regressions, fetch factor data, implement a modeling framework, or provide recommendations.

## License

Licensed under either MIT or Apache-2.0.
