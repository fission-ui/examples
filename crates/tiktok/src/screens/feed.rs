use fission::core::ui::TextContent;
use fission::prelude::*;
use serde::{Deserialize, Serialize};

use crate::state::TikTokState;
use crate::style::{black_alpha, white_alpha};
use crate::widgets::{ActionSidebar, AppIcon, CommentsSheet, FeedTopBar, VideoInfo, VideoItem};

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
    if local.drag_delta_y < -60.0 && local.current_video_index + 1 < max_len {
        local.current_video_index += 1;
        local.show_comments = false;
    } else if local.drag_delta_y > 60.0 {
        local.current_video_index = local.current_video_index.saturating_sub(1);
        local.show_comments = false;
    }

    local.drag_delta_y = 0.0;
    local.is_playing = true;
}

#[fission_reducer(TogglePlayPause)]
fn toggle_play_pause(local: &mut FeedLocalState) {
    local.is_playing = !local.is_playing;
}

#[fission_reducer(ToggleComments)]
fn toggle_comments(local: &mut FeedLocalState) {
    local.show_comments = !local.show_comments;
}

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
        let index = local.current_video_index.min(max_len.saturating_sub(1));

        let video = match state.videos.get(index) {
            Some(video) => video.clone(),
            None => {
                return Container::new(fission::core::ui::Text::new(TextContent::Key(
                    "feed.loading".into(),
                )))
                .bg(tokens.colors.background)
                .flex_grow(1.0)
                .into();
            }
        };

        let drag_start_action =
            ctx.bind_local(DragStart, local_handle.clone(), reduce!(drag_start));
        let drag_update_action =
            ctx.bind_local(DragUpdate, local_handle.clone(), reduce!(drag_update));
        let drag_end_action =
            ctx.bind_local(DragEnd(max_len), local_handle.clone(), reduce!(drag_end));
        let tap_action = ctx.bind_local(
            TogglePlayPause,
            local_handle.clone(),
            reduce!(toggle_play_pause),
        );
        let comment_action = ctx.bind_local(
            ToggleComments,
            local_handle.clone(),
            reduce!(toggle_comments),
        );
        let like_action = ctx.bind(
            crate::state::ToggleLike(video.id.clone()),
            reduce!(crate::state::reduce_toggle_like),
        );
        let save_action = ctx.bind(
            crate::state::ToggleSave(video.id.clone()),
            reduce!(crate::state::reduce_toggle_save),
        );
        let follow_action = ctx.bind(
            crate::state::ToggleFollow(video.user.handle.clone()),
            reduce!(crate::state::reduce_toggle_follow),
        );

        let mut chrome_tracks = fission::motion::fade();
        chrome_tracks.extend(fission::motion::slide_y(12.0));

        let video_layer = fission::motion::appear(
            format!("feed.video.{}", video.id),
            fission::motion::fade(),
            VideoItem {
                id: video.id.clone(),
                video_file: video.video_file.clone(),
                is_playing: local.is_playing,
            },
        );

        let gesture_layer = fission::widgets::Positioned {
            top: Some(0.0),
            bottom: Some(0.0),
            left: Some(0.0),
            right: Some(0.0),
            child: Some(
                fission::core::ui::widgets::GestureDetector {
                    on_drag_start: Some(drag_start_action),
                    on_drag_update: Some(drag_update_action),
                    on_drag_end: Some(drag_end_action),
                    on_tap: Some(tap_action),
                    child: Container::default().flex_grow(1.0).into(),
                    ..Default::default()
                }
                .into(),
            ),
            ..Default::default()
        };

        let pause_indicator = fission::motion::presence(
            format!("feed.pause.{}", video.id),
            !local.is_playing,
            vec![
                fission::motion::MotionTrack::composite(
                    fission::motion::MotionPropertyId::Scale,
                    fission::motion::MotionStartValue::Explicit(fission::motion::scalar(0.82)),
                    fission::motion::scalar(1.0),
                )
                .transition(fission::motion::MotionTransition::spring(480.0, 28.0)),
                fission::motion::MotionTrack::composite(
                    fission::motion::MotionPropertyId::Opacity,
                    fission::motion::MotionStartValue::Explicit(fission::motion::scalar(0.0)),
                    fission::motion::scalar(1.0),
                ),
            ],
            fission::widgets::Positioned {
                top: Some(0.0),
                bottom: Some(0.0),
                left: Some(0.0),
                right: Some(0.0),
                child: Some(
                    fission::widgets::Center {
                        child: Container::new(AppIcon {
                            svg: fission::icons::material::av::play_arrow::round(),
                            size: 38.0,
                            color: tokens.colors.text_primary,
                        })
                        .bg(black_alpha(120))
                        .border(white_alpha(36), 1.0)
                        .border_radius(42.0)
                        .padding_all(22.0)
                        .into(),
                    }
                    .into(),
                ),
                ..Default::default()
            },
        );

        let comments = fission::widgets::Positioned {
            bottom: Some(88.0),
            left: Some(10.0),
            right: Some(10.0),
            child: Some(fission::motion::presence(
                format!("feed.comments.{}", video.id),
                local.show_comments,
                fission::motion::slide_y(260.0),
                CommentsSheet {
                    video: video.clone(),
                    on_close: comment_action.clone(),
                },
            )),
            ..Default::default()
        };

        Container::new(fission::widgets::ZStack {
            children: vec![
                Container::default().bg(tokens.colors.background).into(),
                video_layer,
                fission::widgets::Positioned {
                    top: Some(0.0),
                    left: Some(0.0),
                    right: Some(0.0),
                    child: Some(
                        Container::default()
                            .height(106.0)
                            .bg(black_alpha(118))
                            .into(),
                    ),
                    ..Default::default()
                }
                .into(),
                fission::widgets::Positioned {
                    bottom: Some(0.0),
                    left: Some(0.0),
                    right: Some(0.0),
                    child: Some(
                        Container::default()
                            .height(248.0)
                            .bg(black_alpha(150))
                            .into(),
                    ),
                    ..Default::default()
                }
                .into(),
                gesture_layer.into(),
                fission::widgets::Positioned {
                    top: Some(0.0),
                    left: Some(0.0),
                    right: Some(0.0),
                    child: Some(FeedTopBar::default().into()),
                    ..Default::default()
                }
                .into(),
                fission::widgets::Positioned {
                    bottom: Some(112.0),
                    left: Some(12.0),
                    child: Some(fission::motion::appear(
                        format!("feed.info.{}", video.id),
                        chrome_tracks.clone(),
                        VideoInfo {
                            video: video.clone(),
                        },
                    )),
                    ..Default::default()
                }
                .into(),
                fission::widgets::Positioned {
                    bottom: Some(150.0),
                    right: Some(8.0),
                    child: Some(fission::motion::appear(
                        format!("feed.sidebar.{}", video.id),
                        chrome_tracks,
                        ActionSidebar {
                            video: video.clone(),
                            is_liked: state.is_liked(&video.id),
                            is_saved: state.is_saved(&video.id),
                            is_following: state.is_following(&video.user.handle),
                            display_likes: state.display_likes(&video),
                            display_saves: state.display_saves(&video),
                            on_like: like_action,
                            on_comments: comment_action,
                            on_save: save_action,
                            on_follow: follow_action,
                        },
                    )),
                    ..Default::default()
                }
                .into(),
                pause_indicator,
                comments.into(),
            ],
            ..Default::default()
        })
        .bg(tokens.colors.background)
        .flex_grow(1.0)
        .into()
    }
}
