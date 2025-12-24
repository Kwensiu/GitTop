//! System tray management for GitTop.

use std::sync::OnceLock;
use tray_icon::{
    Icon, TrayIcon, TrayIconBuilder, TrayIconEvent,
    menu::{Menu, MenuEvent, MenuId, MenuItem},
};

static MENU_IDS: OnceLock<MenuIds> = OnceLock::new();

#[derive(Debug)]
struct MenuIds {
    show: MenuId,
    quit: MenuId,
}

#[derive(Debug, Clone)]
pub enum TrayCommand {
    ShowWindow,
    Quit,
}

pub struct TrayManager {
    #[allow(dead_code)]
    tray: TrayIcon,
}

impl TrayManager {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let show_item = MenuItem::new("Show GitTop", true, None);
        let quit_item = MenuItem::new("Quit", true, None);

        MENU_IDS
            .set(MenuIds {
                show: show_item.id().clone(),
                quit: quit_item.id().clone(),
            })
            .expect("TrayManager initialized twice");

        let menu = Menu::new();
        menu.append(&show_item)?;
        menu.append(&quit_item)?;

        let icon = Self::create_icon()?;

        let tray = TrayIconBuilder::new()
            .with_menu(Box::new(menu))
            .with_tooltip("GitTop - GitHub Notifications")
            .with_icon(icon)
            .build()?;

        Ok(Self { tray })
    }

    fn create_icon() -> Result<Icon, Box<dyn std::error::Error>> {
        const SIZE: u32 = 32;
        const CENTER: i32 = (SIZE / 2) as i32;
        const RADIUS: i32 = CENTER - 2;
        const COLOR: [u8; 4] = [100, 149, 237, 255]; // Cornflower blue

        let mut rgba = vec![0u8; (SIZE * SIZE * 4) as usize];

        for (i, pixel) in rgba.chunks_exact_mut(4).enumerate() {
            let x = (i % SIZE as usize) as i32;
            let y = (i / SIZE as usize) as i32;

            if (x - CENTER).abs() + (y - CENTER).abs() <= RADIUS {
                pixel.copy_from_slice(&COLOR);
            }
        }

        Icon::from_rgba(rgba, SIZE, SIZE).map_err(Into::into)
    }

    pub fn poll_global_events() -> Option<TrayCommand> {
        let command = Self::poll_menu_events();
        Self::drain_tray_icon_events();
        command
    }

    fn poll_menu_events() -> Option<TrayCommand> {
        let event = MenuEvent::receiver().try_recv().ok()?;
        let ids = MENU_IDS.get()?;

        [
            (&ids.show, TrayCommand::ShowWindow),
            (&ids.quit, TrayCommand::Quit),
        ]
        .into_iter()
        .find_map(|(id, cmd)| (event.id == *id).then_some(cmd))
    }

    fn drain_tray_icon_events() {
        while let Ok(event) = TrayIconEvent::receiver().try_recv() {
            if matches!(event, TrayIconEvent::Leave { .. }) {
                crate::platform::trim_memory();
            }
        }
    }
}
