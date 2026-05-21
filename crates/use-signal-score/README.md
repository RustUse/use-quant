# use-signal-score

Primitive quantitative signal score vocabulary for `RustUse`.

`use-signal-score` stores signal names, finite scores, directions, and strengths as descriptive primitives. It does not convert scores into trading decisions.

## Example

```rust
use use_signal_score::{SignalDirection, SignalName, SignalScore, SignalStrength};

let score = SignalScore::new(SignalName::new("quality-score")?, 1.2)?
    .with_direction(SignalDirection::Long)
    .with_strength(SignalStrength::Moderate);

assert_eq!(score.score(), 1.2);
assert_eq!(score.direction(), &SignalDirection::Long);
# Ok::<(), Box<dyn std::error::Error>>(())
```

## Scope

Use this crate for descriptive signal vocabulary. It does not implement strategy logic, recommendations, trading rules, or alerts.

## License

Licensed under either MIT or Apache-2.0.
