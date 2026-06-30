use fission::prelude::*;

#[fission_component]
#[derive(Clone)]
pub struct VideoItem {
    pub id: String,
    pub video_file: String,
    pub is_playing: bool,
}

impl Default for VideoItem {
    fn default() -> Self {
        panic!("VideoItem must be initialized with data");
    }
}

impl From<VideoItem> for Widget {
    fn from(item: VideoItem) -> Self {
        let video_id = WidgetId::explicit(&format!("video.surface.{}", item.id));
        let source = crate::data::video_asset_path(&item.video_file);

        Video {
            id: Some(video_id),
            source,
            width: None,
            height: None,
            autoplay: item.is_playing,
            loop_playback: true,
        }
        .into()
    }
}
