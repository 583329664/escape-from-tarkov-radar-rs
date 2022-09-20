use std::f32::consts::PI;

use anyhow::{Ok, Result};
use external_memory_lib::utilities::memory::Memory;

use super::{
    maths::Vector3,
    unity::{self},
};

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct InternalPlayer {
    pub address: usize,
}

impl InternalPlayer {
    pub fn is_dead(&self, memory: &Memory) -> Result<bool> {
        let is_dead = memory.read::<bool>(self.address + 0x6D0)?;
        Ok(is_dead)
    }

    pub fn is_local(&self, memory: &Memory) -> Result<bool> {
        let is_local = memory.read::<bool>(self.address + 0x807)?;
        Ok(is_local)
    }

    pub fn get_movement_context(&self, memory: &Memory) -> Result<MovementContext> {
        let ptr = memory.read::<usize>(self.address + 0x40)?;
        let movement_context = MovementContext { address: ptr };

        Ok(movement_context)
    }

    pub fn get_body(&self, memory: &Memory) -> Result<PlayerBody> {
        let ptr = memory.read::<usize>(self.address + 0xA8)?;
        let body = PlayerBody { address: ptr };

        Ok(body)
    }

    pub fn get_procedural_weapon(&self, memory: &Memory) -> Result<ProceduralWeaponAnimation> {
        let ptr = memory.read::<usize>(self.address + 0x198)?;
        let procedural_weapon = ProceduralWeaponAnimation { address: ptr };

        Ok(procedural_weapon)
    }

    pub fn get_last_agressor(&self, memory: &Memory) -> Option<InternalPlayer> {
        let ptr = memory.read::<usize>(self.address + 0x340).ok()?;

        if ptr == 0 {
            return None;
        }

        let last_agressor = InternalPlayer { address: ptr };

        Some(last_agressor)
    }

    pub fn get_profile(&self, memory: &Memory) -> Result<PlayerProfile> {
        let ptr = memory.read::<usize>(self.address + 0x4F0)?;
        let profile = PlayerProfile { address: ptr };

        Ok(profile)
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct PlayerProfile {
    pub address: usize,
}

impl PlayerProfile {
    pub fn get_player_info(&self, memory: &Memory) -> Result<PlayerInfo> {
        let ptr = memory.read::<usize>(self.address + 0x28)?;
        let profile = PlayerInfo { address: ptr };

        Ok(profile)
    }
    pub fn get_id(&self, memory: &Memory) -> Result<String> {
        let ptr = memory.read::<usize>(self.address + 0x18)?;
        let id = memory.read_unity_string(ptr)?;

        Ok(id)
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct PlayerInfo {
    pub address: usize,
}

impl PlayerInfo {
    pub fn get_name(&self, memory: &Memory) -> Result<String> {
        let ptr = memory.read::<usize>(self.address + 0x10)?;
        let side = memory.read::<i32>(self.address + 0x68)?;
        let registration_date = memory.read::<usize>(self.address + 0x6C)?;

        if side == 4 {
            let name = if registration_date == 0 {
                "Bot Scav".to_string()
            } else {
                "Player Scav".to_string()
            };
            return Ok(name);
        }

        let name = memory.read_unity_string(ptr)?;
        Ok(name)
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct MovementContext {
    pub address: usize,
}

impl MovementContext {
    pub fn get_degrees(&self, memory: &Memory) -> Result<f32> {
        let angle = memory.read::<f32>(self.address + 0x22C)?;
        Ok(if angle < 0.0 {
            (360.0 + angle) * PI / 180.0
        } else {
            angle * PI / 180.0
        })
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
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

    pub fn is_aiming(&self, memory: &Memory) -> Result<bool> {
        let breath_reactor = memory.read::<usize>(self.address + 0x28)?;

        let is_aiming = memory.read::<bool>(breath_reactor + 0xA0)?;
        Ok(is_aiming)
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct PlayerBody {
    pub address: usize,
}

impl PlayerBody {
    pub fn get_location(&self, memory: &Memory) -> Result<Vector3> {
        let ptr = memory.read::<usize>(self.address + 0x28)?;
        let values = memory.read::<usize>(ptr + 0x28)?;
        let bone_matrix = memory.read_sequence(values, vec![0x10])?;
        let root_transform = memory.read::<usize>(bone_matrix + (0x20 + 0 * 8))?;

        let location = unity::transform_to_world_space(root_transform, memory)?;

        Ok(location)
    }
}

pub enum ScavType {
    Marksman = 1,
    Assault = 2,
    BossTest = 4,
    BossBully = 8,
    FollowerTest = 16,
    FollowerBully = 32,
    BossKilla = 64,
    BossKojaniy = 128,
    FollowerKojaniy = 256,
    PmcBot = 512,
    CursedAssault = 1024,
    BossGluhar = 2048,
    FollowerGluharAssault = 4096,
    FollowerGluharSecurity = 8192,
    FollowerGluharScout = 16384,
    FollowerGluharSnipe = 32768,
    FollowerSanitar = 65536,
    BossSanitar = 131072,
    Test = 262144,
    AssaultGroup = 524288,
    SectantWarrior = 1048576,
    SectantPriest = 2097152,
    BossTagilla = 4194304,
    FollowerTagilla = 8388608,
}

pub enum Bones {
    HumanBase = 0,
    HumanPelvis = 14,
    HumanLThigh1 = 15,
    HumanLThigh2 = 16,
    HumanLCalf = 17,
    HumanLFoot = 18,
    HumanLToe = 19,
    HumanRThigh1 = 20,
    HumanRThigh2 = 21,
    HumanRCalf = 22,
    HumanRFoot = 23,
    HumanRToe = 24,
    HumanSpine1 = 29,
    HumanSpine2 = 36,
    HumanSpine3 = 37,
    HumanLCollarbone = 89,
    HumanLUpperarm = 90,
    HumanLForearm1 = 91,
    HumanLForearm2 = 92,
    HumanLForearm3 = 93,
    HumanLPalm = 94,
    HumanRCollarbone = 110,
    HumanRUpperarm = 111,
    HumanRForearm1 = 112,
    HumanRForearm2 = 113,
    HumanRForearm3 = 114,
    HumanRPalm = 115,
    HumanNeck = 132,
    HumanHead = 133,
}
