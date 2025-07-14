//! # Alert Manager Widget Module
//!
//! This module provides an `AlertManager` widget for managing and displaying a stack of alert messages
//! in an egui application. The `AlertManager` is designed to help you present multiple alerts with
//! consistent styling, positioning, and dismissal behavior.
//!
//! ## Usage
//!
//! The `AlertManager` takes a mutable reference to a `Vec<Alert>` representing the current
//! alerts to display. It provides builder-style methods to configure margins, corner radius, width, anchor
//! alignment, custom position, anchor offset, and maximum height for the alert area. Each alert is rendered
//! using the `Alert` widget, and closed alerts are automatically removed from the vector.
//!
//! The `AlertManager` is designed to be an overlay that dynamically sizes to its content allowing
//! non-covered areas to remain interactive but also switches to a scrollable area if the number of
//! alerts results in a full screen. The overlay is positioned within the parent area based on the
//! specified anchor alignment and optional offset. This setup allows for flexible placement but is
//! best suited for dead center top/bottom for the central panel or as a side panel.
//!
//! It is recommended to leave the width/height alone and let it inherit from the parent area, but
//! you have the option to set both if desired.
//!
//! ## Example
//! ```rust
//! # use egui_widget_ext::{AlertManager, Alert};
//! # use egui::{CentralPanel, Context};
//! # fn ui_example(ctx: &Context, alerts: &mut Vec<Alert>) {
//! CentralPanel::default().show(ctx, |ui| {
//!     ui.add(AlertManager::new(alerts, "main")
//!         .corner_radius(8)
//!         .width(400.0)
//!         .anchor(egui::Align2::CENTER_TOP)
//!         .max_height(300.0));
//! });
//! # }
//! ```
//!
//! ## Features
//! - Shared styling for all alerts (margin, corner radius, width, etc.)
//! - Configurable anchor alignment and custom position
//! - Optional anchor offset for fine-tuned placement
//! - Automatic removal of closed alerts from the vector
//! - Scrollable area if alerts exceed the maximum height
//!
//! ## Note
//! The alert manager is intended for use with the `Alert` widget and expects each alert to be an
//! `Alert`. You can push new alerts to the vector at any time, and they will be displayed
//! until dismissed by the user.

use egui::{Align2, Id, Order, ScrollArea, Ui, Vec2, Widget};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use crate::Alert;

/// Manages and displays a list of alerts with shared styling and positioning.
#[derive(Debug)]
pub struct AlertManager<'a> {
    /// Unique key for the alert manager instance (used for state management).
    pub unique_key: String,
    /// List of alerts.
    pub alerts: &'a mut Vec<Alert>,
    /// Default inner margin for alerts.
    pub inner_margin: i8,
    /// Default outer margin for alerts.
    pub outer_margin: i8,
    /// Default corner radius for alerts.
    pub corner_radius: u8,
    /// Default width for the alert area (optional).
    pub width: Option<f32>,
    /// Whether alerts can be closed.
    pub can_close: bool,
    /// Anchor position for the alert stack.
    pub anchor: Align2,
    /// Optional offset from the anchor position.
    pub anchor_offset: Option<Vec2>,
    /// Optional maximum height for the alert area (enables scrolling if exceeded).
    pub max_height: Option<f32>,
}

impl Hash for AlertManager<'_> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.unique_key.hash(state);
        for a in self.alerts.iter() {
            a.hash(state);
        }
        self.inner_margin.hash(state);
        self.outer_margin.hash(state);
        self.corner_radius.hash(state);
        self.width.unwrap_or(-1.0).to_bits().hash(state);
        self.can_close.hash(state);
        self.anchor.hash(state);
        self.anchor_offset
            .unwrap_or(Vec2::ZERO)
            .to_string()
            .hash(state);
        self.max_height.unwrap_or(-1.0).to_bits().hash(state); // Use to_bits for f32
    }
}

impl<'a> AlertManager<'a> {
    /// Create a new alert manager with a reference to a list of alerts.
    pub fn new(alerts: &'a mut Vec<Alert>, unique_key: &str) -> Self {
        Self {
            unique_key: format!("alert_manager_{}", unique_key),
            alerts,
            inner_margin: 10,
            outer_margin: 1,
            corner_radius: 4,
            width: None,
            can_close: true,
            anchor: Align2::CENTER_TOP, // Default to top center
            anchor_offset: None,
            max_height: None,
        }
    }

    /// Set the inner margin for all alerts.
    pub fn inner_margin(mut self, margin: i8) -> Self {
        self.inner_margin = margin;
        self
    }

    /// Set the outer margin for all alerts.
    pub fn outer_margin(mut self, margin: i8) -> Self {
        self.outer_margin = margin;
        self
    }

    /// Set the corner radius for all alerts.
    pub fn corner_radius(mut self, radius: u8) -> Self {
        self.corner_radius = radius;
        self
    }

