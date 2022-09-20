use std::arch::x86_64::{
    __m128, __m128i, _mm_add_ps, _mm_castps_si128, _mm_castsi128_ps, _mm_mul_ps, _mm_set_ps,
    _mm_shuffle_epi32, _mm_sub_ps,
};

use anyhow::{bail, Result};
use external_memory_lib::utilities::memory::Memory;

use super::maths::{Matrix34, Vector3};

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct BaseObject {
    pub previous_object_link: usize,
    pub next_object_link: usize,
    pub actual_object: usize,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct ComponentArray {
    pub array_base: usize,
    pub mem_label_id: usize,
    pub size: usize,
    pub capacity: usize,
}

pub struct Component {
    pub name: String,
    pub namespace: String,
    pub address: usize,
}

pub fn find_object(first_object: usize, object_name: &str, memory: &Memory) -> Result<usize> {
    let mut current_object = memory.read::<BaseObject>(first_object)?;

    if current_object.actual_object == 0 {
        bail!("Incorrect list used in find object loop.");
    }

    loop {
        let address = (current_object.actual_object + 0x60) as usize;
        let entity_name_ptr = memory.read::<usize>(address)?;
        let entity_name = memory.read_string(entity_name_ptr, object_name.len());

        if entity_name.is_ok() && entity_name.unwrap().eq_ignore_ascii_case(object_name) {
            return Ok(current_object.actual_object);
        }

        current_object = memory.read::<BaseObject>(current_object.next_object_link)?;
    }
}

pub fn get_components(game_object: usize, memory: &Memory) -> Result<Vec<Component>> {
    let mut ret: Vec<Component> = vec![];
    let component_array = memory.read::<ComponentArray>(game_object + 0x30)?;

    for i in 0..component_array.size {
        let component_address =
            memory.read::<usize>(component_array.array_base + i * 0x10 + 0x8)?;
        let component_class_address = memory.read::<usize>(component_address + 0x28)?;

        let mono_class = memory.read_sequence(component_class_address, vec![0x0, 0x0])?;
        let name_ptr = memory.read::<usize>(mono_class + 0x48)?;
        let namespace_ptr = memory.read::<usize>(mono_class + 0x50)?;

        let name = memory.read_string(name_ptr, 128)?;
        let namespace = memory.read_string(namespace_ptr, 128)?;

        ret.push(Component {
            name,
            namespace,
            address: component_class_address,
        });
    }

    Ok(ret)
}

pub fn transform_to_world_space(transform: usize, memory: &Memory) -> Result<Vector3> {
    unsafe {
        let transform_internal = memory.read::<usize>(transform + 0x10)?;

        let mul_vec0 = _mm_set_ps(0.000, -2.000, 2.000, -2.000);
        let mul_vec1 = _mm_set_ps(0.000, -2.000, -2.000, 2.000);
        let mul_vec2 = _mm_set_ps(0.000, 2.000, -2.000, -2.000);

        let matrix = memory.read::<usize>(transform_internal + 0x38)?;
        let index = memory.read::<i32>(transform_internal + 0x40)?;

        let matrix_base = memory.read::<usize>(matrix + 0x18)?;
        let dep_table = memory.read::<usize>(matrix + 0x20)?;

        let size_matricies_buf =
            std::mem::size_of::<Matrix34>() * index as usize + std::mem::size_of::<Matrix34>();
        let size_indices_buf =
            std::mem::size_of::<i32>() * index as usize + std::mem::size_of::<i32>();

        if size_indices_buf > 1000000 || size_matricies_buf > 1000000 {
            bail!("Failed to read transform data.");
        };

        let indices_buffer = memory.read_bytes(dep_table, size_indices_buf)?;
        let matricies_buffer = memory.read_bytes(matrix_base, size_matricies_buf)?;

        let indices_buffer = indices_buffer.align_to::<i32>().1;
        let matricies_buffer = matricies_buffer.align_to::<[__m128; 3]>().1;

        let mut result = matricies_buffer[index as usize][0];
        let mut transform_index = indices_buffer[index as usize];

        while transform_index >= 0 {
            let usize_index = transform_index as usize;

            let matrix = matricies_buffer[usize_index];
            let matrix0_m128 = matrix[0];
            let matrix1_m128i = *(&matrix[1] as *const __m128 as *const __m128i);
            let matrix2_m128 = matrix[2];

            let xxxx = _mm_castsi128_ps(_mm_shuffle_epi32(matrix1_m128i, 0x00));
            let yyyy = _mm_castsi128_ps(_mm_shuffle_epi32(matrix1_m128i, 0x55));
            let zwxy = _mm_castsi128_ps(_mm_shuffle_epi32(matrix1_m128i, 0x8E));
            let wzyw = _mm_castsi128_ps(_mm_shuffle_epi32(matrix1_m128i, 0xDB));
            let zzzz = _mm_castsi128_ps(_mm_shuffle_epi32(matrix1_m128i, 0xAA));
            let yxwy = _mm_castsi128_ps(_mm_shuffle_epi32(matrix1_m128i, 0x71));
            let tmp7 = _mm_mul_ps(matrix2_m128, result);

            result = _mm_add_ps(
                _mm_add_ps(
                    _mm_add_ps(
                        _mm_mul_ps(
                            _mm_sub_ps(
                                _mm_mul_ps(_mm_mul_ps(xxxx, mul_vec1), zwxy),
                                _mm_mul_ps(_mm_mul_ps(yyyy, mul_vec2), wzyw),
                            ),
                            _mm_castsi128_ps(_mm_shuffle_epi32(_mm_castps_si128(tmp7), 0xAA)),
                        ),
                        _mm_mul_ps(
                            _mm_sub_ps(
                                _mm_mul_ps(_mm_mul_ps(zzzz, mul_vec2), wzyw),
                                _mm_mul_ps(_mm_mul_ps(xxxx, mul_vec0), yxwy),
                            ),
                            _mm_castsi128_ps(_mm_shuffle_epi32(_mm_castps_si128(tmp7), 0x55)),
                        ),
                    ),
                    _mm_add_ps(
                        _mm_mul_ps(
                            _mm_sub_ps(
                                _mm_mul_ps(_mm_mul_ps(yyyy, mul_vec0), yxwy),
                                _mm_mul_ps(_mm_mul_ps(zzzz, mul_vec1), zwxy),
                            ),
                            _mm_castsi128_ps(_mm_shuffle_epi32(_mm_castps_si128(tmp7), 0x00)),
                        ),
                        tmp7,
                    ),
                ),
                matrix0_m128,
            );

            transform_index = indices_buffer[usize_index];
        }

        Ok(*(&result as *const __m128 as *const Vector3))
    }
}
