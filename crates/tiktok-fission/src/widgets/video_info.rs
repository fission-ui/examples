use fission::prelude::*;
use fission::op::AlignItems;
use crate::data::VideoData;
use crate::ui_helpers::*;

pub fn build_video_info(video: &VideoData) -> Widget {
    Container::new(
        Column {
            children: vec![
                // @username — bold white
                text_bold(&format!("@{}", video.user.name), 16.0, WHITE).into(),
                Container::default().height(5.0).into(),
                // Caption — max 2 lines
                {
                    let mut t = text(&video.caption, 14.0, WHITE);
                    t.max_lines = Some(2);
                    t.into()
                },
                Container::default().height(10.0).into(),
                // Music row — ♫ + track name
                Row {
                    children: vec![
                        text("♫  ", 13.0, WHITE).into(),
                        {
                            let name = if video.music.len() > 28 {
                                format!("{}...", &video.music[..25])
                            } else {
                                video.music.clone()
                            };
                            text(&name, 13.0, WHITE).into()
                        },
                    ],
                    ..Default::default()
                }.into(),
            ],
            align_items: AlignItems::Start,
            ..Default::default()
        }
    )
    .width(260.0)
    .into()
}
