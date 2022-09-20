use std::sync::Arc;

use anyhow::{bail, Result};
use external_memory_lib::utilities::memory::Memory;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

use crate::{
    game::{
        item::{InternalItem, BAD_ITEMS},
        mappers,
        unity::get_components,
        world::get_world_state,
    },
    models::{item::Item, player::Player, world::WorldState},
};

use super::operations::Operations;

pub struct MemoryOperations {
    memory: Arc<Memory>,
    game_state: WorldState,
}

impl MemoryOperations {
    pub fn new(memory: Arc<Memory>) -> Result<MemoryOperations> {
        let game_state = get_world_state(&memory)?;
        let operations = MemoryOperations { memory, game_state };

        Ok(operations)
    }
}

impl Operations for MemoryOperations {
    fn toggle_thermal(&self, thermal_state: &bool) -> Result<bool> {
        let components = get_components(self.game_state.fps_camera_address, &self.memory)?;

        let thermal = components
            .iter()
            .find(|c| c.name.to_ascii_lowercase().contains("thermalvision"))
            .ok_or_else(|| anyhow::anyhow!("No thermal found."))?
            .address;

        let pixel_opts = self.memory.read::<usize>(thermal + 0x38)?;
        self.memory.write(pixel_opts + 0x20, 1)?;
        self.memory.write(pixel_opts + 0x28, 0.0)?;

        let fps_opts = self.memory.read::<usize>(thermal + 0x20)?;
        self.memory.write(fps_opts + 0x14, 144)?;
        self.memory.write(thermal + 0xE0, thermal_state)?;

        Ok(!thermal_state)
    }

    fn update_players(&self, old_players: &[Player]) -> Result<Vec<Player>> {
        let player_list = self
            .memory
            .read::<usize>(self.game_state.world_address + 0x88)?;
        let player_list_length = self.memory.read::<i32>(player_list + 0x18)?;
        let player_list_base = self.memory.read::<usize>(player_list + 0x10)? + 0x20;
        let cached_list_length = old_players.len();

        match player_list_length as usize {
            len if len == cached_list_length => old_players
                .into_par_iter()
                .map(|old_player| {
                    mappers::internal_player_to_player(
                        old_player.address,
                        &self.game_state,
                        &self.memory,
                    )
                })
                .collect::<Result<Vec<_>>>(),
            _ => (0..player_list_length)
                .into_par_iter()
                .map(|i| {
                    let player_ptr = self
                        .memory
                        .read::<usize>(player_list_base + (i * 0x8) as usize)?;
                    mappers::internal_player_to_player(player_ptr, &self.game_state, &self.memory)
                })
                .collect::<Result<Vec<_>>>(),
        }
    }

    fn update_items(&self, old_items: &[Item]) -> Result<Vec<Item>> {
        let item_list = self
            .memory
            .read::<usize>(self.game_state.world_address + 0x68)?;
        let item_list_length = self.memory.read::<i32>(item_list + 0x18)?;
        let item_list_base = self.memory.read::<usize>(item_list + 0x10)? + 0x20;

        (0..item_list_length)
            .into_par_iter()
            .map(|i| -> Result<Item> {
                let entity_address = self
                    .memory
                    .read::<usize>(item_list_base + (i * 0x8) as usize)?;
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
                    address: item_address,
                    name,
                    location,
                    id,
                };

                if BAD_ITEMS
                    .map(|x| x.to_ascii_lowercase())
                    .contains(&item.name)
                {
                    bail!("Found item that is not an item: {}", item.name);
                };

                Ok(item)
            })
            .collect::<Result<Vec<_>>>()
    }
}
