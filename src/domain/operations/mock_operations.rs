use anyhow::Result;

use crate::{domain::{player::Player, item::Item}, game::maths::Vector3};

use super::operations::Operations;

pub struct MockOperations {
    players: Vec<Player>,
    items: Vec<Item>,
}

impl MockOperations {
    pub fn new() -> Self {
        let mut players = Vec::new();
        let mut items = Vec::new();

        for i in 0..10 {
            let name = "Bot ".to_string() + &i.to_string();
            let location = Vector3::new(
                rand::random::<f32>() * 1000.0,
                rand::random::<f32>() * 10.0,
                rand::random::<f32>() * 1000.0,
            );
            let direction = rand::random::<f32>();
            let id = "fake ID".to_string();
            let is_local = i == 0;

            let player = Player {
                name,
                location,
                direction,
                id,
                is_local,
                is_dead: false,
                last_aggressor: None,
            };

            players.push(player);
        }

        for i in 0..10 {
            let name = "Item ".to_string() + &i.to_string();
            let location = Vector3::new(
                rand::random::<f32>() * 1000.0,
                rand::random::<f32>() * 10.0,
                rand::random::<f32>() * 1000.0,
            );
            let id = "fake ID".to_string();

            let item = Item {
                name,
                id,
                location,
            };

            items.push(item);
        }

        MockOperations { players, items }
    }
}

impl Operations for MockOperations {
    fn get_players(&mut self) -> Result<Vec<Player>> {
        Ok(self.players.clone())
    }

    fn toggle_thermal(&mut self) -> Result<()> {
        Ok(())
    }

    fn get_items(&mut self) -> Result<Vec<Item>> {
        Ok(self.items.clone())
    }
}
