//! Notification item widget - displays a single notification.

use iced::widget::{button, column, container, row, text, Space};
use iced::{Alignment, Color, Element, Fill};

use crate::github::types::{NotificationView, SubjectType};
use crate::settings::IconTheme;
use crate::ui::screens::notifications::NotificationMessage;
use crate::ui::{icons, theme};

/// Get color for subject type
fn get_subject_color(subject_type: SubjectType) -> Color {
    let p = theme::palette();
    match subject_type {
        SubjectType::Issue => p.accent_success,
        SubjectType::PullRequest => p.accent,
        SubjectType::Release => p.accent_purple,
        SubjectType::Discussion => p.accent,
        SubjectType::CheckSuite => p.accent_warning,
        SubjectType::RepositoryVulnerabilityAlert => p.accent_danger,
        _ => p.text_secondary,
    }
}

/// Get the icon for a subject type.
fn subject_type_icon(
    subject_type: SubjectType,
    icon_theme: IconTheme,
) -> Element<'static, NotificationMessage> {
    let color = get_subject_color(subject_type);
    let icon_size = theme::notification_scaled(14.0);
    match subject_type {
        SubjectType::Issue => icons::icon_issue(icon_size, color, icon_theme),
        SubjectType::PullRequest => icons::icon_pull_request(icon_size, color, icon_theme),
        SubjectType::Release => icons::icon_release(icon_size, color, icon_theme),
        SubjectType::Discussion => icons::icon_discussion(icon_size, color, icon_theme),
        SubjectType::CheckSuite => icons::icon_check_suite(icon_size, color, icon_theme),
        SubjectType::Commit => icons::icon_commit(icon_size, color, icon_theme),
        SubjectType::RepositoryVulnerabilityAlert => {
            icons::icon_security(icon_size, color, icon_theme)
        }
        SubjectType::Unknown => icons::icon_unknown(icon_size, color, icon_theme),
    }
}

/// Creates a notification item widget
pub fn notification_item(
    notif: &NotificationView,
    icon_theme: IconTheme,
    dense: bool,
) -> Element<'_, NotificationMessage> {
    let p = theme::palette();

    // Use subject-type color for accent bar (not uniform blue)
    let type_color = get_subject_color(notif.subject_type);
    let is_unread = notif.unread;

    // --- SIZING & SPACING ---
    let padding_y = if dense { 8.0 } else { 14.0 };
    let padding_x = if dense { 12.0 } else { 16.0 };
    let content_spacing = if dense { 2.0 } else { 6.0 };
    let row_spacing = if dense { 8.0 } else { 8.0 };

    // Title row
    // In dense mode, title is slightly smaller or just cleaner
    let title_size = theme::notification_scaled(if dense { 13.0 } else { 14.0 });
    let meta_size = theme::notification_scaled(12.0);
    let reason_size = theme::notification_scaled(11.0);

    let title = text(&notif.title).size(title_size).color(p.text_primary);

    // Meta row: icon + repo + reason
    let meta = row![
        subject_type_icon(notif.subject_type, icon_theme),
        Space::new().width(6),
        text(&notif.repo_full_name)
            .size(meta_size)
            .color(p.text_secondary),
        Space::new().width(8),
        text(notif.reason.label())
            .size(reason_size)
            .color(p.text_muted),
    ]
    .align_y(Alignment::Center);

    // Time
    let time = text(&notif.time_ago).size(meta_size).color(p.text_muted);

    // Main content layout
    let content = if dense {
        // DENSE LAYOUT: Single line if possible, or very tight
        // For now, we keep 2 rows but tighter.
        // Option: Row [Icon, Title, Spacer, Repo, Reason, Spacer, Time]
        // But titles can be long. Let's stick to 2 rows but compacted.
        row![
            column![
                // Row 1: Title + Repo + Reason (if fits?) - No, let's keep consistent structure
                // Just tighter spacing.
                row![
                    subject_type_icon(notif.subject_type, icon_theme),
                    Space::new().width(6),
                    text(&notif.title).size(title_size).color(p.text_primary),
                ]
                .align_y(Alignment::Center),
                row![
                    text(&notif.repo_full_name)
                        .size(meta_size)
                        .color(p.text_secondary),
                    Space::new().width(8),
                    text(notif.reason.label())
                        .size(reason_size)
                        .color(p.text_muted),
                ]
                .align_y(Alignment::Center)
                .padding([0, 20]) // Indent meta slightly
            ]
            .spacing(2)
            .width(Fill),
            container(time).padding([0, 8]),
        ]
        .align_y(Alignment::Center)
        .padding([padding_y, padding_x])
    } else {
        // STANDARD LAYOUT
        row![
            column![title, meta].spacing(content_spacing).width(Fill),
            container(time).padding([4, 8]),
        ]
        .spacing(row_spacing)
        .align_y(Alignment::Center)
        .padding([padding_y, padding_x])
    };

    // Wrap in button for click handling
    let item_button = button(content)
        .style(theme::notification_button)
        .on_press(NotificationMessage::Open(notif.id.clone()))
        .width(Fill);

    // Accent bar - visible for unread with type color, transparent for read
    let bar_color = if is_unread {
        type_color
    } else {
        Color::TRANSPARENT
    };

    let accent_bar = container(Space::new().width(3).height(Fill))
        .height(Fill)
        .style(move |_| container::Style {
            background: Some(iced::Background::Color(bar_color)),
            ..Default::default()
        });

    // Card styling
    let card_bg = if is_unread {
        Color::from_rgba(type_color.r, type_color.g, type_color.b, 0.05)
    } else {
        Color::TRANSPARENT
    };

    let border_color = if is_unread {
        Color::from_rgba(type_color.r, type_color.g, type_color.b, 0.12)
    } else {
        Color::TRANSPARENT
    };

    container(
        row![accent_bar, item_button]
            .spacing(0)
            .align_y(Alignment::Center),
    )
    .style(move |_| container::Style {
        background: Some(iced::Background::Color(card_bg)),
        border: iced::Border {
            radius: if dense { 0.0.into() } else { 6.0.into() }, // Square corners in dense mode? Maybe.
            color: border_color,
            width: if is_unread { 1.0 } else { 0.0 },
        },
        ..Default::default()
    })
    .width(Fill)
    .into()
}
