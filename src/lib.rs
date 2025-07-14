//! # Egui Widget Extension Pack
//!
//! This crate provides additional widgets for use with egui, allowing for modular inclusion of UI components via feature flags.
//!
//! ## Features
//! The intent is to have a feature for each widget and its associated functionality so that users can include only what they need.
//! - `toggle_switch`: Simple toggle switch widget
//! - `alert`: Widget for displaying alerts
//! - `all`: Enables all widgets provided by this crate
//!
#[cfg(feature = "toggle_switch")]
mod toggle_switch;
#[cfg(feature = "toggle_switch")]
pub use toggle_switch::toggle_switch;
#[cfg(feature = "alert")]
mod alert;
#[cfg(feature = "alert")]
pub use alert::{Alert, AlertLevel, alert};
#[cfg(feature = "toast")]
mod toast;
#[cfg(feature = "toast")]
pub use toast::{Toast, toast};
#[cfg(feature = "alert_manager")]
mod alert_manager;
#[cfg(feature = "alert_manager")]
pub use alert_manager::{AlertManager, alert_manager};
#[cfg(feature = "toast_manager")]
mod toast_manager;
#[cfg(feature = "toast_manager")]
pub use toast_manager::{ToastManager, toast_manager};
