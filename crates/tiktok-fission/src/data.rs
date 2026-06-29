use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VideoData {
    pub id: String,
    #[serde(rename = "videoFile")]
    pub video_file: String,
    pub user: UserData,
    pub caption: String,
    pub music: String,
    pub likes: u64,
    pub comments: u64,
    pub shares: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UserData {
    pub name: String,
    #[serde(rename = "imageUrl")]
    pub image_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FeedData {
    pub videos: Vec<VideoData>,
}

/// Load video feed data from the bundled JSON file.
pub fn load_feed() -> FeedData {
    let raw = include_str!("assets/data.json");
    serde_json::from_str(raw).expect("Invalid data.json")
}

/// Get the absolute path to a local video file in assets/videos/.
/// At runtime, this resolves relative to the executable's working directory.
pub fn video_asset_path(filename: &str) -> String {
    // Use the path relative to the project root for development
    format!("src/assets/videos/{}", filename)
}
