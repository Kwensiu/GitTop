//! Linux-specific platform implementations.

/// Focus an existing GitTop window.
/// TODO: Implement using X11/Wayland window activation.
pub fn focus_existing_window() {
    // On Linux, this depends on the display server (X11 vs Wayland).
    // X11: Could use xdotool or libX11 to find and activate window
    // Wayland: More complex, compositor-specific protocols
    // For now, this is a no-op - the tray icon handles window restoration.
}

/// Enable dark mode for system UI elements.
/// Linux context menus follow GTK/Qt theme settings.
pub fn enable_dark_mode() {
    // Linux context menus use GTK theming.
    // The theme is controlled by GTK_THEME or gsettings.
    // tray-icon/muda should respect the system theme.
}

/// Reduce memory footprint.
/// Uses malloc_trim on glibc systems.
pub fn trim_memory() {
    // On Linux with glibc, we can call malloc_trim to release memory
    // back to the OS. This is similar to EmptyWorkingSet on Windows.
    #[cfg(target_env = "gnu")]
    unsafe {
        // malloc_trim returns 1 if memory was released, 0 otherwise
        extern "C" {
            fn malloc_trim(pad: usize) -> i32;
        }
        let _ = malloc_trim(0);
    }
}
