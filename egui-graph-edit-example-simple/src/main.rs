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
        Box::new(|cc| {
            // Graph node IDs are not stable when the render order changes
            #[cfg(debug_assertions)]
            cc.egui_ctx.all_styles_mut(|s| {
                s.debug.warn_if_rect_changes_id = false;
            });

            Ok(Box::<NodeGraphExampleSimple>::default())
        }),
    )
    .expect("Failed to run native example");
}
