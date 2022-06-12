use eframe::{
    egui::{self, Label},
    emath::{Pos2, Vec2},
    epaint::{Color32, Stroke},
};
use geo::{coord, point, prelude::EuclideanDistance, Line};
use inputbot::{handle_input_events, KeybdKey::*};
use std::{ops::Add, sync::{Arc, Mutex}, thread::sleep, time::Duration};

use crate::domain::{
    operations::memory_operations::MemoryOperations, operations::operations::Operations,
};

use external_memory_lib::MemoryConfigurer;

pub struct Radar {
    pub operations: Arc<Mutex<dyn Operations>>,
}

impl Radar {
    pub fn new() -> Radar {
        let memory = Arc::new(
            MemoryConfigurer::default()
                .configure("EscapeFromTarkov.exe", "UnityPlayer.dll", 0x17FFD28)
                .build()
                .unwrap(),
        );

        let operations = Arc::new(Mutex::new(MemoryOperations::new(memory).unwrap()));

        let thermal_operations = Arc::clone(&operations);
        BackspaceKey.bind(move || {
            while BackspaceKey.is_pressed() {
                let _ = thermal_operations.lock().unwrap().toggle_thermal();
                sleep(Duration::from_millis(500));
            }
        });

        handle_input_events();

        Radar { operations }
    }
}

impl eframe::App for Radar {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let frame = egui::containers::Frame {
            fill: Color32::GRAY,
            stroke: Stroke::new(5.0, Color32::BLACK),
            rounding: egui::Rounding {
                nw: 5.0,
                ne: 5.0,
                sw: 5.0,
                se: 5.0,
            },
            inner_margin: egui::style::Margin {
                left: 20.,
                right: 20.,
                top: 20.,
                bottom: 20.,
            },
            outer_margin: egui::style::Margin {
                left: 20.,
                right: 20.,
                top: 20.,
                bottom: 20.,
            },
            shadow: eframe::epaint::Shadow {
                extrusion: 10.0,
                color: Color32::LIGHT_BLUE,
            },
        };

        egui::CentralPanel::default().frame(frame).show(ctx, |ui| {
            ui.heading("Radar Online...");

            let players = self.operations.lock().unwrap().get_players().unwrap_or_default();
            let local_player_res = players.iter().find(|p| p.is_local);

            if players.is_empty() || local_player_res.is_none() {
                ui.spinner();
                return;
            }

            let local_player = local_player_res.unwrap();

            let window_size = ui.available_size();
            let center = Pos2::new(window_size.x / 2.0, window_size.y / 2.0);

            let diff_x = center.x - local_player.location.x;
            let diff_y = center.y - local_player.location.y;

            let x_look = (local_player.direction.cos() * 2000.0) + center.x;
            let y_look = (local_player.direction.sin() * 2000.0) + center.y;

            let line: Line<f32> = Line::new(
                coord! { x: center.x, y: center.y },
                coord! { x: x_look, y: y_look },
            );

            let mut radar_color = Color32::LIGHT_BLUE;

            ui.painter().line_segment(
                [Pos2::new(center.x, center.y), Pos2::new(x_look, y_look)],
                Stroke::new(2.5, Color32::BLACK),
            );

            for player in players.iter() {
                let text = format!("   {}: \n    {}   ", player.name, player.location);
                let (_, galley, _) = Label::new(text).layout_in_ui(ui);
                let position = Pos2::new(player.location.x + diff_x, player.location.y + diff_y);

                let color = match player {
                    player if player.is_local => Color32::GREEN,
                    player if local_player.location.z - player.location.z > 5.0 => {
                        Color32::LIGHT_RED
                    }
                    player if local_player.location.z - player.location.z < -5.0 => {
                        Color32::from_rgb(128, 0, 32)
                    }
                    _ => Color32::RED,
                };

                let distance = line.euclidean_distance(&point! { x: position.x, y: position.y });
                if distance.abs() < 5.0 && (local_player.location.z - player.location.z).abs() < 5.0
                {
                    radar_color = Color32::from_rgba_unmultiplied(0, 0, 0x8B, 50);
                }

                galley.paint_with_color_override(
                    ui.painter(),
                    position.add(Vec2::new(5.0, -5.0)),
                    Color32::BLACK,
                );

                ui.painter().circle_filled(position, 10.0, color);

                if player.is_dead && player.last_aggressor.is_some() {
                    ui.heading(format!("{} was killed by {}", player.name, player.last_aggressor.as_ref().unwrap()));
                }
            }

            ui.painter().circle(
                center,
                400.0,
                radar_color,
                Stroke::new(2.5, Color32::from_rgba_unmultiplied(0xAD, 0xD8, 0xE6, 50)),
            );

            ctx.request_repaint();
        });
    }
}
