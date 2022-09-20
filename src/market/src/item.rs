use serde_derive::Deserialize;
use serde_derive::Serialize;

pub type Items = Vec<Item>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    pub uid: String,
    pub name: String,
    pub banned_on_flea: bool,
    pub have_market_data: Option<bool>,
    pub tags: Vec<String>,
    pub short_name: String,
    pub price: i64,
    pub base_price: i64,
    pub avg24h_price: i64,
    pub avg7days_price: i64,
    pub trader_name: String,
    pub trader_price: i64,
    pub trader_price_cur: String,
    pub trader_price_rub: i64,
    pub updated: String,
    pub slots: i64,
    pub diff24h: f64,
    pub diff7days: f64,
    pub icon: String,
    pub link: String,
    pub wiki_link: String,
    pub img: String,
    pub img_big: String,
    pub bsg_id: String,
    pub is_functional: bool,
    pub reference: String,
    pub api_key: Option<String>,
}