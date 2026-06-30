use crate::data::VideoData;
use crate::style::{black_alpha, white_alpha};
use fission::op::{AlignItems, JustifyContent};
use fission::prelude::*;

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
        let tokens = &view.env().theme.tokens;
        let verified = if info.video.user.verified { " ✓" } else { "" };
        let sponsored = view
            .env()
            .i18n
            .get(&view.env().locale, "feed.sponsored")
            .unwrap_or("Sponsored");
        let mut tag_widgets = Vec::new();
        for tag in info.video.hashtags.iter().take(3) {
            tag_widgets.push(
                Container::new(
                    fission::core::ui::Text::new(tag.clone())
                        .size(12.0)
                        .weight(tokens.typography.font_weight_bold)
                        .color(tokens.colors.text_primary),
                )
                .border_radius(12.0)
                .bg(white_alpha(24))
                .padding([8.0, 8.0, 4.0, 4.0])
                .into(),
            );
        }

        Container::new(fission::core::ui::Column {
            children: vec![
                fission::core::ui::Row {
                    children: vec![
                        fission::core::ui::Text::new(format!(
                            "@{}{}",
                            info.video.user.handle, verified
                        ))
                        .size(16.0)
                        .weight(tokens.typography.font_weight_bold)
                        .color(tokens.colors.text_primary)
                        .into(),
                        if info.video.is_sponsored {
                            Container::new(
                                fission::core::ui::Text::new(sponsored)
                                    .size(11.0)
                                    .color(tokens.colors.text_primary),
                            )
                            .bg(black_alpha(90))
                            .border_radius(10.0)
                            .padding([8.0, 8.0, 4.0, 4.0])
                            .into()
                        } else {
                            Container::default().into()
                        },
                    ],
                    gap: Some(8.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Start,
                    ..Default::default()
                }
                .into(),
                fission::core::ui::Text::new(info.video.caption.clone())
                    .size(14.0)
                    .color(tokens.colors.text_primary)
                    .max_lines(3)
                    .into(),
                fission::core::ui::Row {
                    children: tag_widgets,
                    gap: Some(6.0),
                    ..Default::default()
                }
                .into(),
                crate::widgets::SoundPill {
                    video_id: info.video.id,
                    sound: info.video.music,
                }
                .into(),
            ],
            gap: Some(8.0),
            align_items: AlignItems::Start,
            ..Default::default()
        })
        .width(278.0)
        .into()
    }
}
