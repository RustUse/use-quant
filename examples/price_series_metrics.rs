use use_quant::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let series = price_series::PriceSeries::from_points([
        price_series::PricePoint::new(market_price::MarketPrice::new(100.0)?),
        price_series::PricePoint::new(market_price::MarketPrice::new(102.0)?),
        price_series::PricePoint::new(market_price::MarketPrice::new(101.0)?),
        price_series::PricePoint::new(market_price::MarketPrice::new(105.0)?),
        price_series::PricePoint::new(market_price::MarketPrice::new(103.0)?),
        price_series::PricePoint::new(market_price::MarketPrice::new(108.0)?),
    ]);

    let return_values = series
        .adjacent_simple_returns()?
        .into_iter()
        .map(return_::SimpleReturn::value)
        .collect::<Vec<_>>();

    let prices = series
        .iter()
        .map(|point| point.price().value())
        .collect::<Vec<_>>();

    let volatility = volatility::Volatility::sample_from_returns(&return_values)?;
    let drawdown = drawdown::Drawdown::maximum_from_values(&prices)?;

    println!("volatility: {}", volatility.value());
    println!("maximum drawdown: {}", drawdown.value());

    Ok(())
}
