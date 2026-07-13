#![forbid(unsafe_code)]
#![cfg_attr(not(debug_assertions), deny(warnings))] // Forbid warnings in release builds
#![warn(clippy::all, rust_2018_idioms)]

mod app;

use app::NodeGraphExampleSimple;

fn main() {
    // egui native app boilerplate:
    eframe::run_native(
        "Egui Graph Edit simple example",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Ok(Box::<NodeGraphExampleSimple>::default())),
    )
    .expect("Failed to run native example");
}
