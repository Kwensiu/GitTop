#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

//! GitTop - A beautiful native GitHub notification manager
//! No browser engine required. Pure Rust. Pure performance.

mod github;
mod settings;
mod ui;

use iced::{application, Font, Size};
use ui::App;

fn main() -> iced::Result {
    application(App::new, App::update, App::view)
        .title(|app: &App| app.title())
        .theme(|app: &App| app.theme())
        .subscription(App::subscription)
        .window_size(Size::new(420.0, 640.0))
        .antialiasing(true)
        .default_font(Font::DEFAULT)
        .run()
}
