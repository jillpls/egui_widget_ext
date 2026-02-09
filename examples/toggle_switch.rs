//! # Toggle Switch Demo Example
//!
//! This example demonstrates the usage of the `toggle_switch` widget from the
//! `egui_widget_ext` crate. It shows how to use the toggle switch in an egui app
//! and how its state can be used to dynamically change a label.
//!
//! Run this example to see the toggle switch in action. Toggling the switch will
//! update the label to indicate whether it is ON or OFF.
//!
//! To run this example:
//! ```sh
//! cargo run --example toggle_switch --features "toggle_switch"
//! ```
//!
//! This file is intended for demonstration and manual testing purposes only.

use eframe::egui;
use egui_widget_ext::toggle_switch;

struct ToggleSwitchApp {
    is_on: bool,
}

impl ToggleSwitchApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self { is_on: false }
    }
}

impl eframe::App for ToggleSwitchApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.add(toggle_switch(&mut self.is_on));
                let label = if self.is_on {
                    "The switch is ON"
                } else {
                    "The switch is OFF"
                };
                ui.label(label);
            });
        });
    }
}

fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_maximized(true),
        ..Default::default()
    };
    eframe::run_native(
        "Toggle Switch Demo",
        native_options,
        Box::new(|cc| Ok(Box::new(ToggleSwitchApp::new(cc)))),
    )
}
