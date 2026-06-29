use serde::{Deserialize, Serialize};
use std::collections::HashSet;

use crate::data::VideoData;
use fission::prelude::*;

#[derive(Action, Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DragEnd;

#[derive(Action, Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DragUpdate;

#[derive(Action, Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DragStart;

pub fn reduce_drag_start(state: &mut TikTokState, _action: DragStart) {
    state.drag_delta_y = 0.0;
}

pub fn reduce_drag_update(state: &mut TikTokState, _action: DragUpdate, ctx: &mut ReducerContext<TikTokState>) {
    if let Some((_, _, _, dy)) = ctx.input.as_pointer() {
        state.drag_delta_y += dy;
    }
}

pub fn reduce_drag_end(state: &mut TikTokState, _action: DragEnd) {
    if state.drag_delta_y < -60.0 {
        // swipe up → next video
        if state.current_video_index + 1 < state.videos.len() {
            state.current_video_index += 1;
        }
    } else if state.drag_delta_y > 60.0 {
        // swipe down → previous video
        state.current_video_index = state.current_video_index.saturating_sub(1);
    }
    state.drag_delta_y = 0.0;
    state.is_playing = true;
}

// ─── Tab Navigation ──────────────────────────────────────────────────────────

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Tab {
    #[default]
    Home,
    Discover,
    Create,
    Inbox,
    Profile,
}

// ─── App State ───────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct TikTokState {
    pub active_tab: Tab,
    pub videos: Vec<VideoData>,
    pub current_video_index: usize,
    pub is_playing: bool,
    pub liked_videos: HashSet<String>,
    pub drag_delta_y: f32,
    pub is_loaded: bool,
}

impl Default for TikTokState {
    fn default() -> Self {
        let feed = crate::data::load_feed();
        Self {
            active_tab: Tab::Home,
            videos: feed.videos,
            current_video_index: 0,
            is_playing: true,
            liked_videos: HashSet::new(),
            drag_delta_y: 0.0,
            is_loaded: true,
        }
    }
}

impl fission::GlobalState for TikTokState {}

impl TikTokState {
    /// Check if a specific video is liked
    pub fn is_liked(&self, video_id: &str) -> bool {
        self.liked_videos.contains(video_id)
    }

    /// Get the current video, if any
    pub fn current_video(&self) -> Option<&VideoData> {
        self.videos.get(self.current_video_index)
    }

    /// Get like count for a video, adjusted for local like state
    pub fn display_likes(&self, video: &VideoData) -> u64 {
        if self.liked_videos.contains(&video.id) {
            video.likes + 1
        } else {
            video.likes
        }
    }
}

// ─── Helper: Format large numbers (1200 -> "1.2K") ──────────────────────────

pub fn format_count(n: u64) -> String {
    if n >= 1_000_000 {
        format!("{:.1}M", n as f64 / 1_000_000.0)
    } else if n >= 1_000 {
        format!("{:.1}K", n as f64 / 1_000.0)
    } else {
        n.to_string()
    }
}
