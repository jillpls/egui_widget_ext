//! # Toast Manager Widget Module
//!
//! This module provides a `ToastManager` widget for managing and displaying temporary toast
//! notifications in an egui application. Toasts are short-lived, non-blocking messages that
//! appear in a corner of the UI and automatically disappear after a set duration.
//!
//! ## Usage
//!
//! The `ToastManager` takes a mutable reference to a `VecDeque<Toast>` representing the current
//! toasts to display. The widget provides builder-style methods to configure the maximum number of
//! toasts, margins, corner radius, width, anchor alignment, and anchor offset. Each toast is
//! rendered using the `Toast` widget trait. All anchor positions are relative to the screen, and
//! NOT the parent area.
//!
//! The manager ensures that only up to `max_toasts` are displayed at once, automatically removing
//! the oldest toasts as new ones are added. Toasts are typically used for transient feedback.
//!
//! **Important:**  
//! Toasts rely on timeouts to disappear after a set duration. To ensure that the toast expires and
//! disappears at the correct time, you should call
//! `ctx.request_repaint_after(std::time::Duration::from_secs(1));` (or a similar interval) in your
//! egui update loop. This ensures that egui continues to repaint even if there is no user
//! interaction, allowing the toast to update and expire as expected.
//!
//!
//! ## Example
//! ```rust
//! use std::collections::VecDeque;
//! use egui_widget_ext::{ToastManager, Toast, toast, toast_manager};
//! fn ui_example(ui: &mut egui::Ui) {
//!     let mut toasts = VecDeque::from([
//!         // Convenience function to create a default toast
//!         toast("Saved successfully!"),
//!         toast("Error occurred"),
//!         // Custom toast with specific styling
//!         Toast::new("This is a custom toast!")
//!             .with_color(egui::Color32::from_rgb(255, 200, 200))
//!             .inner_margin(16)
//!             .outer_margin(8)
//!             .corner_radius(12)
//!             .width(300.0)
//!             .duration(chrono::Duration::seconds(5)),
//!     ]);
//!     // Add a manager that allows up to 3 toasts at once
//!     ui.add(ToastManager::new(&mut toasts, "main").max_toasts(3));
//!     // Add a separate manager using the convenience function
//!     ui.add(toast_manager(&mut toasts, "main1"));
//! }
//! ```
//!
//! ## Features
//! - Configurable maximum number of toasts
//! - Shared styling for all toasts (margin, corner radius, width, etc.)
//! - Configurable anchor alignment and offset
//! - Automatic removal of oldest toasts when limit is exceeded
//! - Each toast can have its own duration
//!
//! ## Note
//! - The `ToastManager` widget is designed to use a mutable reference to a `VecDeque<Toast>`.
//!   Manager settings are used to override the default appearance of all toasts.
//! - A scroll area is not supported in this widget as toasts are typically transient and low
//!   volume at any given time.
//! - The provided width is clamped to parent height if it exceeds the available space or 1.0 if it
//!   is less than 0.0. We use 1.0 to ensure something is shown to indicate that there are toasts.
//!

use std::collections::VecDeque;

use egui::Widget;

use crate::Toast;

pub struct ToastManager<'a> {
    /// Unique key for the toast manager area, used to prevent conflicts with other areas.
    unique_key: String,
    /// Mutable reference to the deque of toasts.
    toasts: &'a mut VecDeque<Toast>,
    max_toasts: usize,
    inner_margin: i8,
    outer_margin: i8,
    corner_radius: u8,
    width: f32,
    anchor: egui::Align2,
    anchor_offset: egui::Vec2,
}

impl<'a> ToastManager<'a> {
    pub fn new(toasts: &'a mut VecDeque<Toast>, unique_key: &str) -> Self {
        Self {
            unique_key: format!("toast_manager_{}", unique_key),
            toasts,
            max_toasts: 1,
            inner_margin: 5,
            outer_margin: 1,
            corner_radius: 4,
            width: 200.0,
            anchor: egui::Align2::RIGHT_BOTTOM,
            anchor_offset: egui::Vec2::ZERO,
        }
    }

    /// Set the maximum number of toasts to display.
    pub fn max_toasts(mut self, max: usize) -> Self {
        self.max_toasts = max;
        self
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
        self.width = width;
        self
    }

    /// Set the anchor position for the toast area.
    pub fn anchor(mut self, anchor: egui::Align2) -> Self {
        assert!(
            anchor == egui::Align2::RIGHT_BOTTOM
                || anchor == egui::Align2::LEFT_BOTTOM
                || anchor == egui::Align2::CENTER_BOTTOM
                || anchor == egui::Align2::RIGHT_TOP
                || anchor == egui::Align2::LEFT_TOP
                || anchor == egui::Align2::CENTER_TOP,
            "Invalid anchor position for ToastManager. Must be one of: RIGHT_BOTTOM, LEFT_BOTTOM,\
             CENTER_BOTTOM, RIGHT_TOP, LEFT_TOP, CENTER_TOP."
        );
        self.anchor = anchor;
        self
    }

    /// Set the anchor offset for the toast area.
    pub fn anchor_offset(mut self, offset: egui::Vec2) -> Self {
        self.anchor_offset = offset;
        self
    }
}

impl<'a> Widget for ToastManager<'a> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        // Remove expired toasts
        self.toasts.retain(|toast| !toast.has_expired());

        // Ensure we don't exceed the maximum number of toasts
        while self.toasts.len() > self.max_toasts {
            self.toasts.pop_front();
        }

        let parent_area = ui.max_rect();
        let width = self.width.clamp(1.0, parent_area.width());
        let is_bottom = self.anchor == egui::Align2::RIGHT_BOTTOM
            || self.anchor == egui::Align2::LEFT_BOTTOM
            || self.anchor == egui::Align2::CENTER_BOTTOM;

        let toast_iter: Box<dyn Iterator<Item = &Toast>> = if is_bottom {
            Box::new(self.toasts.iter())
        } else {
            Box::new(self.toasts.iter().rev())
        };

        egui::Area::new(egui::Id::new(self.unique_key))
            .anchor(self.anchor, self.anchor_offset)
            .show(ui.ctx(), |ui| {
                ui.vertical(|ui| {
                    for toast in toast_iter {
                        let toast = toast
                            .clone()
                            .inner_margin(self.inner_margin)
                            .outer_margin(self.outer_margin)
                            .corner_radius(self.corner_radius)
                            .width(width);
                        toast.ui(ui);
                    }
                });
            })
            .response
    }
}

/// Convenience function to create a toast manager widget with a mutable VecDeque of toasts.
///
/// # Parameters
/// - `toasts`: A mutable reference to a VecDeque of `(String, f32)` tuples representing the current
///   toasts.
///
/// # Returns
/// A `ToastManager` instance configured with the provided toasts.
///
/// # Example
/// ```
/// use std::collections::VecDeque;
/// use egui_widget_ext::{ToastManager, toast, toast_manager};
/// egui::__run_test_ui(|ui| {
///     let mut toasts = VecDeque::from([toast("Hello, World!")]);
///     ui.add(toast_manager(&mut toasts, "main"));
/// });
/// ```
pub fn toast_manager<'a>(toasts: &'a mut VecDeque<Toast>, unique_key: &str) -> ToastManager<'a> {
    ToastManager::new(toasts, unique_key)
}
