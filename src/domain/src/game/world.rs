use bincode::Decode;
use external_memory_lib::Memory;
use anyhow::Result;

use super::unity::find_object_in;

#[repr(C)]
#[derive(Copy, Clone, Debug, Decode)]
pub struct GameObjectManager {
    pub last_tagged_node: usize,
    pub tagged_nodes: usize,
    pub last_main_camera_tagged_node: usize,
    pub main_camera_tagged_nodes: usize,
    pub last_active_node: usize,
    pub active_nodes: usize,
}

pub fn get_world(memory: &Memory) -> Result<(usize, GameObjectManager)> {
    let gom_ptr = memory.read::<usize>(memory.base_address)?;
    let gom = memory.read::<GameObjectManager>(gom_ptr)?;

    let world_ptr = find_object_in(gom.active_nodes, "GameWorld", memory)?;
    let world = memory.read_sequence(world_ptr, [0x30, 0x18, 0x28].to_vec())?;

    Ok((world, gom))
}