    /// Set the width for the alert area.
    pub fn width(mut self, width: f32) -> Self {
        self.width = Some(width);
        self
    }

    /// Set whether alerts can be closed.
    pub fn can_close(mut self, can_close: bool) -> Self {
        self.can_close = can_close;
        self
    }

    /// Set the anchor position for the alert stack.
    pub fn anchor(mut self, anchor: Align2) -> Self {
        let is_valid = matches!(
            anchor,
            Align2::LEFT_TOP
                | Align2::CENTER_TOP
                | Align2::RIGHT_TOP
                | Align2::LEFT_BOTTOM
                | Align2::CENTER_BOTTOM
                | Align2::RIGHT_BOTTOM
        );
        assert!(
            is_valid,
            "Invalid anchor position: {:?}. We only support top or bottom anchors.",
            anchor
        );
        self.anchor = anchor;
        self
    }

    /// Set an offset from the anchor position.
    pub fn anchor_offset(mut self, offset: Vec2) -> Self {
        self.anchor_offset = Some(offset);
        self
    }

    /// Set the maximum height for the alert area (enables scrolling if exceeded).
    pub fn max_height(mut self, max_height: f32) -> Self {
        self.max_height = Some(max_height);
        self
    }
}

impl<'a> Widget for AlertManager<'a> {
    fn ui(self, ui: &mut Ui) -> egui::Response {
        let parent_area = ui.max_rect();
        let mut to_remove = Vec::new();
        let hasher_id = Id::new(format!("alert_manager_hash_{}", self.unique_key));
        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);
        let hash = hasher.finish();
        let old_hash = ui.ctx().memory(|mem| mem.data.get_temp::<u64>(hasher_id));
        ui.ctx().memory_mut(|mem| {
            mem.data.insert_temp(hasher_id, hash);
        });

        // Determine best size limits
        let max_height = self
            .max_height
            .unwrap_or(parent_area.height())
            .clamp(1.0, parent_area.height());
        let max_width = self
            .width
            .unwrap_or(parent_area.width())
            .clamp(1.0, parent_area.width());

        // Create floating area for the alert manager
        let id: Id = Id::new(self.unique_key.clone());
        egui::Area::new(id)
            .order(Order::Foreground)
            .anchor(self.anchor, self.anchor_offset.unwrap_or(Vec2::ZERO))
            .constrain_to(parent_area)
            .default_size(Vec2::new(max_width, max_height))
            .sizing_pass(old_hash.is_some() && old_hash.unwrap() != hash)
            .show(ui.ctx(), |ui| {
                if !ui.is_enabled() && !ui.is_visible() {
                    // Detect sizing pass: do not use ScrollArea since that will hide the content size
                    // resulting in a chicken and egg problem.
                    for alert in self.alerts.iter() {
                        let mut new_alert = alert
                            .clone()
                            .inner_margin(self.inner_margin)
                            .outer_margin(self.outer_margin)
                            .corner_radius(self.corner_radius)
                            .can_close(self.can_close);
                        if self.width.is_some() {
                            new_alert = new_alert.width(self.width.unwrap());
                        }
                        ui.add(new_alert);
                    }
                } else {
                    let is_bottom = self.anchor == Align2::LEFT_BOTTOM
                        || self.anchor == Align2::CENTER_BOTTOM
                        || self.anchor == Align2::RIGHT_BOTTOM;
                    // Normal pass: use ScrollArea
                    let scroll_resp = ScrollArea::both()
                        .stick_to_bottom(is_bottom)
                        .max_height(max_height)
                        .max_width(max_width)
                        .show(ui, |ui| {
                            // Reverse alerts order if bottom anchor
                            let alert_iter: Box<dyn Iterator<Item = (usize, &Alert)>> = if is_bottom
                            {
                                Box::new(self.alerts.iter().enumerate())
                            } else {
                                Box::new(self.alerts.iter().enumerate().rev())
                            };

                            // Iterate through alerts and render them
                            for (idx, alert) in alert_iter {
                                let mut new_alert = alert
                                    .clone()
                                    .inner_margin(self.inner_margin)
                                    .outer_margin(self.outer_margin)
                                    .corner_radius(self.corner_radius)
                                    .can_close(self.can_close);
                                if self.width.is_some() {
                                    new_alert = new_alert.width(self.width.unwrap());
                                }
                                let resp = ui.add(new_alert);
                                if self.can_close && resp.clicked() {
                                    to_remove.push(idx);
                                }
                            }

                            // Remove closed alerts in reverse order to avoid index shifting issues
                            for idx in to_remove.into_iter().rev() {
                                self.alerts.remove(idx);
                            }
                        });
                    scroll_resp.inner
                }
            })
            .response
    }
}

/// Convenience function to create an alert manager widget with a mutable vector of alerts.
pub fn alert_manager<'a>(alerts: &'a mut Vec<Alert>, unique_key: &str) -> AlertManager<'a> {
    AlertManager::new(alerts, unique_key)
}
