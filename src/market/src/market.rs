use anyhow::Result;

use crate::item::Items;

pub async fn download_items() -> Result<Items> {
    let market_items: Items = reqwest::get("https://market_master.filter-editor.com/data/marketData_en.json")
        .await?
        .json::<Items>()
        .await?
        .into_iter()
        .filter(|item| item.avg24h_price > 50000 || item.trader_price > 50000)
        .collect();
    
    Ok(market_items)
}
