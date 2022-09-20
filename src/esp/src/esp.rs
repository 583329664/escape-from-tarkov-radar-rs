use std::f32::consts::PI;

use anyhow::Result;
use crossbeam_channel::Receiver;
use domain::{
    game::maths::{Vector2, Vector3},
    models::player::{LocalPlayer, Player, PlayerType},
};
use nvidia_overlay_hook::types::{overlay::Overlay, rectangle::Rectangle};

pub struct Esp {
    pub player_channel: Receiver<Vec<Player>>,
}

impl Esp {
    pub fn new(player_channel: Receiver<Vec<Player>>) -> Self {
        Esp { player_channel }
    }

    pub fn start(&self) -> Result<()> {
        let mut overlay = Overlay::new().expect("Failed to create overlay");

        loop {
            overlay.begin_draw();
            overlay.clear()?;

            let players = self.player_channel.try_recv().unwrap_or_default();
            let local_player_res = players.iter().find(|p| p.as_local().is_some());

            if players.is_empty() || local_player_res.is_none() {
                continue;
            }

            let local_player = local_player_res.unwrap();
            let local_player_kind = local_player.as_local().unwrap();
            let window_width_height = overlay.get_width_and_height();

            for player in players.iter() {
                if player.player_type == PlayerType::Local || local_player_kind.fps_camera.is_none()
                {
                    continue;
                }

                let world_location = world_to_screen(
                    player,
                    local_player,
                    local_player_kind,
                    window_width_height.0,
                    window_width_height.1,
                )
                .unwrap();

                if world_location == Vector2::ZERO {
                    continue;
                }

                let distance = local_player.location.distance(player.location);

                if distance > 300.0 {
                    continue;
                }

                let rectangle_height = 1500.0 / distance;
                let rectangle_width = 500.0 / distance;

                let rectangle = Rectangle::new_float(
                    world_location.x - rectangle_width / 2.0,
                    world_location.y - rectangle_height / 2.0,
                    rectangle_width,
                    rectangle_height,
                );

                overlay.draw_rectangle(rectangle)
            }

            overlay.end_draw()?;
        }
    }
}

fn world_to_screen(
    player: &Player,
    local_player: &Player,
    local_player_kind: &LocalPlayer,
    screen_width: f32,
    screen_height: f32,
) -> Result<Vector2> {
    let location = player.location;
    let aiming = local_player.is_aiming && local_player_kind.optic_camera.is_some();
    let fps_camera = local_player_kind.fps_camera.as_ref().unwrap();
    let optic_camera = local_player_kind.optic_camera.as_ref();

    if location == Vector3::default() {
        return Ok(Vector2::ZERO);
    }

    let matrix = if aiming {
        optic_camera.unwrap().matrix
    } else {
        fps_camera.matrix
    };
    let translated_matrix = matrix.transpose();
    let translation_vector = Vector3::new(
        translated_matrix.m41,
        translated_matrix.m42,
        translated_matrix.m43,
    );

    let w = translation_vector.dot_product(location) + translated_matrix.m44;

    if w < 0.1 {
        return Ok(Vector2::ZERO);
    }

    let up = Vector3::new(
        translated_matrix.m21,
        translated_matrix.m22,
        translated_matrix.m23,
    );
    let right = Vector3::new(
        translated_matrix.m11,
        translated_matrix.m12,
        translated_matrix.m13,
    );

    let mut x = up.dot_product(location) + translated_matrix.m24;
    let mut y = right.dot_product(location) + translated_matrix.m14;

    if aiming {
        let mut fov = fps_camera.fov;

        if fps_camera.fov == 35.0 && optic_camera.unwrap().fov == 19.4 {
            fov = 50.0;
        }

        let angle_rad_half = PI / 180.0 * fov * 0.5;
        let angle_ctg = (angle_rad_half).cos() / (angle_rad_half).sin();

        x /= angle_ctg * fps_camera.aspect_ratio * 0.5;
        y /= angle_ctg * 0.5;
    }

    let screen_x = screen_width / 2.0 * (1.0 + x / w);
    let screen_y = screen_height / 2.0 * (1.0 - y / w);

    return Ok(Vector2::new(screen_x, screen_y));
}
