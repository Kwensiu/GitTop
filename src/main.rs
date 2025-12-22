// Temporarily disabled for debugging - enables console output in release builds
// #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

//! GitTop - A beautiful native GitHub notification manager
//! No browser engine required. Pure Rust. Pure performance.

mod cache;
mod github;
mod platform;

mod settings;
mod tray;
mod ui;

use iced::window::Position;
use iced::{application, Font, Point, Size};
use settings::AppSettings;
use single_instance::SingleInstance;
use ui::App;

/// Mutex name for single instance detection
const SINGLE_INSTANCE_MUTEX: &str = "GitTop-SingleInstance-Mutex-7a8b9c0d";

fn main() -> iced::Result {
    // Check for existing instance
    let instance = SingleInstance::new(SINGLE_INSTANCE_MUTEX).unwrap();

    if !instance.is_single() {
        // Another instance is running - try to focus it and exit
        platform::focus_existing_window();
        return Ok(());
    }

    // Enable dark mode for context menus
    platform::enable_dark_mode();

    // Initialize tray icon on main thread (required for macOS)
    // The tray must be kept alive for the duration of the app
    let _tray = tray::TrayManager::new().ok();

    // Load settings to restore window state
    let settings = AppSettings::load();
    let window_size = Size::new(settings.window_width, settings.window_height);
    let window_position = match (settings.window_x, settings.window_y) {
        (Some(x), Some(y)) => Position::Specific(Point::new(x as f32, y as f32)),
        _ => Position::Centered,
    };

    application(App::new, App::update, App::view)
        .title(|app: &App| app.title())
        .theme(|app: &App| app.theme())
        .subscription(App::subscription)
        .window_size(window_size)
        .position(window_position)
        .antialiasing(true)
        .default_font(Font::DEFAULT)
        .exit_on_close_request(false)
        .run()
}
