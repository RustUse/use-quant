use use_quant::{WeightSet, asset_weight, portfolio_weight};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let weights = WeightSet::new([
        asset_weight("AAPL", portfolio_weight(0.40)?)?,
        asset_weight("MSFT", portfolio_weight(0.35)?)?,
        asset_weight("CASH", portfolio_weight(0.25)?)?,
    ])?;

    println!("weight sum: {:.2}", weights.sum());
    println!("gross exposure: {:.2}", weights.gross_exposure());
    println!("net exposure: {:.2}", weights.net_exposure());
    println!("long only: {}", weights.is_long_only());
    println!("fully invested: {}", weights.is_fully_invested());

    assert!(weights.is_long_only());
    assert!(weights.is_fully_invested());
    assert!((weights.gross_exposure() - 1.0).abs() < 1e-12);
    assert!((weights.net_exposure() - 1.0).abs() < 1e-12);

    Ok(())
}
