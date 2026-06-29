use fission::prelude::*;
use fission::op::{AlignItems, JustifyContent};
use serde::{Deserialize, Serialize};

use crate::state::TikTokState;
use crate::widgets::{ActionSidebar, VideoInfo, VideoItem};
use crate::app;

// ─── Local State ─────────────────────────────────────────────────────────────

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FeedLocalState {
    pub current_video_index: usize,
    pub is_playing: bool,
    pub drag_delta_y: f32,
    pub show_comments: bool,
}

impl Default for FeedLocalState {
    fn default() -> Self {
        Self {
            current_video_index: 0,
            is_playing: true,
            drag_delta_y: 0.0,
            show_comments: false,
        }
    }
}

impl fission::core::GlobalState for FeedLocalState {}

// ─── Actions & Reducers ──────────────────────────────────────────────────────

#[fission_reducer(DragStart)]
fn drag_start(local: &mut FeedLocalState) {
    local.drag_delta_y = 0.0;
}

#[fission_reducer(DragUpdate)]
fn drag_update(local: &mut FeedLocalState, ctx: &mut ReducerContext<FeedLocalState>) {
    if let Some((_, _, _, dy)) = ctx.input.as_pointer() {
        local.drag_delta_y += dy;
    }
}

#[fission_reducer(DragEnd)]
fn drag_end(local: &mut FeedLocalState, max_len: usize) {
    if local.drag_delta_y < -60.0 {
        // swipe up → next video
        if local.current_video_index + 1 < max_len {
            local.current_video_index += 1;
        }
    } else if local.drag_delta_y > 60.0 {
        // swipe down → previous video
        local.current_video_index = local.current_video_index.saturating_sub(1);
    }
    local.drag_delta_y = 0.0;
    local.is_playing = true;
}

#[fission_reducer(TogglePlayPause)]
fn toggle_play_pause(local: &mut FeedLocalState) {
    local.is_playing = !local.is_playing;
}

// ─── Component ───────────────────────────────────────────────────────────────

#[fission_component]
#[derive(Clone, Default)]
pub struct FeedScreen {
    #[local_state(default = FeedLocalState::default())]
    local: FeedLocalState,
}

impl From<FeedScreen> for Widget {
    fn from(screen: FeedScreen) -> Self {
        let (ctx, view) = fission::build::current::<TikTokState>();
        let state = view.state();
        let tokens = &view.env().theme.tokens;
        
        let local_handle = screen.local();
        let local = local_handle.get();
        let max_len = state.videos.len();

        let video = match state.videos.get(local.current_video_index) {
            Some(v) => v.clone(),
            None => {
                return Container::new(fission::core::ui::Text::new("Loading..."))
                    .bg(tokens.colors.background)
                    .into();
            }
        };

        let is_liked = state.is_liked(&video.id);
        let display_likes = state.display_likes(&video);

        let drag_start_action = ctx.bind_local(DragStart, local_handle.clone(), reduce!(drag_start));
        let drag_update_action = ctx.bind_local(DragUpdate, local_handle.clone(), reduce!(drag_update));
        let drag_end_action = ctx.bind_local(DragEnd(max_len), local_handle.clone(), reduce!(drag_end));
        let tap_action = ctx.bind_local(TogglePlayPause, local_handle.clone(), reduce!(toggle_play_pause));
        
        // Use global action for like
        let like_action = ctx.bind(crate::state::ToggleLike(video.id.clone()), reduce!(crate::state::reduce_toggle_like));

        // ─── Layers ───────────────────────────────────────────────────────────────
        let video_layer = VideoItem {
            video_file: video.video_file.clone(),
            is_playing: local.is_playing,
        };

        let top_bar = fission::widgets::Positioned {
            top: Some(0.0),
            left: Some(0.0),
            right: Some(0.0),
            child: Some(build_top_bar(tokens).into()),
            ..Default::default()
        };

        let info = fission::widgets::Positioned {
            bottom: Some(110.0),
            left: Some(12.0),
            child: Some(VideoInfo { video: video.clone() }.into()),
            ..Default::default()
        };

        let sidebar = fission::widgets::Positioned {
            bottom: Some(260.0),
            right: Some(8.0),
            child: Some(ActionSidebar {
                video: video.clone(),
                is_liked,
                display_likes,
                on_like: like_action,
            }.into()),
            ..Default::default()
        };

        let pause_indicator: Widget = if !local.is_playing {
            fission::widgets::Positioned {
                top: Some(0.0), bottom: Some(0.0), left: Some(0.0), right: Some(0.0),
                child: Some(
                    fission::widgets::Center {
                        child: Container::new(fission::core::ui::Text::new("▶"))
                            .bg(fission::op::Color { r: 0, g: 0, b: 0, a: 120 })
                            .border_radius(40.0)
                            .padding_all(20.0)
                            .into()
                    }.into()
                ),
                ..Default::default()
            }.into()
        } else {
            fission::core::ui::widgets::Spacer::default().into()
        };

        let gesture_layer = fission::widgets::Positioned {
            top: Some(0.0), bottom: Some(0.0), left: Some(0.0), right: Some(0.0),
            child: Some({
                fission::core::ui::widgets::GestureDetector {
                    on_drag_start:  Some(drag_start_action),
                    on_drag_update: Some(drag_update_action),
                    on_drag_end:    Some(drag_end_action),
                    on_tap:         Some(tap_action),
                    child: Container::default().flex_grow(1.0).into(),
                    ..Default::default()
                }.into()
            }),
            ..Default::default()
        };

        Container::new(
            fission::widgets::ZStack {
                children: vec![
                    Container::default().bg(tokens.colors.background).into(),
                    video_layer.into(),
                    top_bar.into(),
                    info.into(),
                    sidebar.into(),
                    pause_indicator,
                    gesture_layer.into(),
                ],
                ..Default::default()
            }
        )
        .bg(tokens.colors.background)
        .into()
    }
}

fn build_top_bar(tokens: &fission::theme::Tokens) -> Widget {
    fission::core::ui::Column {
        children: vec![
            Container::default().height(48.0).into(),
            fission::core::ui::Row {
                children: vec![
                    tab_label("Following", false, tokens),
                    Container::default().width(28.0).into(),
                    tab_label("For You", true, tokens),
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

fn tab_label(label: &str, active: bool, tokens: &fission::theme::Tokens) -> Widget {
    let color = if active { tokens.colors.text_primary } else { tokens.colors.text_secondary };
    let weight: u16 = if active { 700 } else { 400 };

    fission::core::ui::Column {
        children: vec![
            fission::core::ui::Text::new(label.to_string())
                // .color(color) // Fission 0.5.0 Text doesn't have .color() directly maybe? Wait, let's just use Text.
                .into(),
            if active {
                Container::default().bg(tokens.colors.text_primary).width(28.0).height(2.0).into()
            } else {
                Container::default().height(2.0).into()
            },
        ],
        align_items: AlignItems::Center,
        ..Default::default()
    }.into()
}
