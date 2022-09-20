use anyhow::Result;

use crate::{
    game::maths::Vector3,
    models::{
        item::Item,
        player::{EnemyPlayer, LocalPlayer, Player, PlayerKind, PlayerType},
    },
};

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
                address: 0,
                name,
                location,
                direction,
                id,
                last_aggressor: None,
                is_aiming: false,
                player_type: if is_local {
                    PlayerType::Local
                } else {
                    PlayerType::Enemy
                },
                kind: if is_local {
                    PlayerKind::Local(LocalPlayer {
                        fps_camera: None,
                        optic_camera: None,
                    })
                } else {
                    PlayerKind::Enemy(EnemyPlayer {})
                },
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
                address: 0,
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
    fn toggle_thermal(&self, thermal_state: &bool) -> Result<bool> {
        Ok(!thermal_state)
    }

    fn update_players(&self, old_players: &[Player]) -> Result<Vec<Player>> {
        let mut new_players = Vec::new();

        for player in old_players {
            let location = Vector3::new(
                player.location.x + rand::random::<f32>() * 10.0,
                player.location.y + rand::random::<f32>() * 10.0,
                player.location.z + rand::random::<f32>() * 10.0,
            );

            let new_player = Player {
                address: player.address,
                name: player.name.clone(),
                location,
                direction: player.direction,
                id: player.id.clone(),
                last_aggressor: player.last_aggressor.clone(),
                is_aiming: player.is_aiming,
                kind: player.kind.clone(),
                player_type: player.player_type.clone(),
            };

            new_players.push(new_player);
        }

        Ok(new_players)
    }

    fn update_items(&self, old_items: &[Item]) -> Result<Vec<Item>> {
        let mut new_items = Vec::new();

        for item in old_items {
            let location = Vector3::new(
                item.location.x + rand::random::<f32>() * 10.0,
                item.location.y + rand::random::<f32>() * 10.0,
                item.location.z + rand::random::<f32>() * 10.0,
            );

            let new_item = Item {
                location,
                ..item.clone()
            };

            new_items.push(new_item);
        }

        Ok(new_items)
    }
}
