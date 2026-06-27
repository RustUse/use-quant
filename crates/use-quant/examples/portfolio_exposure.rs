use use_quant::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let weights = WeightSet::from_asset_weights([
        AssetWeight::new("AAPL", PortfolioWeight::new(0.40)?)?,
        AssetWeight::new("MSFT", PortfolioWeight::new(0.35)?)?,
        AssetWeight::new("CASH", PortfolioWeight::new(0.25)?)?,
    ])?;

    println!("weight sum: {:.2}", weights.sum());

    assert!((weights.sum() - 1.0).abs() < 1e-12);
    assert!(weights.is_approximately_fully_invested(1e-12)?);

    Ok(())
}
