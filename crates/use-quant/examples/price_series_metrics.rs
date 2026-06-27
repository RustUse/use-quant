use use_quant::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let series = PriceSeries::from_points([
        PricePoint::new(MarketPrice::new(100.0)?),
        PricePoint::new(MarketPrice::new(102.0)?),
        PricePoint::new(MarketPrice::new(101.0)?),
        PricePoint::new(MarketPrice::new(105.0)?),
        PricePoint::new(MarketPrice::new(103.0)?),
        PricePoint::new(MarketPrice::new(108.0)?),
    ]);

    let returns = series.adjacent_simple_returns()?;
    let return_values: Vec<f64> = returns.iter().map(|value| value.value()).collect();

    let prices: Vec<f64> = series.iter().map(|point| point.price().value()).collect();

    let volatility = Volatility::sample_from_returns(&return_values)?;
    let drawdown = Drawdown::maximum_from_values(&prices)?;

    println!("price count: {}", series.len());
    println!("return count: {}", returns.len());
    println!("realized volatility: {:.4}", volatility.value());
    println!("max drawdown: {:.4}", drawdown.value());

    assert!(!series.is_empty());
    assert_eq!(returns.len(), series.len() - 1);
    assert!(volatility.value() >= 0.0);
    assert!(drawdown.value() <= 0.0);

    Ok(())
}
