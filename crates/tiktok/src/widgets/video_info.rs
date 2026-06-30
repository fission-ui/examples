use crate::data::VideoData;
use crate::style::{black_alpha, tiktok_cyan};
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
        let sponsored = view
            .env()
            .i18n
            .get(&view.env().locale, "feed.sponsored")
            .unwrap_or("Sponsored");
        let hashtags = info.video.hashtags.join(" ");

        Container::new(fission::core::ui::Column {
            children: vec![
                fission::core::ui::Row {
                    children: vec![
                        fission::core::ui::Row {
                            children: vec![
                                fission::core::ui::Text::new(format!(
                                    "@{}",
                                    info.video.user.handle
                                ))
                                .size(16.0)
                                .weight(tokens.typography.font_weight_bold)
                                .color(tokens.colors.text_primary)
                                .into(),
                                if info.video.user.verified {
                                    crate::widgets::AppIcon {
                                        svg: fission::icons::material::action::verified::round(),
                                        size: 15.0,
                                        color: tiktok_cyan(),
                                    }
                                    .into()
                                } else {
                                    Container::default().into()
                                },
                            ],
                            gap: Some(4.0),
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::Start,
                            ..Default::default()
                        }
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
                fission::core::ui::Text::new(hashtags)
                    .size(13.0)
                    .weight(tokens.typography.font_weight_bold)
                    .color(tokens.colors.text_primary)
                    .max_lines(2)
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
