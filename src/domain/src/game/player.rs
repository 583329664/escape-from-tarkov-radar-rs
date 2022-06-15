use std::f32::consts::PI;

use anyhow::Result;
use external_memory_lib::Memory;

use super::{maths::Vector3, unity::transform_to_world_space};

#[repr(C)]
pub struct InternalPlayer {
    pub address: usize,
}

impl InternalPlayer {
    pub fn get_movement_context(&self, memory: &Memory) -> Result<MovementContext> {
        let ptr = memory.read::<usize>(self.address + 0x40)?;
        let movement_context = MovementContext { address: ptr };

        Ok(movement_context)
    }

    pub fn get_procedural_weapon(&self, memory: &Memory) -> Result<ProceduralWeaponAnimation> {
        let ptr = memory.read::<usize>(self.address + 0x198)?;
        let procedural_weapon = ProceduralWeaponAnimation { address: ptr };

        Ok(procedural_weapon)
    }

    pub fn get_profile(&self, memory: &Memory) -> Result<PlayerProfile> {
        let ptr = memory.read::<usize>(self.address + 0x4E0)?;
        let profile = PlayerProfile { address: ptr };

        Ok(profile)
    }

    pub fn get_body(&self, memory: &Memory) -> Result<PlayerBody> {
        let ptr = memory.read::<usize>(self.address + 0xA8)?;
        let body = PlayerBody { address: ptr };

        Ok(body)
    }

    pub fn get_last_aggressor(&self, memory: &Memory) -> Result<Option<String>> {
        let ptr = memory.read::<usize>(self.address + 0x340)?;
        let player = InternalPlayer { address: ptr };
        let profile = player.get_profile(memory);

        if profile.is_err() {
            return Ok(None);
        }

        let info = profile.unwrap().get_info(memory)?;
        let name = info.get_name(memory)?;

        Ok(Some(name))
    }

    pub fn get_is_dead(&self, memory: &Memory) -> Result<bool> {
        let is_dead = memory.read::<bool>(self.address + 0x6D0)?;

        Ok(is_dead)
    }
}

#[repr(C)]
pub struct PlayerProfile {
    pub address: usize,
}

impl PlayerProfile {
    pub fn get_info(&self, memory: &Memory) -> Result<PlayerInfo> {
        let ptr = memory.read::<usize>(self.address + 0x28)?;
        let info = PlayerInfo { address: ptr };

        Ok(info)
    }

    pub fn get_id(&self, memory: &Memory) -> Result<String> {
        let ptr = memory.read::<usize>(self.address + 0x18)?;
        let id = memory.read_unity_string(ptr)?;

        Ok(id)
    }
}

#[repr(C)]
pub struct PlayerInfo {
    pub address: usize,
}

impl PlayerInfo {
    pub fn get_name(&self, memory: &Memory) -> Result<String> {
        let ptr = memory.read::<usize>(self.address + 0x10)?;
        let side = memory.read::<i32>(self.address + 0x60)?;
        let registration_date = memory.read::<usize>(self.address + 0x64)?;

        if side == 4 {
            let name = if registration_date == 0 { "Bot Scav".to_string() } else { "Player Scav".to_string() };
            return Ok(name);
        }

        let name = memory.read_unity_string(ptr)?;
        Ok(name)
    }
}

#[repr(C)]
pub struct MovementContext {
    pub address: usize,
}

impl MovementContext {
    pub fn get_degrees(&self, memory: &Memory) -> Result<f32> {
        let angle = memory.read::<f32>(self.address + 0x22C)?;
        Ok(if angle < 0.0 { (360.0 + angle) * PI/180.0 } else { angle * PI/180.0 })
    }
}

pub struct ProceduralWeaponAnimation {
    pub address: usize,
}

impl ProceduralWeaponAnimation {
    pub fn zero_out_recoil(&self, memory: &Memory) -> Result<()> {
        let empty_byte_slice = vec![0_u8; 0x0];

        let breath_reactor = memory.read::<usize>(self.address + 0x28)?;
        memory.write_ptr(breath_reactor + 0xA4, empty_byte_slice.as_ptr().addr());

        let walk_reactor = memory.read::<usize>(self.address + 0x30)?;
        memory.write_ptr(walk_reactor + 0x40, empty_byte_slice.as_ptr().addr());
        memory.write_ptr(walk_reactor + 0x44, empty_byte_slice.as_ptr().addr());

        let motion_reactor = memory.read::<usize>(self.address + 0x38)?;
        memory.write_ptr(motion_reactor + 0xD0, empty_byte_slice.as_ptr().addr());

        let shoot_reactor = memory.read::<usize>(self.address + 0x48)?;
        memory.write_ptr(shoot_reactor + 0x40, empty_byte_slice.as_ptr().addr());
        memory.write_ptr(shoot_reactor + 0x48, empty_byte_slice.as_ptr().addr());
    
        Ok(())
    }
}

pub struct PlayerBody {
    pub address: usize,
}

impl PlayerBody {
    pub fn get_player_bones(&self, memory: &Memory) -> Result<PlayerBones> {
        let ptr = memory.read::<usize>(self.address + 0x20)?;
        let player_bones = PlayerBones { address: ptr };

        Ok(player_bones)
    }
}

pub struct PlayerBones {
    pub address: usize,
}

impl PlayerBones {
    pub fn get_location(&self, memory: &Memory) -> Result<Vector3> {
        let head_transform_ptr = memory.read::<usize>(self.address + 0xD8)?;
        let head_trasnform = memory.read::<usize>(head_transform_ptr + 0x10)?;
        let position = transform_to_world_space(head_trasnform, memory)?;

        Ok(position)
    }
}
