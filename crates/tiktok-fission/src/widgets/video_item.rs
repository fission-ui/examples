use fission::prelude::*;

pub fn build_video_item(video_file: &str, is_playing: bool) -> Widget {
    let video_id = WidgetId::explicit(video_file);
    let source = format!("assets/videos/{}", video_file);

    // Video fills the entire screen — no explicit size constraints
    // The ZStack parent will stretch it to fill available space
    Video {
        id: Some(video_id),
        source,
        width: None,
        height: None,
        autoplay: is_playing,
        loop_playback: true,
    }.into()
}
