#![feature(strict_provenance)]
#![feature(stdarch)]

use anyhow::Result;
use radar::{Radar};

mod domain;
mod game;
mod radar;

fn main() -> Result<()> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Radar RS",
        options,
        Box::new(|_cc| Box::new(Radar::new()))
    );
}
