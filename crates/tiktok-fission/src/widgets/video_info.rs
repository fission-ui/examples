use fission::prelude::*;
use fission::op::AlignItems;
use crate::data::VideoData;

#[fission_component]
#[derive(Clone)]
pub struct VideoInfo {
    pub video: VideoData,
}

impl Default for VideoInfo {
    fn default() -> Self {
        panic!("VideoInfo must be initialized with data");
    }
}

impl From<VideoInfo> for Widget {
    fn from(info: VideoInfo) -> Self {
        let (_ctx, view) = fission::build::current::<crate::state::TikTokState>();
        let _tokens = &view.env().theme.tokens;

        Container::new(
            fission::core::ui::Column {
                children: vec![
                    fission::core::ui::Text::new(format!("@{}", info.video.user.name)).into(),
                    Container::default().height(5.0).into(),
                    {
                        let mut t = fission::core::ui::Text::new(info.video.caption.clone());
                        t.max_lines = Some(2);
                        t.into()
                    },
                    Container::default().height(10.0).into(),
                    fission::core::ui::Row {
                        children: vec![
                            fission::core::ui::Text::new("♫  ".to_string()).into(),
                            {
                                let name = if info.video.music.len() > 28 {
                                    format!("{}...", &info.video.music[..25])
                                } else {
                                    info.video.music.clone()
                                };
                                fission::core::ui::Text::new(name).into()
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
}
