use crate::data::{NotificationData, TrendData, VideoData};
use fission::i18n::Locale;
use fission::prelude::*;
use fission::theme::DesignMode;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Action, Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct SetCurrentPath(pub String);

pub fn reduce_set_current_path(
    state: &mut TikTokState,
    action: SetCurrentPath,
    _ctx: &mut ReducerContext<TikTokState>,
) {
    state.current_path = action.0;
}

#[derive(Action, Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ToggleLike(pub String);

pub fn reduce_toggle_like(
    state: &mut TikTokState,
    action: ToggleLike,
    _ctx: &mut ReducerContext<TikTokState>,
) {
    if state.liked_videos.contains(&action.0) {
        state.liked_videos.remove(&action.0);
    } else {
        state.liked_videos.insert(action.0);
    }
}

#[derive(Action, Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ToggleSave(pub String);

pub fn reduce_toggle_save(
    state: &mut TikTokState,
    action: ToggleSave,
    _ctx: &mut ReducerContext<TikTokState>,
) {
    if state.saved_videos.contains(&action.0) {
        state.saved_videos.remove(&action.0);
    } else {
        state.saved_videos.insert(action.0);
    }
}

#[derive(Action, Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ToggleFollow(pub String);

pub fn reduce_toggle_follow(
    state: &mut TikTokState,
    action: ToggleFollow,
    _ctx: &mut ReducerContext<TikTokState>,
) {
    if state.followed_creators.contains(&action.0) {
        state.followed_creators.remove(&action.0);
    } else {
        state.followed_creators.insert(action.0);
    }
}

#[derive(Debug, Clone)]
pub struct TikTokState {
    pub current_path: String,
    pub videos: Vec<VideoData>,
    pub trends: Vec<TrendData>,
    pub notifications: Vec<NotificationData>,
    pub liked_videos: HashSet<String>,
    pub saved_videos: HashSet<String>,
    pub followed_creators: HashSet<String>,
    pub is_loaded: bool,
    pub locale: Locale,
    pub theme_mode: DesignMode,
}

impl Default for TikTokState {
    fn default() -> Self {
        let feed = crate::data::load_feed();
        Self {
            current_path: "/".to_string(),
            videos: feed.videos,
            trends: feed.trends,
            notifications: feed.notifications,
            liked_videos: HashSet::new(),
            saved_videos: HashSet::new(),
            followed_creators: ["maia.moves".to_string(), "bytebites".to_string()]
                .into_iter()
                .collect(),
            is_loaded: true,
            locale: Locale::from("en-US"),
            theme_mode: DesignMode::Dark,
        }
    }
}

impl fission::GlobalState for TikTokState {}

impl TikTokState {
    pub fn is_liked(&self, video_id: &str) -> bool {
        self.liked_videos.contains(video_id)
    }

    pub fn is_saved(&self, video_id: &str) -> bool {
        self.saved_videos.contains(video_id)
    }

    pub fn is_following(&self, handle: &str) -> bool {
        self.followed_creators.contains(handle)
    }

    pub fn display_likes(&self, video: &VideoData) -> u64 {
        if self.liked_videos.contains(&video.id) {
            video.likes + 1
        } else {
            video.likes
        }
    }

    pub fn display_saves(&self, video: &VideoData) -> u64 {
        if self.saved_videos.contains(&video.id) {
            video.saves + 1
        } else {
            video.saves
        }
    }

    pub fn creator_videos(&self, handle: &str) -> Vec<VideoData> {
        self.videos
            .iter()
            .filter(|video| video.user.handle == handle)
            .cloned()
            .collect()
    }

    pub fn unread_notifications(&self) -> usize {
        self.notifications
            .iter()
            .filter(|notification| notification.unread)
            .count()
    }
}

pub fn format_count(n: u64) -> String {
    if n >= 1_000_000 {
        format!("{:.1}M", n as f64 / 1_000_000.0)
    } else if n >= 1_000 {
        format!("{:.1}K", n as f64 / 1_000.0)
    } else {
        n.to_string()
    }
}
