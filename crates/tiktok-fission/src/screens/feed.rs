use fission::prelude::*;
use fission::op::{AlignItems, JustifyContent};
use fission::core::StateField;
use crate::state::TikTokState;
use crate::ui_helpers::*;
use crate::widgets::{action_sidebar, video_info, video_item};
use crate::app;

pub fn build_feed(ctx: &BuildCtxHandle<()>, state_handle: &StateField<TikTokState>) -> Widget {
    let state = state_handle.get();

    let video = match state.current_video() {
        Some(v) => v.clone(),
        None => {
            return Container::new(text("Loading...", 18.0, WHITE))
                .bg(BLACK)
                .into();
        }
    };

    let is_liked = state.is_liked(&video.id);
    let display_likes = state.display_likes(&video);

    // ─── Layer 0: Full-screen video (edge-to-edge) ───────────────────────────
    let video_layer = video_item::build_video_item(&video.video_file, state.is_playing);

    // ─── Layer 1: Top bar — "Following | For You", transparent bg ────────────
    let top_bar = pos_top_bar(0.0, build_top_bar());

    // ─── Layer 2: Bottom-left — @username, caption, ♫ music ─────────────────
    let info = pos_bl(12.0, 110.0, video_info::build_video_info(&video));

    // ─── Layer 3: Bottom-right — avatar, like, comment, share, disc ─────────
    let sidebar = pos_tr(8.0, 260.0, action_sidebar::build_action_sidebar(
        &video,
        is_liked,
        display_likes,
        app::toggle_like_action(ctx, state_handle, video.id.clone()),
    ));

    // ─── Layer 4: Pause indicator (centre, only when paused) ─────────────────
    let pause_indicator: Widget = if !state.is_playing {
        pos_fill(
            Container::new(
                Container::new(text("▶", 40.0, WHITE))
                    .bg(rgba(0, 0, 0, 120))
                    .border_radius(40.0)
                    .padding_all(20.0)
            )
            .flex_grow(1.0)
        ).into()
    } else {
        Container::default().into()
    };

    // ─── Layer 5: Gesture detector (covers entire screen for swipe/tap) ──────
    let gesture_layer = pos_fill({
        let w: Widget = GestureDetector {
            on_drag_start:  Some(app::drag_start_action(ctx, state_handle)),
            on_drag_update: Some(app::drag_update_action(ctx, state_handle)),
            on_drag_end:    Some(app::drag_end_action(ctx, state_handle)),
            on_tap:         Some(app::toggle_play_pause_action(ctx, state_handle)),
            child: Container::default().bg(TRANSPARENT).flex_grow(1.0).into(),
            ..Default::default()
        }.into();
        w
    });

    // ─── Compose all layers ──────────────────────────────────────────────────
    Container::new(
        ZStack {
            children: vec![
                // Black background layer (FIRST to ensure it's behind everything)
                Container::default().bg(BLACK).into(),
                video_layer,
                top_bar.into(),
                info.into(),
                sidebar.into(),
                pause_indicator,
                gesture_layer.into(),
            ],
            ..Default::default()
        }
    )
    .bg(BLACK)
    .into()
}

/// Top bar: "Following | For You" centred, with underline on active tab
fn build_top_bar() -> Widget {
    Column {
        children: vec![
            // Status bar spacer (safe area)
            Container::default().height(48.0).into(),
            // Tab row
            Row {
                children: vec![
                    tab_label("Following", false),
                    Container::default().width(28.0).into(),
                    tab_label("For You", true),
                ],
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            }.into(),
        ],
        align_items: AlignItems::Center,
        ..Default::default()
    }.into()
}

fn tab_label(label: &str, active: bool) -> Widget {
    let color = if active { WHITE } else { rgba(255, 255, 255, 160) };
    let weight: u16 = if active { 700 } else { 400 };

    Column {
        children: vec![
            text_weight(label, 17.0, color, weight).into(),
            if active {
                Container::default().bg(WHITE).width(28.0).height(2.0).into()
            } else {
                Container::default().height(2.0).into()
            },
        ],
        align_items: AlignItems::Center,
        ..Default::default()
    }.into()
}
