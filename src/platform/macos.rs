//! macOS-specific platform implementations.

/// Focus an existing GitTop window.
/// TODO: Implement using NSRunningApplication or AppleScript.
pub fn focus_existing_window() {
    // On macOS, the system typically handles single-instance apps
    // through the application delegate. For now, this is a no-op.
    // Future: Use objc2 crate to call [[NSRunningApplication currentApplication] activateWithOptions:]
}

/// Enable dark mode for system UI elements.
/// macOS respects the system appearance automatically.
pub fn enable_dark_mode() {
    // macOS context menus automatically follow system appearance.
    // No action needed.
}

/// Reduce memory footprint.
/// TODO: Could potentially use madvise or similar.
pub fn trim_memory() {
    // macOS doesn't have a direct equivalent to EmptyWorkingSet.
    // The system manages memory pressure automatically.
    // Could potentially use jemalloc's purge or madvise(MADV_FREE).
}
