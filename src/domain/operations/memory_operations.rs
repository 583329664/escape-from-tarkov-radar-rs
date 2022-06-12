use std::{sync::Arc, collections::HashMap};

use anyhow::{Result, bail};
use external_memory_lib::Memory;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

use crate::{
    domain::{item::Item, player::Player},
    game::{
        player::InternalPlayer,
        unity::{find_object_in, get_components},
        world::{get_world, GameObjectManager}, item::{InternalItem, BAD_ITEMS}, maths::Vector3,
    },
};

use super::operations::Operations;

const LOCAL_ID: &str = "4223581";

pub struct GameState {
    world: usize,
    game_object_manager: GameObjectManager,
}

pub struct LocalState {
    pub players: HashMap<usize, Player>,
    pub items: HashMap<usize, Item>,
    pub is_thermal_enabled: bool,
}

pub struct MemoryOperations {
    memory: Arc<Memory>,
    game_state: GameState,
    local_state: LocalState,
}

impl MemoryOperations {
    pub fn new(memory: Arc<Memory>) -> Result<MemoryOperations> {
        let world_and_gom = get_world(&memory)?;

        let operations = MemoryOperations {
            memory,
            game_state: GameState {
                world: world_and_gom.0,
                game_object_manager: world_and_gom.1,
            },
            local_state: LocalState {
                players: HashMap::new(),
                items: HashMap::new(),
                is_thermal_enabled: false
            },
        };

        Ok(operations)
    }
}

impl Operations for MemoryOperations {
    fn toggle_thermal(&mut self) -> Result<()> {
        let camera = find_object_in(
            self.game_state.game_object_manager.main_camera_tagged_nodes,
            "FPS Camera",
            &self.memory,
        )?;

        let components = get_components(camera, &self.memory)?;

        let thermal = components
            .iter()
            .find(|c| c.name.to_ascii_lowercase().contains("thermalvision"))
            .ok_or_else(|| anyhow::anyhow!("No thermal found."))?
            .address;

        let pixel_opts = self.memory.read::<usize>(thermal + 0x38)?;
        self.memory.write_by_type(pixel_opts + 0x20, 1)?;
        self.memory.write_by_type(pixel_opts + 0x28, 0.0)?;

        let fps_opts = self.memory.read::<usize>(thermal + 0x20)?;
        self.memory.write_by_type(fps_opts + 0x14, 144)?;

        // apparently multiple writes is required to change the thermal state consistently
        self.memory.write_by_type(thermal + 0xE0, self.local_state.is_thermal_enabled)?;
        self.memory.write_by_type(thermal + 0xE0, self.local_state.is_thermal_enabled)?;
        self.memory.write_by_type(thermal + 0xE0, self.local_state.is_thermal_enabled)?;
        self.memory.write_by_type(thermal + 0xE0, self.local_state.is_thermal_enabled)?;
        
        self.local_state.is_thermal_enabled = !self.local_state.is_thermal_enabled;

        Ok(())
    }

