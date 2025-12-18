//! FreeBSD-specific platform implementations.

/// Focus an existing GitTop window.
/// TODO: Implement using X11 window activation.
pub fn focus_existing_window() {
    // FreeBSD typically uses X11, similar to Linux.
    // For now, this is a no-op.
}

/// Enable dark mode for system UI elements.
/// FreeBSD context menus follow GTK/Qt theme settings.
pub fn enable_dark_mode() {
    // Similar to Linux, GTK theming controls context menu appearance.
}

/// Reduce memory footprint.
pub fn trim_memory() {
    // FreeBSD uses jemalloc by default.
    // Could potentially call jemalloc's purge functions.
    // For now, this is a no-op - the OS handles memory pressure.
}
