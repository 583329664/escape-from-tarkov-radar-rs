use anyhow::Result;
use external_memory_lib::utilities::memory::Memory;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct UnityString {
    p1: [u8; 0x10],
    pub length: i32,
    p2: [u8; 0x14],
    pub data: usize,
}

impl UnityString {
    pub fn read_string(&self, memory: &Memory) -> Result<String> {
        let multiplier = std::mem::size_of::<u16>() as i32;
        let length = self.length * multiplier;
        let string = memory
            .read_string(self.data, length as usize)?
            .replace('\0', "");

        Ok(string)
    }
}
