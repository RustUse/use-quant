use use_quant::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let start = MarketPrice::new(100.0)?;
    let end = MarketPrice::new(105.0)?;

    let simple_return = SimpleReturn::from_prices(start.value(), end.value())?;
    let log_return = LogReturn::from_prices(start.value(), end.value())?;

    println!("start price: {}", start.value());
    println!("end price: {}", end.value());
    println!("simple return: {:.4}", simple_return.value());
    println!("log return: {:.4}", log_return.value());

    assert!(simple_return.value() > 0.0);
    assert!(log_return.value() > 0.0);

    Ok(())
}
