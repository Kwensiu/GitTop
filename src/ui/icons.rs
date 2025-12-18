//! Centralized icon helpers with theme support.
//!
//! Supports both SVG icons (Lucide) and emoji icons based on user preference.

use iced::widget::{svg, text, Svg, Text};
use iced::{Color, Element};
use icondata_core::IconData;

use crate::settings::IconTheme;

/// Convert IconData to SVG bytes by building the XML string.
fn icon_to_svg_bytes(data: &IconData) -> Vec<u8> {
    let mut svg_str = String::from(r#"<svg xmlns="http://www.w3.org/2000/svg""#);

    if let Some(w) = data.width {
        svg_str.push_str(&format!(r#" width="{}""#, w));
    }
    if let Some(h) = data.height {
        svg_str.push_str(&format!(r#" height="{}""#, h));
    }
    if let Some(vb) = data.view_box {
        svg_str.push_str(&format!(r#" viewBox="{}""#, vb));
    }
    if let Some(fill) = data.fill {
        svg_str.push_str(&format!(r#" fill="{}""#, fill));
    }
    if let Some(stroke) = data.stroke {
        svg_str.push_str(&format!(r#" stroke="{}""#, stroke));
    }
    if let Some(stroke_width) = data.stroke_width {
        svg_str.push_str(&format!(r#" stroke-width="{}""#, stroke_width));
    }
    if let Some(stroke_linecap) = data.stroke_linecap {
        svg_str.push_str(&format!(r#" stroke-linecap="{}""#, stroke_linecap));
    }
    if let Some(stroke_linejoin) = data.stroke_linejoin {
        svg_str.push_str(&format!(r#" stroke-linejoin="{}""#, stroke_linejoin));
    }
    if let Some(style) = data.style {
        svg_str.push_str(&format!(r#" style="{}""#, style));
    }

    svg_str.push('>');
    svg_str.push_str(data.data);
    svg_str.push_str("</svg>");

    svg_str.into_bytes()
}

/// Create an SVG icon from IconData with the specified size.
pub fn icon(data: &IconData, size: f32) -> Svg<'static> {
    let bytes = icon_to_svg_bytes(data);
    svg(svg::Handle::from_memory(bytes))
        .width(size)
        .height(size)
}

/// Create a colored SVG icon.
pub fn icon_colored(data: &IconData, size: f32, color: Color) -> Svg<'static> {
    let bytes = icon_to_svg_bytes(data);
    svg(svg::Handle::from_memory(bytes))
        .width(size)
        .height(size)
        .style(move |_, _| svg::Style { color: Some(color) })
}

/// Create an emoji text icon.
fn emoji_icon(emoji: &'static str, size: u32, color: Color) -> Text<'static> {
    text(emoji).size(size).color(color)
}

// =============================================================================
// THEME-AWARE ICON FUNCTIONS
// These return Element<M> so they can be either SVG or Text based on theme.
// =============================================================================

/// App branding icon (diamond).
pub fn icon_brand<M: 'static>(size: f32, color: Color, theme: IconTheme) -> Element<'static, M> {
    match theme {
        IconTheme::Svg => icon_colored(&icondata_lu::LuDiamond, size, color).into(),
        IconTheme::Emoji => emoji_icon("◆", size as u32, color).into(),
    }
}

/// User/profile icon.
pub fn icon_user<M: 'static>(size: f32, color: Color, theme: IconTheme) -> Element<'static, M> {
    match theme {
        IconTheme::Svg => icon_colored(&icondata_lu::LuUser, size, color).into(),
        IconTheme::Emoji => emoji_icon("👤", size as u32, color).into(),
    }
}

/// Power/logout icon.
pub fn icon_power<M: 'static>(size: f32, color: Color, theme: IconTheme) -> Element<'static, M> {
    match theme {
        IconTheme::Svg => icon_colored(&icondata_lu::LuPower, size, color).into(),
        IconTheme::Emoji => emoji_icon("⏻", size as u32, color).into(),
    }
}

/// Refresh icon.
pub fn icon_refresh<M: 'static>(size: f32, color: Color, theme: IconTheme) -> Element<'static, M> {
    match theme {
        IconTheme::Svg => icon_colored(&icondata_lu::LuRefreshCw, size, color).into(),
        IconTheme::Emoji => emoji_icon("↻", size as u32, color).into(),
    }
}

/// Check/success icon.
pub fn icon_check<M: 'static>(size: f32, color: Color, theme: IconTheme) -> Element<'static, M> {
    match theme {
        IconTheme::Svg => icon_colored(&icondata_lu::LuCheck, size, color).into(),
        IconTheme::Emoji => emoji_icon("✓", size as u32, color).into(),
    }
}

/// Alert/warning icon.
pub fn icon_alert<M: 'static>(size: f32, color: Color, theme: IconTheme) -> Element<'static, M> {
    match theme {
        IconTheme::Svg => icon_colored(&icondata_lu::LuTriangleAlert, size, color).into(),
        IconTheme::Emoji => emoji_icon("⚠", size as u32, color).into(),
    }
}

/// Inbox/all icon.
pub fn icon_inbox<M: 'static>(size: f32, color: Color, theme: IconTheme) -> Element<'static, M> {
    match theme {
        IconTheme::Svg => icon_colored(&icondata_lu::LuInbox, size, color).into(),
        IconTheme::Emoji => emoji_icon("📋", size as u32, color).into(),
    }
}

/// Folder/repository icon.
pub fn icon_folder<M: 'static>(size: f32, color: Color, theme: IconTheme) -> Element<'static, M> {
    match theme {
        IconTheme::Svg => icon_colored(&icondata_lu::LuFolder, size, color).into(),
        IconTheme::Emoji => emoji_icon("📁", size as u32, color).into(),
    }
}

/// Issue icon (circle dot).
pub fn icon_issue<M: 'static>(size: f32, color: Color, theme: IconTheme) -> Element<'static, M> {
    match theme {
        IconTheme::Svg => icon_colored(&icondata_lu::LuCircleDot, size, color).into(),
        IconTheme::Emoji => emoji_icon("●", size as u32, color).into(),
    }
}

/// Pull request icon.
pub fn icon_pull_request<M: 'static>(
    size: f32,
    color: Color,
    theme: IconTheme,
) -> Element<'static, M> {
    match theme {
        IconTheme::Svg => icon_colored(&icondata_lu::LuGitPullRequest, size, color).into(),
        IconTheme::Emoji => emoji_icon("⇄", size as u32, color).into(),
    }
}

/// Release/tag icon.
pub fn icon_release<M: 'static>(size: f32, color: Color, theme: IconTheme) -> Element<'static, M> {
    match theme {
        IconTheme::Svg => icon_colored(&icondata_lu::LuTag, size, color).into(),
        IconTheme::Emoji => emoji_icon("◆", size as u32, color).into(),
    }
}

/// Discussion icon.
pub fn icon_discussion<M: 'static>(
    size: f32,
    color: Color,
    theme: IconTheme,
) -> Element<'static, M> {
    match theme {
        IconTheme::Svg => icon_colored(&icondata_lu::LuMessageCircle, size, color).into(),
        IconTheme::Emoji => emoji_icon("💬", size as u32, color).into(),
    }
}

/// CI/workflow check icon.
pub fn icon_check_suite<M: 'static>(
    size: f32,
    color: Color,
    theme: IconTheme,
) -> Element<'static, M> {
    match theme {
        IconTheme::Svg => icon_colored(&icondata_lu::LuCircleCheck, size, color).into(),
        IconTheme::Emoji => emoji_icon("✓", size as u32, color).into(),
    }
}

/// Commit icon.
pub fn icon_commit<M: 'static>(size: f32, color: Color, theme: IconTheme) -> Element<'static, M> {
    match theme {
        IconTheme::Svg => icon_colored(&icondata_lu::LuGitCommitHorizontal, size, color).into(),
        IconTheme::Emoji => emoji_icon("◉", size as u32, color).into(),
    }
}

/// Security/vulnerability icon.
pub fn icon_security<M: 'static>(size: f32, color: Color, theme: IconTheme) -> Element<'static, M> {
    match theme {
        IconTheme::Svg => icon_colored(&icondata_lu::LuShieldAlert, size, color).into(),
        IconTheme::Emoji => emoji_icon("⚠", size as u32, color).into(),
    }
}

/// Unknown/generic icon.
pub fn icon_unknown<M: 'static>(size: f32, color: Color, theme: IconTheme) -> Element<'static, M> {
    match theme {
        IconTheme::Svg => icon_colored(&icondata_lu::LuCircle, size, color).into(),
        IconTheme::Emoji => emoji_icon("○", size as u32, color).into(),
    }
}

/// Circle check/success icon with fill.
pub fn icon_circle_check<M: 'static>(
    size: f32,
    color: Color,
    theme: IconTheme,
) -> Element<'static, M> {
    match theme {
        IconTheme::Svg => icon_colored(&icondata_lu::LuCircleCheck, size, color).into(),
        IconTheme::Emoji => emoji_icon("✓", size as u32, color).into(),
    }
}

/// Settings gear icon.
pub fn icon_settings<M: 'static>(size: f32, color: Color, theme: IconTheme) -> Element<'static, M> {
    match theme {
        IconTheme::Svg => icon_colored(&icondata_lu::LuSettings, size, color).into(),
        IconTheme::Emoji => emoji_icon("⚙", size as u32, color).into(),
    }
}

/// Chevron down icon.
pub fn icon_chevron_down<M: 'static>(
    size: f32,
    color: Color,
    theme: IconTheme,
) -> Element<'static, M> {
    match theme {
        IconTheme::Svg => icon_colored(&icondata_lu::LuChevronDown, size, color).into(),
        IconTheme::Emoji => emoji_icon("▼", size as u32, color).into(),
    }
}

/// Chevron right icon.
pub fn icon_chevron_right<M: 'static>(
    size: f32,
    color: Color,
    theme: IconTheme,
) -> Element<'static, M> {
    match theme {
        IconTheme::Svg => icon_colored(&icondata_lu::LuChevronRight, size, color).into(),
        IconTheme::Emoji => emoji_icon("▶", size as u32, color).into(),
    }
}

/// Chevron left icon.
pub fn icon_chevron_left<M: 'static>(
    size: f32,
    color: Color,
    theme: IconTheme,
) -> Element<'static, M> {
    match theme {
        IconTheme::Svg => icon_colored(&icondata_lu::LuChevronLeft, size, color).into(),
        IconTheme::Emoji => emoji_icon("◀", size as u32, color).into(),
    }
}

/// Trash icon.
pub fn icon_trash<M: 'static>(size: f32, color: Color, theme: IconTheme) -> Element<'static, M> {
    match theme {
        IconTheme::Svg => icon_colored(&icondata_lu::LuTrash2, size, color).into(),
        IconTheme::Emoji => emoji_icon("🗑", size as u32, color).into(),
    }
}
