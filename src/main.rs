use anyhow::Result;
use esp::esp::Esp;
use external_memory_lib::utilities::builder::MemoryConfigurer;
use radar::radar::start_radar;
use std::{sync::Arc, thread};

use crossbeam_channel::bounded;
use domain::{
    application::{memory_operations::MemoryOperations, operations::Operations},
    models::player::Player,
};

use winapi::um::winuser::GetAsyncKeyState;

fn main() -> Result<()> {
    // load memory
    let memory = Arc::new(
        MemoryConfigurer::default()
            .configure("EscapeFromTarkov.exe", "UnityPlayer.dll", 0x17FFD28)
            .build()
            .unwrap(),
    );

    // create channels for inproc comms
    let shared_state = Arc::new(MemoryOperations::new(memory).unwrap());
    let (sender, receiver) = bounded(1);

    // create async producers
    player_loop(shared_state.clone(), sender);
    input_loop(shared_state);

    // create async consumers
    // start_radar(receiver);

    let esp = Esp::new(receiver);
    esp.start()?;

    Ok(())
}

fn player_loop(
    player_state: Arc<MemoryOperations>,
    sender: crossbeam_channel::Sender<Vec<Player>>,
) {
    thread::spawn(move || {
        let mut players: Vec<Player> = Vec::new();
        loop {
            let new_players_res = player_state.update_players(&players);

            if let Ok(new_players) = new_players_res {
                sender.send(new_players.clone()).unwrap();
                players = new_players;
            } else {
                eprintln!("Error: {:?}", new_players_res);
            }
        }
    });
}

fn input_loop(input_state: Arc<MemoryOperations>) {
    thread::scope(|s| {
        let mut thermal_toggled = false;
        input_state.toggle_thermal(&false).unwrap();
        s.spawn(move || {
            if unsafe { GetAsyncKeyState(winapi::um::winuser::VK_RETURN) != 0 } {
                thermal_toggled = input_state.toggle_thermal(&thermal_toggled).unwrap();
                eprintln!("Thermal toggled: {}", thermal_toggled);
            }
        });
    });
}
