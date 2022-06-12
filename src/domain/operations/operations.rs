use anyhow::Result;

use crate::domain::{player::Player, item::Item};

pub trait Operations {
    fn get_players(&mut self) -> Result<Vec<Player>>;
    fn toggle_thermal(&mut self) -> Result<()>;
    fn get_items(&mut self) -> Result<Vec<Item>>;
}
