//! # Toast Manager Demo Example
//!
//! This example demonstrates the usage of the `ToastManager` widget from the
//! `egui_widget_ext` crate. It shows how to display toast notifications with different durations
//! and how to trigger them from the UI. Toasts are transient messages that disappear automatically.
//!
//! ## Example
//! Run this example with:
//! ```sh
//! cargo run --example toast_manager --features "toast toast_manager"
//! ```
//!
//! This will open a window demonstrating the toast manager's capabilities.

use eframe::egui;
use egui::{CentralPanel, TopBottomPanel};
use egui_widget_ext::{Toast, ToastManager};
use std::collections::VecDeque;
use std::ops::DerefMut;
use std::sync::Mutex;

struct ToastManagerApp {
    /// List of toasts to be managed by the ToastManager.
    toasts: Mutex<VecDeque<Toast>>,
    /// Maximum number of toasts to display at once.
    max_toasts: usize,
    /// Anchor position for the toast area.
    anchor: egui::Align2,
}

impl Default for ToastManagerApp {
    fn default() -> Self {
        Self {
            toasts: Mutex::new(VecDeque::new()),
            max_toasts: 4,
            anchor: egui::Align2::RIGHT_BOTTOM,
        }
    }
}

impl ToastManagerApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}

impl eframe::App for ToastManagerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Always repaint so toasts can expire
        ctx.request_repaint_after(std::time::Duration::from_millis(100));

        TopBottomPanel::top("top_panel").show(ctx, |ui| {
            let mut toasts_guard = self.toasts.try_lock().unwrap();
            let toasts = toasts_guard.deref_mut();
            ui.horizontal(|ui| {
                if ui.button("Show Info Toast").clicked() {
                    toasts.push_back(
                        Toast::new("This is an info toast!")
                            .with_color(egui::Color32::from_rgb(200, 200, 255))
                            .duration(chrono::Duration::seconds(3)),
                    );
                }
                if ui.button("Show Success Toast").clicked() {
                    toasts.push_back(
                        Toast::new("Success!")
                            .with_color(egui::Color32::LIGHT_GREEN)
                            .duration(chrono::Duration::seconds(2)),
                    );
                }
                if ui.button("Show Error Toast").clicked() {
                    toasts.push_back(
                        Toast::new("Something went wrong!")
                            .with_color(egui::Color32::LIGHT_RED)
                            .duration(chrono::Duration::seconds(4)),
                    );
                }
                if ui.button("Show Many Toasts").clicked() {
                    for i in 0..5 {
                        toasts.push_back(
                            Toast::new(&format!("Toast #{i}"))
                                .duration(chrono::Duration::seconds(2 + i)),
                        );
                    }
                }
            });
            ui.separator();
            ui.label("Toast Manager Settings:");
            ui.horizontal(|ui| {
                ui.label("Max Toasts:");
                ui.add(egui::DragValue::new(&mut self.max_toasts).range(1..=10));
            });
            ui.horizontal(|ui| {
                ui.label("Anchor:");
                let anchors = [
                    ("Top Left", egui::Align2::LEFT_TOP),
                    ("Top Right", egui::Align2::RIGHT_TOP),
                    ("Bottom Left", egui::Align2::LEFT_BOTTOM),
                    ("Bottom Right", egui::Align2::RIGHT_BOTTOM),
                    ("Center Top", egui::Align2::CENTER_TOP),
                    ("Center Bottom", egui::Align2::CENTER_BOTTOM),
                ];
                for (label, anchor) in anchors.iter() {
                    if ui
                        .selectable_label(self.anchor == *anchor, *label)
                        .clicked()
                    {
                        self.anchor = *anchor;
                    }
                }
            });
        });

        CentralPanel::default().show(ctx, |ui| {
            ui.heading("Toast Manager Demo");
            ui.label(
                "Use the buttons above to trigger toasts. Toasts will appear in the selected \
anchor location and disappear automatically.",
            );
            let mut toasts_guard = self.toasts.try_lock().unwrap();
            let toasts = toasts_guard.deref_mut();
            ui.add(
                ToastManager::new(toasts, "main")
                    .max_toasts(self.max_toasts)
                    .anchor(self.anchor),
            );
        });
    }
}

fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_maximized(true),
        ..Default::default()
    };
    eframe::run_native(
        "Toast Manager Demo",
        native_options,
        Box::new(|cc| Ok(Box::new(ToastManagerApp::new(cc)))),
    )
}
