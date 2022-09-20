use anyhow::Result;
use external_memory_lib::utilities::memory::Memory;

use crate::models::world::WorldState;

use super::unity::find_object;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct GameObjectManager {
    pub last_tagged_node: usize,
    pub tagged_nodes: usize,
    pub last_main_camera_tagged_node: usize,
    pub main_camera_tagged_nodes: usize,
    pub last_active_node: usize,
    pub active_nodes: usize,
}

pub fn get_world_state(memory: &Memory) -> Result<WorldState> {
    let gom_ptr = memory.read::<usize>(memory.base_address)?;
    let gom = memory.read::<GameObjectManager>(gom_ptr)?;

    let world_ptr = find_object(gom.active_nodes, "GameWorld", memory)?;
    let world = memory.read_sequence(world_ptr, [0x30, 0x18, 0x28].to_vec())?;

    let camera_ptr = find_object(gom.main_camera_tagged_nodes, "FPS Camera", &memory)?;

    Ok(WorldState {
        world_address: world,
        gom,
        fps_camera_address: camera_ptr,
    })
}
