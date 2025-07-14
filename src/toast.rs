//! # Toast Widget Module
//!
//! This module provides a customizable toast notification widget for use with the `egui` GUI
//! library. Toasts are transient messages that appear for a short duration and then disappear
//! automatically. They are useful for providing non-intrusive feedback to users, such as success
//! messages.
//!
//! ## Usage
//!
//! The [`Toast`] struct allows you to configure the appearance, message, color, margins, corner
//! radius, width, and duration of the toast. You can use the [`toast`] convenience function for a
//! quick way to create a toast with a message.
//!
//! **Important:**  
//! Toasts rely on timeouts to disappear after a set duration. To ensure that the toast expires and
//! disappears at the correct time, you should call
//! `ctx.request_repaint_after(chrono::Duration::seconds(1).to_std().unwrap_or_default());` (or a similar interval) in your
//! egui update loop. This ensures that egui continues to repaint even if there is no user
//! interaction, allowing the toast to update and expire as expected.
//!
//! ## Example
//! ```
//! # egui::__run_test_ui(|ui| {
//! use egui_widget_ext::{Toast, toast};
//! use egui::Color32;
//! use chrono::Duration;
//!
//! // Using the struct directly and exercising all configuration methods
//! let custom_toast = Toast::new("Custom toast")
//!     .with_color(Color32::from_rgb(255, 200, 200))
//!     .inner_margin(16)
//!     .outer_margin(8)
//!     .corner_radius(12)
//!     .width(300.0)
//!     .duration(Duration::seconds(5));
//! ui.add(custom_toast);
//!
//! // Using the convenience function
//! ui.add(toast("This is a default toast!"));
//! # });
//! ```
//!
//! ## Components
//! - [`Toast`]: Struct for configuring and displaying the toast widget.
//! - [`toast`]: Convenience function for creating a toast widget.

use chrono::{DateTime, Duration, Utc};

use egui::{Color32, CornerRadius, Frame, Label, Margin, Response, RichText, Stroke, Ui, Widget};

/// A customizable toast notification widget for egui.
///
/// The `Toast` struct allows you to configure the appearance and message of the toast box.
/// It supports setting the background color, message, inner and outer margins, corner radius, width,
/// and the duration for which the toast should be visible. Toasts are intended to be temporary and
/// will expire after the specified duration.
#[derive(Debug, Clone, PartialEq)]
pub struct Toast {
    /// The message to display in the toast.
    pub message: String,
    /// The background color of the toast.
    pub color: Color32,
    /// The inner margin (padding) of the toast box.
    pub inner_margin: i8,
    /// The outer margin of the toast box.
    pub outer_margin: i8,
    /// The corner radius of the toast box.
    pub corner_radius: u8,
    /// Toast width, if specified.
    pub width: Option<f32>,
    /// Start instant for the toast, used for timing.
    pub start_time: DateTime<Utc>,
    /// Duration for which the toast should be visible.
    pub duration: Duration,
}

impl Default for Toast {
    fn default() -> Self {
        Self {
            message: "No message provided".to_string(),
            color: Color32::from_rgb(200, 200, 255), // Default to a blue color
            inner_margin: 10,
            outer_margin: 1,
            corner_radius: 4,
            width: None,                    // Default to no specific width
            start_time: Utc::now(),         // Start timing immediately
            duration: Duration::seconds(3), // Default duration of 3 seconds
        }
    }
}

impl Toast {
    /// Create a new toast with the given message and default color.
    pub fn new(message: &str) -> Self {
        let color = Color32::from_rgb(200, 200, 255); // Default blue color
        Self {
            message: message.to_string(),
            color,
            ..Default::default()
        }
    }

    /// Set the background color of the toast.
    pub fn with_color(mut self, color: Color32) -> Self {
        self.color = color;
        self
    }

    /// Set the inner margin (padding) of the toast box.
    pub fn inner_margin(mut self, margin: i8) -> Self {
        self.inner_margin = margin;
        self
    }

    /// Set the outer margin of the toast box.
    pub fn outer_margin(mut self, margin: i8) -> Self {
        self.outer_margin = margin;
        self
    }

    /// Set the corner radius of the toast box.
    pub fn corner_radius(mut self, radius: u8) -> Self {
        self.corner_radius = radius;
        self
    }

    /// Set the width of the toast box.
    pub fn width(mut self, width: f32) -> Self {
        self.width = Some(width);
        self
    }

    /// Set the duration for which the toast should be visible.
    pub fn duration(mut self, duration: Duration) -> Self {
        self.duration = duration;
        self
    }

    /// Check if the toast has expired based on the current time.
    ///
    /// Returns `true` if the toast's duration has elapsed, otherwise `false`.
    pub fn has_expired(&self) -> bool {
        Utc::now().signed_duration_since(self.start_time) >= self.duration
    }
}

impl Widget for Toast {
    fn ui(self, ui: &mut Ui) -> Response {
        let frame = Frame::default()
            .fill(self.color)
            .stroke(Stroke::new(1.0, Color32::from_rgb(200, 200, 200)))
            .corner_radius(CornerRadius::same(self.corner_radius))
            .inner_margin(Margin::same(self.inner_margin))
            .outer_margin(Margin::same(self.outer_margin));

        let response = frame
            .show(ui, |ui| {
                if let Some(width) = self.width {
                    ui.set_width(width);
                }
                ui.horizontal(|ui| {
                    let r1 = ui
                        .add(Label::new(RichText::new(&self.message).color(Color32::BLACK)).wrap());
                    ui.add_space(ui.available_width());
                    r1
                })
                .inner
            })
            .inner;
        response
    }
}

/// Convenience function to create a toast with a message and default settings.
///
/// # Arguments
/// - `message`: The message to display in the toast.
///
/// # Returns
/// A new `Toast` instance with the specified message and default settings.
///
/// # Example
/// ```
/// # egui::__run_test_ui(|ui| {
/// use egui_widget_ext::{toast};
/// ui.add(toast("Success!"));
/// # });
/// ```
pub fn toast(message: &str) -> Toast {
    Toast::new(message)
}
