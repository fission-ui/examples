use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VideoData {
    pub id: String,
    pub video_file: String,
    pub user: UserData,
    pub caption: String,
    pub hashtags: Vec<String>,
    pub music: SoundData,
    pub location: String,
    pub category: String,
    pub likes: u64,
    pub comments: u64,
    pub shares: u64,
    pub saves: u64,
    pub views: u64,
    pub duration_seconds: u32,
    pub is_sponsored: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UserData {
    pub handle: String,
    pub display_name: String,
    pub avatar_color: String,
    pub verified: bool,
    pub bio: String,
    pub followers: u64,
    pub following: u64,
    pub total_likes: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SoundData {
    pub title: String,
    pub artist: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TrendData {
    pub tag: String,
    pub title: String,
    pub subtitle: String,
    pub posts: u64,
    pub lift: String,
    pub category: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct NotificationData {
    pub id: String,
    pub actor: String,
    pub action: String,
    pub detail: String,
    pub age: String,
    pub unread: bool,
    pub avatar_color: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FeedData {
    pub videos: Vec<VideoData>,
    pub trends: Vec<TrendData>,
    pub notifications: Vec<NotificationData>,
}

pub fn load_feed() -> FeedData {
    let raw = include_str!("assets/data.json");
    serde_json::from_str(raw).expect("invalid TikTok example data")
}

pub fn video_asset_path(filename: &str) -> String {
    format!(
        "{}/src/assets/videos/{}",
        env!("CARGO_MANIFEST_DIR"),
        filename
    )
}