    fn get_players(&mut self) -> Result<Vec<Player>> {
        let player_list = self.memory.read::<usize>(self.game_state.world + 0x88)?;
        let player_list_length = self.memory.read::<i32>(player_list + 0x18)?;
        let player_list_base = self.memory.read::<usize>(player_list + 0x10)? + 0x20;

        let players: Vec<(usize, Player)> = match player_list_length as usize {
            len if len == self.local_state.players.len() => self.local_state.players
                .clone()
                .into_par_iter()
                .map(|(player_ptr, cached_player)| -> Result<(usize, Player)> {
                    let player = InternalPlayer {
                        address: player_ptr,
                    };

                    let player_body = player.get_body(&self.memory)?;
                    let player_bones = player_body.get_player_bones(&self.memory)?;
                    let movement_context = player.get_movement_context(&self.memory)?;

                    let location = player_bones.get_location(&self.memory)?;
                    let direction = movement_context.get_degrees(&self.memory)?;
                    let last_aggressor = player.get_last_aggressor(&self.memory)?;
                    let is_dead = player.get_is_dead(&self.memory)?;

                    if cached_player.is_local {
                        let weapon_animator = player.get_procedural_weapon(&self.memory)?;
                        weapon_animator.zero_out_recoil(&self.memory)?;
                    }

                    let updated_player = Player { location: Vector3::flip(location), direction, last_aggressor, is_dead, ..cached_player };
                    Ok((player_ptr, updated_player))
                })
                .collect::<Result<Vec<_>>>()?,
            _ => (0..player_list_length)
                .into_par_iter()
                .map(|i| -> Result<(usize, Player)> {
                    let player_ptr = self
                        .memory
                        .read::<usize>(player_list_base + (i * 0x8) as usize)?;

                    let player = InternalPlayer {
                        address: player_ptr,
                    };

                    let movement_context = player.get_movement_context(&self.memory)?;
                    let player_profile = player.get_profile(&self.memory)?;
                    let player_info = player_profile.get_info(&self.memory)?;
                    let player_body = player.get_body(&self.memory)?;
                    let player_bones = player_body.get_player_bones(&self.memory)?;

                    let name = player_info.get_name(&self.memory)?;
                    let id = player_profile.get_id(&self.memory)?;
                    let direction = movement_context.get_degrees(&self.memory)?;
                    let location = player_bones.get_location(&self.memory)?;
                    let last_aggressor = player.get_last_aggressor(&self.memory)?;
                    let is_dead = player.get_is_dead(&self.memory)?;

                    if id == LOCAL_ID {
                        let weapon_animator = player.get_procedural_weapon(&self.memory)?;
                        weapon_animator.zero_out_recoil(&self.memory)?;
                    }

                    let player = Player {
                        name,
                        id: id.clone(),
                        direction,
                        location: Vector3::flip(location),
                        last_aggressor,
                        is_dead,
                        is_local: id == LOCAL_ID
                    };

                    Ok((player_ptr, player))
                })
                .collect::<Result<Vec<_>>>()?,
            };

        players.into_iter().for_each(|(player_ptr, player)| {
            self.local_state.players.insert(player_ptr, player);
        });

        Ok(self.local_state.players.values().cloned().collect())
    }

    fn get_items(&mut self) -> Result<Vec<Item>> {
        let item_list = self.memory.read::<usize>(self.game_state.world + 0x68)?;
        let item_list_length = self.memory.read::<i32>(item_list + 0x18)?;
        let item_list_base = self.memory.read::<usize>(item_list + 0x10)? + 0x20;

        let items = (0..item_list_length)
            .into_par_iter()
            .map(|i| -> Result<(usize, Item)> {
                let entity_address = self.memory.read::<usize>(item_list_base + (i * 0x8) as usize)?;
                let unknown_ptr = self.memory.read::<usize>(entity_address + 0x10)?;
                let interactive_class = self.memory.read::<usize>(unknown_ptr + 0x28)?;
                let base_object = self.memory.read::<usize>(interactive_class + 0x10)?;
                let game_object_address = self.memory.read::<usize>(base_object + 0x30)?;
                let item_address = self.memory.read::<usize>(interactive_class + 0x50)?;
                let item_template = self.memory.read::<usize>(item_address + 0x40)?;

                let item = InternalItem {
                    address: item_address,
                    template_address: item_template,
                    game_object_address,
                };

                let location = item.get_location(&self.memory)?;
                let id = item.get_id(&self.memory)?;
                let name = match id.as_str() {
                    "55d7217a4bdc2d86028b456d" => "Corpse".to_string(),
                    _ => item.get_name(&self.memory)?,
                };

                let item = Item {
                    name,
                    location,
                    id,
                };

                if BAD_ITEMS.map(|x| x.to_ascii_lowercase()).contains(&item.name) {
                    bail!("Found item that is not an item: {}", item.name);
                };

                Ok((item_address, item))
            })
            .collect::<Result<Vec<_>>>()?;

        items.into_iter().for_each(|(item_address, item)| {
            self.local_state.items.insert(item_address, item);
        });
        
        Ok(self.local_state.items.values().cloned().collect())
    }
}
