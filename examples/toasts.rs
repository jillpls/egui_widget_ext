//! # Toast Demo Example
//!
//! This example demonstrates the usage of the `toast` widget from the `egui_widget_ext` crate.
//! It shows how to display toast notifications with different durations, colors, and messages.
//! Toasts are transient and disappear after a set duration.
//!
//! To run this example:
//! ```sh
//! cargo run --example toasts
//! ```

use chrono::Duration;
use eframe::egui;
use egui::{Align2, Color32, Context, Vec2};
use egui_widget_ext::Toast;

struct ToastsApp {
    toasts: Vec<Toast>,
}

impl ToastsApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self { toasts: Vec::new() }
    }
}

impl eframe::App for ToastsApp {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Toast Demo");
            ui.label("Click a button to show a toast notification.");

            ui.horizontal(|ui| {
                if ui.button("Info Toast").clicked() {
                    self.toasts.push(Toast::new("This is an info toast!"));
                }
                if ui.button("Success Toast").clicked() {
                    self.toasts.push(
                        Toast::new("This is a success toast!")
                            .with_color(Color32::from_rgb(76, 175, 80)),
                    );
                }
                if ui.button("8 sec Long Toast").clicked() {
                    self.toasts.push(
                        Toast::new("This toast will last for 8 seconds!")
                            .duration(Duration::seconds(8)),
                    );
                }
            });

            // Remove expired toasts
            self.toasts.retain(|entry| !entry.has_expired());

            egui::Area::new(egui::Id::new("toasts_area"))
                .anchor(Align2::RIGHT_BOTTOM, Vec2::new(0.0, 0.0))
                .order(egui::Order::Foreground)
                .show(ui.ctx(), |ui| {
                    ui.set_width(350.0);
                    ui.vertical(|ui| {
                        for toast in &self.toasts {
                            ui.add(toast.clone());
                        }
                    });
                });

            // Ensure toasts expire even if there's no user input
            if !self.toasts.is_empty() {
                ctx.request_repaint_after(std::time::Duration::from_millis(200));
            }
        });
    }
}

fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_maximized(true),
        ..Default::default()
    };
    eframe::run_native(
        "Toast Demo",
        native_options,
        Box::new(|cc| Ok(Box::new(ToastsApp::new(cc)))),
    )
}
