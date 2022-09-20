use anyhow::Result;
use external_memory_lib::utilities::memory::Memory;

use super::maths::Matrix44;

#[repr(C)]
#[derive(Clone, Debug)]
pub struct InternalCamera {
    pub address: usize,
}

impl InternalCamera {
    pub fn get_matrix(&self, memory: &Memory) -> Result<Matrix44> {
        let matrix = memory.read::<Matrix44>(self.address + 0xDC)?;

        Ok(matrix)
    }

    pub fn get_fov(&self, memory: &Memory) -> Result<f32> {
        let fov = memory.read::<f32>(self.address + 0x15C)?;

        Ok(fov)
    }

    pub fn get_aspect_ratio(&self, memory: &Memory) -> Result<f32> {
        let aspect_ratio = memory.read::<f32>(self.address + 0x4C8)?;

        Ok(aspect_ratio)
    }
}
