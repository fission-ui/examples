use serde::{Deserialize, Serialize};
use std::collections::HashSet;

use crate::data::VideoData;
use fission::prelude::*;

// ─── Actions ─────────────────────────────────────────────────────────────────

#[derive(Action, Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct SetCurrentPath(pub String);

pub fn reduce_set_current_path(state: &mut TikTokState, action: SetCurrentPath, _ctx: &mut ReducerContext<TikTokState>) {
    state.current_path = action.0;
}

#[derive(Action, Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ToggleLike(pub String);

pub fn reduce_toggle_like(state: &mut TikTokState, action: ToggleLike, _ctx: &mut ReducerContext<TikTokState>) {
    if state.liked_videos.contains(&action.0) {
        state.liked_videos.remove(&action.0);
    } else {
        state.liked_videos.insert(action.0);
    }
}

// ─── App State ───────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct TikTokState {
    pub current_path: String,
    pub videos: Vec<VideoData>,
    pub liked_videos: HashSet<String>,
    pub is_loaded: bool,
}

impl Default for TikTokState {
    fn default() -> Self {
        let feed = crate::data::load_feed();
        Self {
            current_path: "/".to_string(),
            videos: feed.videos,
            liked_videos: HashSet::new(),
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
