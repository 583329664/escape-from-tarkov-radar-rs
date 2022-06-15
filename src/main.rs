use std::{sync::Arc, time::Duration, thread::sleep};
use anyhow::Result;

use crossbeam_channel::bounded;
use domain::{application::{memory_operations::MemoryOperations, operations::Operations}, models::player::Player};
use external_memory_lib::MemoryConfigurer;
use inputbot::handle_input_events;
use inputbot::KeybdKey::{*};
use view::radar;

fn main() -> Result<()> {
    let memory = Arc::new(
        MemoryConfigurer::default()
            .configure("EscapeFromTarkov.exe", "UnityPlayer.dll", 0x17FFD28)
            .build()
            .unwrap(),
    );

    let shared_state = Arc::new(MemoryOperations::new(memory).unwrap());
    let (sender, receiver) = bounded(1);

    // player thread
    let player_state = shared_state.clone();
    std::thread::spawn(move || {
        let mut players: Vec<Player> = Vec::new();
        loop {
            let new_players_res = player_state.update_players(&players);

            if let Ok(new_players) = new_players_res {
                sender.send(new_players.clone()).unwrap();
                players = new_players;
            }
        }
    });

    // input thread
    std::thread::spawn(move || {
        let key_state = shared_state.clone();
        BackspaceKey.bind(move || {
            while BackspaceKey.is_pressed() {
                key_state.toggle_thermal(&true).unwrap();
                sleep(Duration::from_millis(500));
            }
        });

        let other_key_state = shared_state;
        EnterKey.bind(move || {
            while EnterKey.is_pressed() {
                other_key_state.toggle_thermal(&false).unwrap();
                sleep(Duration::from_millis(500));
            }
        });

        handle_input_events();
    });

    // gui thread
    radar::start_radar(receiver);

    Ok(())
}
