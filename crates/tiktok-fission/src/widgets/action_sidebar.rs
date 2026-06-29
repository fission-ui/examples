use fission::prelude::*;
use fission::op::AlignItems;
use crate::data::VideoData;
use crate::state::format_count;
use crate::ui_helpers::*;

pub fn build_action_sidebar(
    video: &VideoData,
    is_liked: bool,
    display_likes: u64,
    on_like: ActionEnvelope,
) -> Widget {
    let column = Column {
        align_items: AlignItems::Center,
        children: vec![
            // ─── Avatar with red "+" follow badge ────────────────────────
            build_avatar(&video.user.name),
            Container::default().height(20.0).into(),
            // ─── Like — RED circle (always red for visibility) ───────────
            action_btn(LIKE_RED, display_likes, Some(on_like)),
            Container::default().height(20.0).into(),
            // ─── Comment — WHITE circle ──────────────────────────────────
            action_btn(WHITE, video.comments, None),
            Container::default().height(20.0).into(),
            // ─── Share — LIGHT GREY circle ───────────────────────────────
            action_btn(rgba(200, 200, 200, 255), video.shares, None),
            Container::default().height(24.0).into(),
            // ─── Music disc ──────────────────────────────────────────────
            build_disc(),
            Container::default().height(8.0).into(),
        ],
        ..Default::default()
    };

    Container {
        child: Some(column.into()),
        ..Default::default()
    }
    .width(68.0)
    .into()
}

/// Circular avatar placeholder with first-letter initial + red "+" follow badge
fn build_avatar(_username: &str) -> Widget {
    // Dark grey circle (avatar placeholder)
    Container::default()
        .bg(rgba(80, 80, 80, 255))
        .border_radius(27.0)
        .width(54.0)
        .height(54.0)
        .into()
}

/// Icon + count label, optionally tappable
fn action_btn(
    icon_color: Color,
    count: u64,
    action: Option<ActionEnvelope>,
) -> Widget {
    let icon = Container::default()
        .bg(icon_color)
        .border_radius(20.0)
        .width(40.0)
        .height(40.0)
        .into();

    let inner = Column {
        align_items: AlignItems::Center,
        children: vec![
            icon,
            Container::default().height(4.0).into(),
            text(&format_count(count), 12.0, WHITE).into(),
        ],
        ..Default::default()
    };

    match action {
        Some(a) => GestureDetector {
            on_tap: Some(a),
            child: inner.into(),
            ..Default::default()
        }.into(),
        None => inner.into(),
    }
}

/// Static spinning-style disc — concentric dark circles with music note
fn build_disc() -> Widget {
    // Simple dark circle — no Positioned children
    Container::default()
        .bg(rgba(30, 30, 30, 255))
        .border_radius(28.0)
        .width(56.0)
        .height(56.0)
        .into()
}
