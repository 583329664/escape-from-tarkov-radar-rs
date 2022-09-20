use anyhow::{anyhow, Result};
use external_memory_lib::utilities::memory::Memory;

use crate::models::{player::Player, world::WorldState};

use super::player::InternalPlayer;

pub fn internal_player_to_player(
    player_ptr: usize,
    game_state: &WorldState,
    memory: &Memory,
) -> Result<Player> {
    let internal_player = InternalPlayer {
        address: player_ptr,
    };

    let movement_context = internal_player.get_movement_context(memory)?;
    let direction = movement_context.get_degrees(memory)?;

    let player_profile = internal_player.get_profile(memory)?;
    let player_info = player_profile.get_player_info(memory)?;
    let player_body = internal_player.get_body(memory)?;
    let weapon_animator = internal_player.get_procedural_weapon(memory)?;

    let name = player_info.get_name(memory)?;
    let id = player_profile.get_id(memory)?;
    let location = player_body.get_location(memory)?;
    let is_dead = internal_player.is_dead(memory)?;
    let is_local = internal_player.is_local(memory)?;
    let is_aiming = weapon_animator.is_aiming(memory)?;
    let last_aggressor = internal_player
        .get_last_agressor(memory)
        .map_or(
            Err(anyhow!("Player has no last aggressor")),
            |last_aggressor: InternalPlayer| -> Result<String> {
                let last_aggressor_profile = last_aggressor.get_profile(memory)?;
                let last_aggressor_info = last_aggressor_profile.get_player_info(memory)?;
                let last_aggressor_name = last_aggressor_info.get_name(memory)?;
                Ok(last_aggressor_name)
            },
        )
        .map_or_else(|_| None, |v| Some(v));

    let player = match (name.clone(), is_dead, is_local) {
        (_, _, true) => Player::new_local(
            player_ptr,
            name,
            id,
            location,
            direction,
            last_aggressor,
            weapon_animator,
            is_aiming,
            game_state,
            memory,
        )?,
        (_, true, _) => Player::new_dead(location),
        _ => Player::new_enemy(
            player_ptr,
            name,
            id,
            location,
            direction,
            last_aggressor,
            is_aiming,
        ),
    };

    Ok(player)
}
