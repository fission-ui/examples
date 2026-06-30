use crate::data::VideoData;
use crate::state::format_count;
use crate::style::{black_alpha, color_from_hex, white_alpha};
use fission::op::{AlignItems, JustifyContent};
use fission::prelude::*;

#[fission_component]
#[derive(Clone)]
pub struct ProfileVideoTile {
    pub video: VideoData,
}

impl From<ProfileVideoTile> for Widget {
    fn from(tile: ProfileVideoTile) -> Self {
        let (_ctx, view) = fission::build::current::<crate::state::TikTokState>();
        let tokens = &view.env().theme.tokens;
        let accent = color_from_hex(&tile.video.user.avatar_color, tokens.colors.primary);
        let id = WidgetId::explicit(&format!("profile.tile.{}", tile.video.id));

        Container::new(fission::widgets::ZStack {
            children: vec![
                Container::default()
                    .width(120.0)
                    .height(166.0)
                    .bg(accent)
                    .border_radius(8.0)
                    .into(),
                fission::widgets::Positioned {
                    bottom: Some(8.0),
                    left: Some(8.0),
                    right: Some(8.0),
                    child: Some(
                        fission::core::ui::Row {
                            children: vec![
                                fission::core::ui::Text::new("▶")
                                    .size(11.0)
                                    .color(tokens.colors.text_primary)
                                    .into(),
                                fission::core::ui::Text::new(format_count(tile.video.views))
                                    .size(12.0)
                                    .weight(tokens.typography.font_weight_bold)
                                    .color(tokens.colors.text_primary)
                                    .into(),
                            ],
                            gap: Some(4.0),
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::Start,
                            ..Default::default()
                        }
                        .into(),
                    ),
                    ..Default::default()
                }
                .into(),
                fission::widgets::Positioned {
                    top: Some(8.0),
                    right: Some(8.0),
                    child: Some(
                        Container::new(
                            fission::core::ui::Text::new(format!(
                                "{}s",
                                tile.video.duration_seconds
                            ))
                            .size(11.0)
                            .color(tokens.colors.text_primary),
                        )
                        .bg(black_alpha(80))
                        .border_radius(8.0)
                        .padding([6.0, 6.0, 3.0, 3.0])
                        .into(),
                    ),
                    ..Default::default()
                }
                .into(),
            ],
            ..Default::default()
        })
        .width(120.0)
        .height(166.0)
        .border(white_alpha(24), 1.0)
        .border_radius(8.0)
        .id(id)
    }
}
