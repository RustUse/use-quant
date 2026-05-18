# use-risk

Primitive quantitative risk vocabulary for RustUse.

`use-risk` stores risk measures, levels, numeric limits, and budgets as simple descriptive wrappers.

## Example

```rust
use use_risk::{RiskBudget, RiskLevel, RiskLimit, RiskMeasure};

let limit = RiskLimit::new(RiskMeasure::Volatility, 0.20)?.with_level(RiskLevel::Medium);
let budget = RiskBudget::new(RiskMeasure::Drawdown, 0.10)?;

assert_eq!(limit.measure().to_string(), "volatility");
assert_eq!(budget.amount(), 0.10);
# Ok::<(), Box<dyn std::error::Error>>(())
```

## Scope

Use this crate for simple risk vocabulary and numeric thresholds. It does not compute VaR/ES, run a risk platform, report regulation, or provide advice.

## License

Licensed under either MIT or Apache-2.0.
