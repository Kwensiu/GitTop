//! Minimal theme for GitTop - optimized for performance.
//!
//! Design principles:
//! - All style functions are simple and avoid allocations
//! - Colors are defined as constants
//! - Minimal branching in style functions

use iced::widget::{button, container, scrollable, text, text_input};
use iced::{Background, Border, Color, Theme};

// ============================================================================
// COLOR CONSTANTS - Teams Dark palette
// ============================================================================

/// Background colors
pub const BG_BASE: Color = Color::from_rgb(0.16, 0.16, 0.16); // #292929
pub const BG_CARD: Color = Color::from_rgb(0.20, 0.20, 0.20); // #333333
pub const BG_CONTROL: Color = Color::from_rgb(0.24, 0.24, 0.24); // #3D3D3D
pub const BG_HOVER: Color = Color::from_rgb(0.28, 0.28, 0.28); // #474747

/// Text colors
pub const TEXT_PRIMARY: Color = Color::WHITE;
pub const TEXT_SECONDARY: Color = Color::from_rgb(0.75, 0.75, 0.75);
pub const TEXT_MUTED: Color = Color::from_rgb(0.55, 0.55, 0.55);

/// Accent colors
pub const ACCENT_BLUE: Color = Color::from_rgb(0.38, 0.80, 1.0); // #60CDFF
pub const ACCENT_GREEN: Color = Color::from_rgb(0.42, 0.80, 0.37);
pub const ACCENT_ORANGE: Color = Color::from_rgb(0.97, 0.39, 0.05);
pub const ACCENT_RED: Color = Color::from_rgb(0.91, 0.07, 0.14);
pub const ACCENT_PURPLE: Color = Color::from_rgb(0.78, 0.69, 0.87);

/// Border color
pub const BORDER: Color = Color::from_rgb(0.27, 0.27, 0.27);

// ============================================================================
// CONTAINER STYLES
// ============================================================================

pub fn app_container(_: &Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(BG_BASE)),
        ..Default::default()
    }
}

pub fn header(_: &Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(BG_CARD)),
        ..Default::default()
    }
}

// ============================================================================
// BUTTON STYLES
// ============================================================================

pub fn primary_button(_: &Theme, status: button::Status) -> button::Style {
    let bg = match status {
        button::Status::Hovered => Color::from_rgb(0.30, 0.75, 0.95),
        button::Status::Pressed => Color::from_rgb(0.25, 0.70, 0.90),
        _ => ACCENT_BLUE,
    };
    button::Style {
        background: Some(Background::Color(bg)),
        text_color: Color::BLACK,
        border: Border {
            radius: 4.0.into(),
            ..Default::default()
        },
        ..Default::default()
    }
}

pub fn ghost_button(_: &Theme, status: button::Status) -> button::Style {
    let (bg, text) = match status {
        button::Status::Hovered => (BG_HOVER, TEXT_PRIMARY),
        button::Status::Pressed => (BG_CONTROL, TEXT_SECONDARY),
        _ => (Color::TRANSPARENT, TEXT_PRIMARY),
    };
    button::Style {
        background: Some(Background::Color(bg)),
        text_color: text,
        border: Border {
            radius: 4.0.into(),
            ..Default::default()
        },
        ..Default::default()
    }
}

pub fn secondary_button(_: &Theme, status: button::Status) -> button::Style {
    let bg = match status {
        button::Status::Hovered => BG_HOVER,
        button::Status::Pressed => BG_CONTROL,
        _ => BG_CONTROL,
    };
    button::Style {
        background: Some(Background::Color(bg)),
        text_color: TEXT_PRIMARY,
        border: Border {
            color: BORDER,
            width: 1.0,
            radius: 4.0.into(),
        },
        ..Default::default()
    }
}

pub fn notification_button(_: &Theme, status: button::Status) -> button::Style {
    let bg = match status {
        button::Status::Hovered => BG_HOVER,
        button::Status::Pressed => BG_CONTROL,
        _ => Color::TRANSPARENT,
    };
    button::Style {
        background: Some(Background::Color(bg)),
        text_color: TEXT_PRIMARY,
        border: Border {
            radius: 4.0.into(),
            ..Default::default()
        },
        ..Default::default()
    }
}

// ============================================================================
// TEXT STYLES
// ============================================================================

pub fn muted_text(_: &Theme) -> text::Style {
    text::Style {
        color: Some(TEXT_MUTED),
    }
}

pub fn secondary_text(_: &Theme) -> text::Style {
    text::Style {
        color: Some(TEXT_SECONDARY),
    }
}

// ============================================================================
// TEXT INPUT STYLE
// ============================================================================

pub fn text_input_style(_: &Theme, status: text_input::Status) -> text_input::Style {
    let (bg, border_color, border_width) = match status {
        text_input::Status::Focused { .. } => (BG_BASE, ACCENT_BLUE, 2.0),
        text_input::Status::Hovered => (BG_HOVER, BORDER, 1.0),
        _ => (BG_CONTROL, BORDER, 1.0),
    };
    text_input::Style {
        background: Background::Color(bg),
        border: Border {
            color: border_color,
            width: border_width,
            radius: 4.0.into(),
        },
        icon: TEXT_MUTED,
        placeholder: TEXT_MUTED,
        value: TEXT_PRIMARY,
        selection: ACCENT_BLUE,
    }
}

// ============================================================================
// SCROLLBAR STYLE - Keep this as simple as possible
// ============================================================================

pub fn scrollbar(_: &Theme, _status: scrollable::Status) -> scrollable::Style {
    let scroller = scrollable::Scroller {
        background: Background::Color(Color::from_rgb(0.4, 0.4, 0.4)),
        border: Border {
            width: 0.0,
            color: Color::TRANSPARENT,
            radius: 4.0.into(),
        },
    };

    let rail = scrollable::Rail {
        background: None,
        border: Border::default(),
        scroller,
    };

    scrollable::Style {
        vertical_rail: rail,
        horizontal_rail: rail,
        container: container::Style::default(),
        gap: None,
        auto_scroll: scrollable::AutoScroll {
            background: Background::Color(Color::TRANSPARENT),
            border: Border::default(),
            shadow: iced::Shadow::default(),
            icon: Color::BLACK,
        },
    }
}
