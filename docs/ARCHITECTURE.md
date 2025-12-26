# Architecture & Platform Specifics

GitTop tries to feel native everywhere. To do that, we sometimes have to do things differently on each OS. All that messy logic is hidden inside `src/platform/`.

## How the App Runs (`run_app`)

The core application loop changes depending on where you are.

| OS | Runner Mode | Why? |
|----|-------------|------|
| **Windows / macOS** | `iced::application` | Standard desktop app. We can just hide the window when minimizing to tray. |
| **Linux** | `iced::daemon` | Wayland makes "hiding" windows complicated. |

### The Linux Wayland Situation

On Linux (especially Wayland), you can't reliably "hide" a window. If you try, it might just minimize or stay visible.

So, on Linux we use `iced::daemon`. This lets the process run without *any* window open.
*   **Minimize to Tray**: We actually `close()` (destroy) the window.
*   **Open from Tray**: We `open()` (create) a fresh window.

It sounds drastic, but it's the correct way to handle tray-only apps on modern Linux compositors.

## Desktop Integration

### System Tray

We use the `tray-icon` crate to sit in your status bar.

*   **Windows**: Standard Win32 tray icon.
*   **Linux**: Uses `libayatana` or AppIndicator. We have to initialize GTK first.
*   **macOS**: Native status item.

### Notifications

We use the native notification systems so GitTop feels integrated.

*   **Windows**: WinRT Toasts (via `tauri-winrt-notification`).
*   **Linux**: DBus notifications (via `notify-rust`).
*   **macOS**: Native Notification Center.

### Memory Optimization

We are aggressive about memory usage. When you minimize the app to the tray, we ask the OS to reclaim unused memory.

*   **Windows**: Calls `EmptyWorkingSet()` to trim the working set.
*   **Linux (glibc)**: Calls `malloc_trim()` to release heap memory back to the OS.

### Dark Mode

*   **Windows**: We call the undocumented `SetPreferredAppMode` API in `uxtheme.dll` to force the window frame to match your system theme.
*   **Linux / macOS**: Usually "Just Works" by following the system GTK/Qt theme.
