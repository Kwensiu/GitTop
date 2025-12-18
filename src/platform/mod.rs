//! Platform-specific functionality.
//!
//! This module provides cross-platform abstractions for OS-specific features
//! like memory management, window focusing, and theme settings.

#[cfg(windows)]
mod windows;

#[cfg(target_os = "macos")]
mod macos;

#[cfg(target_os = "linux")]
mod linux;

#[cfg(target_os = "freebsd")]
mod freebsd;

// Re-export platform functions with unified API

/// Focus an existing application window (for single-instance support).
/// Called when a second instance tries to launch.
pub fn focus_existing_window() {
    #[cfg(windows)]
    windows::focus_existing_window();
    
    #[cfg(target_os = "macos")]
    macos::focus_existing_window();
    
    #[cfg(target_os = "linux")]
    linux::focus_existing_window();
    
    #[cfg(target_os = "freebsd")]
    freebsd::focus_existing_window();
}

/// Enable dark mode for system UI elements (context menus, etc.).
/// Should be called early in app initialization.
pub fn enable_dark_mode() {
    #[cfg(windows)]
    windows::enable_dark_mode();
    
    #[cfg(target_os = "macos")]
    macos::enable_dark_mode();
    
    #[cfg(target_os = "linux")]
    linux::enable_dark_mode();
    
    #[cfg(target_os = "freebsd")]
    freebsd::enable_dark_mode();
}

/// Aggressively reduce memory footprint.
/// Trims working set on Windows, may trigger GC hints on other platforms.
/// Call when minimizing to tray.
pub fn trim_memory() {
    #[cfg(windows)]
    windows::trim_working_set();
    
    #[cfg(target_os = "macos")]
    macos::trim_memory();
    
    #[cfg(target_os = "linux")]
    linux::trim_memory();
    
    #[cfg(target_os = "freebsd")]
    freebsd::trim_memory();
}
