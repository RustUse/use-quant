use use_quant::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let series = PriceSeries::from_prices([100.0, 102.0, 101.0, 105.0, 103.0, 108.0])?;

    let returns = series.simple_returns()?;
    let volatility = Volatility::sample_from_returns(&returns)?;
    let drawdown = Drawdown::max_from_price_series(&series)?;

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
