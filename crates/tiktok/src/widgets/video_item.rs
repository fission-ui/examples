use fission::op::ImageFit;
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
        let poster_id = WidgetId::explicit(&format!("video.poster.{}", item.id));
        let source = crate::data::poster_asset_path(&item.video_file);
        let mut poster = Image::file(source).size(800.0, 600.0).fit(ImageFit::Cover);
        poster.id = Some(poster_id);

        poster.into()
    }
}
