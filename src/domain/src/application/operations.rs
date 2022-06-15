use anyhow::Result;

use crate::models::{player::Player, item::Item};

pub trait Operations {
    fn toggle_thermal(&self, thermal_state: &bool) -> Result<bool>;
    fn update_players(&self, old_players: &[Player]) -> Result<Vec<Player>>;
    fn update_items(&self, old_items: &[Item]) -> Result<Vec<Item>>;
}
