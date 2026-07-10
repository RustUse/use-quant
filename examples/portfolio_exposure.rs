use use_quant::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _weights = portfolio_weight::WeightSet::from_asset_weights([
        portfolio_weight::AssetWeight::new("AAPL", portfolio_weight::PortfolioWeight::new(0.40)?)?,
        portfolio_weight::AssetWeight::new("MSFT", portfolio_weight::PortfolioWeight::new(0.35)?)?,
        portfolio_weight::AssetWeight::new("CASH", portfolio_weight::PortfolioWeight::new(0.25)?)?,
    ])?;

    let total_weight = 0.40 + 0.35 + 0.25;

    println!("total weight: {total_weight}");

    Ok(())
}
