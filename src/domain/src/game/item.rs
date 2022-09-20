use anyhow::Result;
use external_memory_lib::utilities::memory::Memory;

use super::{maths::Vector3, unity::transform_to_world_space};

pub const BAD_ITEMS: [&str; 13] = [
    "body",
    "XXXcap",
    "Ammo_crate_Cap",
    "Grenade_box_Door",
    "Medical_Door",
    "Toolbox_Door",
    "card_file_box",
    "cover_",
    "lootable",
    "scontainer_Blue_Barrel_Base_Cap",
    "scontainer_wood_CAP",
    "suitcase_plastic_lootable_open",
    "weapon_box_cover",
];

pub struct InternalItem {
    pub address: usize,
    pub template_address: usize,
    pub game_object_address: usize,
}

impl InternalItem {
    pub fn get_rarity(&self, memory: &Memory) -> Result<ItemRarity> {
        let ptr = memory.read::<usize>(self.template_address + 0xC4)?;
        let rarity_as_int = memory.read::<i32>(ptr)?;
        let rarity = match rarity_as_int {
            0 => ItemRarity::NotExist,
            1 => ItemRarity::Common,
            2 => ItemRarity::Rare,
            3 => ItemRarity::SuperRare,
            _ => return Err(anyhow::anyhow!("Unknown rarity: {}", rarity_as_int)),
        };

        Ok(rarity)
    }

    pub fn get_id(&self, memory: &Memory) -> Result<String> {
        let ptr = memory.read::<usize>(self.template_address + 0x50)?;
        let id = memory.read_unity_string(ptr)?;

        Ok(id)
    }

    pub fn get_name(&self, memory: &Memory) -> Result<String> {
        let ptr = memory.read::<usize>(self.game_object_address + 0x60)?;
        let name = memory.read_string(ptr, 128)?;

        Ok(name)
    }

    pub fn get_location(&self, memory: &Memory) -> Result<Vector3> {
        let ptr_1 = memory.read::<usize>(self.game_object_address + 0x30)?;
        let ptr_2 = memory.read::<usize>(ptr_1 + 0x8)?;
        let container = memory.read::<usize>(ptr_2 + 0x28)?;
        let location = transform_to_world_space(container, memory)?;

        Ok(location)
    }
}

pub enum ItemRarity {
    NotExist = 0,
    Common = 1,
    Rare = 2,
    SuperRare = 3,
}
