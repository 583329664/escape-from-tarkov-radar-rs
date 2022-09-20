use anyhow::{Ok, Result};
use external_memory_lib::utilities::memory::Memory;

use crate::game::{
    camera::InternalCamera, maths::Vector3, player::ProceduralWeaponAnimation, unity,
};

use super::{
    camera::{Camera, CameraType},
    world::WorldState,
};

#[derive(Clone, Debug, PartialEq)]
pub struct LocalPlayer {
    pub fps_camera: Option<Camera>,
    pub optic_camera: Option<Camera>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct EnemyPlayer {}

#[derive(Clone, Debug, PartialEq)]
pub struct AllyPlayer {}

#[derive(Clone, Debug, PartialEq)]
pub struct ScavPlayer {}

#[derive(Clone, Debug, PartialEq)]
pub struct DeadPlayer {}

#[derive(Clone, Debug, PartialEq)]
pub struct Player {
    pub address: usize,
    pub name: String,
    pub id: String,
    pub location: Vector3,
    pub direction: f32,
    pub last_aggressor: Option<String>,
    pub is_aiming: bool,
    pub kind: PlayerKind,
    pub player_type: PlayerType,
}

#[derive(Clone, Debug, PartialEq)]
pub enum PlayerKind {
    Local(LocalPlayer),
    Enemy(EnemyPlayer),
    Ally(AllyPlayer),
    Scav(ScavPlayer),
    Dead(DeadPlayer),
}

impl Player {
    pub fn new_enemy(
        address: usize,
        name: String,
        id: String,
        location: Vector3,
        direction: f32,
        last_aggressor: Option<String>,
        is_aiming: bool,
    ) -> Self {
        Self {
            address,
            name,
            id,
            location,
            direction,
            last_aggressor,
            is_aiming,
            kind: PlayerKind::Enemy(EnemyPlayer {}),
            player_type: PlayerType::Enemy,
        }
    }

    pub fn new_ally(
        address: usize,
        name: String,
        id: String,
        location: Vector3,
        direction: f32,
        last_aggressor: Option<String>,
        is_aiming: bool,
    ) -> Self {
        Self {
            address,
            name,
            id,
            location,
            direction,
            last_aggressor,
            is_aiming,
            kind: PlayerKind::Ally(AllyPlayer {}),
            player_type: PlayerType::Ally,
        }
    }

    pub fn new_scav(
        address: usize,
        name: String,
        id: String,
        location: Vector3,
        direction: f32,
        last_aggressor: Option<String>,
        is_aiming: bool,
    ) -> Self {
        Self {
            address,
            name,
            id,
            location,
            direction,
            last_aggressor,
            is_aiming,
            kind: PlayerKind::Scav(ScavPlayer {}),
            player_type: PlayerType::Scav,
        }
    }

    pub fn new_dead(location: Vector3) -> Self {
        Self {
            address: 0,
            name: "".to_string(),
            id: "".to_string(),
            location: location,
            direction: 0.0,
            last_aggressor: None,
            kind: PlayerKind::Dead(DeadPlayer {}),
            is_aiming: false,
            player_type: PlayerType::Dead,
        }
    }

    pub fn new_local(
        address: usize,
        name: String,
        id: String,
        location: Vector3,
        direction: f32,
        last_aggressor: Option<String>,
        weapon_animator: ProceduralWeaponAnimation,
        is_aiming: bool,
        game_state: &WorldState,
        memory: &Memory,
    ) -> Result<Self> {
        let mut player = Self {
            address,
            name,
            id,
            location,
            direction,
            last_aggressor,
            is_aiming,
            player_type: PlayerType::Local,
            kind: PlayerKind::Local(LocalPlayer {
                fps_camera: None,
                optic_camera: None,
            }),
        };

        weapon_animator.zero_out_recoil(memory)?;

        let updated_camera_leadup =
            memory.read_sequence(game_state.fps_camera_address, vec![0x30, 0x18])?;
        let updated_camera = InternalCamera {
            address: updated_camera_leadup,
        };
        let fov = updated_camera.get_fov(memory)?;
        let matrix = updated_camera.get_matrix(memory)?;
        let aspect_ratio = updated_camera.get_aspect_ratio(memory)?;
        let camera = Some(Camera {
            fov,
            matrix,
            aspect_ratio,
            camera_type: CameraType::FpsCamera,
        });

        player.kind = PlayerKind::Local(LocalPlayer {
            fps_camera: camera.clone(),
            optic_camera: None,
        });

        if is_aiming {
            let optic_camera_address = unity::find_object(
                game_state.gom.tagged_nodes,
                "BaseOpticCamera(Clone)",
                memory,
            )?;
            let optic_camera_leadup =
                memory.read_sequence(optic_camera_address, vec![0x30, 0x18])?;
            let optic_camera = InternalCamera {
                address: optic_camera_leadup,
            };

            let fov = optic_camera.get_fov(memory)?;
            let matrix = optic_camera.get_matrix(memory)?;
            let aspect_ratio = optic_camera.get_aspect_ratio(memory)?;
            let optic_camera = Some(Camera {
                fov,
                matrix,
                aspect_ratio,
                camera_type: CameraType::OpticCamera,
            });

            player.kind = PlayerKind::Local(LocalPlayer {
                fps_camera: camera.clone(),
                optic_camera: optic_camera,
            });
        }

        Ok(player)
    }

    pub fn as_local(&self) -> Option<&LocalPlayer> {
        match &self.kind {
            PlayerKind::Local(local_player) => Some(local_player),
            _ => None,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum PlayerType {
    Enemy,
    Ally,
    Scav,
    Dead,
    Local,
}
