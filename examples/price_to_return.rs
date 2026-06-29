use use_quant::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let start = market_price::MarketPrice::new(100.0)?;
    let end = market_price::MarketPrice::new(105.0)?;

    let simple_return = return_::SimpleReturn::from_prices(start.value(), end.value())?;
    let log_return = return_::LogReturn::from_prices(start.value(), end.value())?;

    println!("simple return: {}", simple_return.value());
    println!("log return: {}", log_return.value());

    Ok(())
}
