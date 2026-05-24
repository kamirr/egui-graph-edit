#![forbid(unsafe_code)]
#![cfg_attr(not(debug_assertions), deny(warnings))] // Forbid warnings in release builds
#![warn(clippy::all, rust_2018_idioms)]

use egui_graph_edit_example::NodeGraphExample;

// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
fn main() {
    use eframe::egui::Visuals;

    eframe::run_native(
        "Egui Graph Edit example",
        eframe::NativeOptions::default(),
        Box::new(|cc| {
            Ok({
                cc.egui_ctx.set_visuals(Visuals::dark());

                // Graph node IDs are not stable when the render order changes
                #[cfg(debug_assertions)]
                cc.egui_ctx.all_styles_mut(|s| {
                    s.debug.warn_if_rect_changes_id = false;
                });

                #[cfg(feature = "persistence")]
                {
                    Box::new(NodeGraphExample::new(cc))
                }
                #[cfg(not(feature = "persistence"))]
                Box::<NodeGraphExample>::default()
            })
        }),
    )
    .expect("Failed to run native example");
}
