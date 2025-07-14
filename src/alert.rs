//! # Alert Widget Module
//!
//! This module provides a customizable alert box widget for use with the `egui` GUI library.
//! The alert box displays a message with a severity level (success, info, warning, error) and
//! includes a close ("✕") button. The appearance of the alert can be customized via margins and corner radius.
//!
//! ## Example
//! ```
//! # egui::__run_test_ui(|ui| {
//! use egui_widget_ext::{alert, AlertLevel};
//! ui.add(alert(AlertLevel::Warning, "This is a warning!")).clicked().then(|| {
//!     println!("Alert clicked!");
//! });
//! # });
//! ```
//!
//! ## Components
//! - [`AlertLevel`]: Enum representing the severity of the alert.
//! - [`Alert`]: Struct for configuring and displaying the alert widget.
//! - [`alert`]: Convenience function for creating an alert widget.

use std::hash::Hash;

use egui::{Button, Color32, CornerRadius, Frame, Label, Margin, RichText, Stroke, Ui, Widget};

/// Represents the severity level of an alert. Determines the background color and semantic meaning
/// of the alert box.
///
/// - `Success`: Indicates a successful operation or state (green).
/// - `Info`: Indicates informational messages that are not critical (blue).
/// - `Warning`: Indicates a warning that may require attention but is not critical (yellow).
/// - `Error`: Indicates an error or critical issue that needs immediate attention (red).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AlertLevel {
    /// Indicates a successful operation or state.
    Success,
    /// Indicates informational messages that are not critical.
    Info,
    /// Indicates a warning that may require attention but is not critical.
    Warning,
    /// Indicates an error or critical issue that needs immediate attention.
    Error,
}

/// A customizable alert box widget for egui.
///
/// The `Alert` struct allows you to configure the appearance and message of the alert box.
/// It supports setting the background color (via [`AlertLevel`]), the message, inner and outer margins,
/// and the corner radius. The alert box always includes a close ("✕") button.
///
/// Use the [`alert`] function for a convenient way to create an alert with a given level and message.
#[derive(Debug, Clone, PartialEq)]
pub struct Alert {
    /// The severity level of the alert.
    level: AlertLevel,
    /// The background color of the alert box.
    color: Color32,
    /// The message displayed in the alert box.
    message: String,
    /// Padding inside the alert box.
    inner_margin: i8,
    /// Margin outside the alert box.
    outer_margin: i8,
    /// Corner radius of the alert box.
    corner_radius: u8,
    /// Whether to show the close ("✕") button.
    can_close: bool,
    /// Optional width constraint for the alert box.
    width: Option<f32>,
}

impl Hash for Alert {
    /// Hash the alert's properties to ensure consistent behavior in hash maps and sets.
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.level.hash(state);
        self.color.hash(state);
        self.message.hash(state);
        self.inner_margin.hash(state);
        self.outer_margin.hash(state);
        self.corner_radius.hash(state);
        self.can_close.hash(state);
        self.width.unwrap_or(-1.0).to_bits().hash(state);
    }
}

impl Default for Alert {
    /// Creates a default alert with a generic error color and message.
    fn default() -> Self {
        Alert {
            level: AlertLevel::Info,
            color: Color32::from_rgb(255, 200, 200),
            message: "No message provided".to_string(),
            inner_margin: 10,
            outer_margin: 1,
            corner_radius: 4,
            can_close: true, // Show close button by default
            width: None,
        }
    }
}

impl Alert {
    /// Create a new alert with the given message and default info color.
    pub fn new(message: &str) -> Self {
        let level = AlertLevel::Info;
        let color = Self::level_to_color(level);
        Self {
            level,
            color,
            message: message.to_string(),
            ..Default::default()
        }
    }

    /// Set the alert's severity level, which determines its background color.
    pub fn with_level(mut self, level: AlertLevel) -> Self {
        self.level = level;
        self.color = Self::level_to_color(level);
        self
    }

    /// Set the inner margin (padding) of the alert box.
    pub fn inner_margin(mut self, margin: i8) -> Self {
        self.inner_margin = margin;
        self
    }

    /// Set the outer margin of the alert box.
    pub fn outer_margin(mut self, margin: i8) -> Self {
        self.outer_margin = margin;
        self
    }

    /// Set the corner radius of the alert box.
    pub fn corner_radius(mut self, radius: u8) -> Self {
        self.corner_radius = radius;
        self
    }

    /// Set whether the close ("✕") button is shown.
    pub fn can_close(mut self, closeable: bool) -> Self {
        self.can_close = closeable;
        self
    }

    /// Set the width of the alert box.
    pub fn width(mut self, width: f32) -> Self {
        self.width = Some(width);
        self
    }

    /// Map an [`AlertLevel`] to its corresponding background color.
    fn level_to_color(level: AlertLevel) -> Color32 {
        match level {
            AlertLevel::Success => Color32::LIGHT_GREEN,
            AlertLevel::Info => Color32::LIGHT_BLUE,
            AlertLevel::Warning => Color32::LIGHT_YELLOW,
            AlertLevel::Error => Color32::LIGHT_RED,
        }
    }

    /// Expose alert message for external access.
    pub fn get_message(&self) -> &str {
        &self.message
    }

    /// Get the alert's severity level.
    pub fn get_level(&self) -> AlertLevel {
        self.level
    }
}

impl Widget for Alert {
    /// Render the alert widget in the given egui UI context.
    ///
    /// The alert is displayed as a colored frame with the message and an optional close button.
    /// The returned [`egui::Response`] covers both the label and the close button (if present).
    fn ui(self, ui: &mut Ui) -> egui::Response {
        ui.set_width(self.width.unwrap_or(ui.available_width()));
        Frame::default()
            .fill(self.color)
            .stroke(Stroke::new(1.0, Color32::from_rgb(200, 200, 200)))
            .corner_radius(CornerRadius::same(self.corner_radius))
            .inner_margin(Margin::same(self.inner_margin))
            .outer_margin(Margin::same(self.outer_margin))
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    if self.can_close {
                        let _r2 = ui.add_enabled(
                            false,
                            Label::new(RichText::new(&self.message).color(Color32::BLACK)).wrap(),
                        );
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            ui.add(
                                Button::new(RichText::new("X").color(Color32::DARK_RED).strong())
                                    .frame(false),
                            )
                        })
                        .inner
                    } else {
                        let label_resp = ui.add_enabled(
                            false,
                            Label::new(RichText::new(&self.message).color(Color32::BLACK)).wrap(),
                        );
                        ui.add_space(ui.available_width());
                        label_resp
                    }
                })
                .inner
            })
            .inner
    }
}

/// Convenience function to create an alert widget with a given level and message.
///
/// # Parameters
/// - `level`: The [`AlertLevel`] of the alert, which determines the background color.
/// - `message`: The message to display inside the alert box.
///
/// # Returns
/// Returns an [`Alert`] widget configured with the specified level and message.
///
/// # Example
/// ```
/// # egui::__run_test_ui(|ui| {
/// use egui_widget_ext::{alert, AlertLevel};
/// ui.add(alert(AlertLevel::Warning, "This is a warning!")).clicked().then(|| {
///     println!("Alert clicked!");
/// });
/// # });
/// ```
pub fn alert(level: AlertLevel, message: &str) -> Alert {
    Alert::new(message).with_level(level)
}
