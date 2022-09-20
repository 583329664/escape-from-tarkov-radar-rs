use crate::game::{world::GameObjectManager};

#[derive(Clone, Debug)]
pub struct WorldState {
    pub gom: GameObjectManager,
    pub world_address: usize,
    pub fps_camera_address: usize,
}
