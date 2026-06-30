use crate::data::VideoData;
use crate::state::format_count;
use crate::style::{black_alpha, color_from_hex, white_alpha};
use fission::op::{AlignItems, JustifyContent};
use fission::prelude::*;

#[fission_component]
#[derive(Clone)]
pub struct VideoPreviewCard {
    pub video: VideoData,
    pub rank: usize,
}

impl From<VideoPreviewCard> for Widget {
    fn from(card: VideoPreviewCard) -> Self {
        let (_ctx, view) = fission::build::current::<crate::state::TikTokState>();
        let tokens = &view.env().theme.tokens;
        let id = WidgetId::explicit(&format!("preview.card.{}", card.video.id));
        let accent = color_from_hex(&card.video.user.avatar_color, tokens.colors.primary);
        let mut tracks = fission::motion::fade();
        tracks.extend(fission::motion::slide_y(20.0));
        for track in tracks.iter_mut() {
            track.transition = fission::motion::MotionTransition::tween(
                220,
                fission::motion::MotionEasing::EaseOut,
            )
            .delay_ms((card.rank as u64).saturating_mul(35));
        }

        let body = Container::new(fission::widgets::ZStack {
            children: vec![
                Container::default()
                    .width(172.0)
                    .height(230.0)
                    .bg(accent)
                    .border_radius(10.0)
                    .into(),
                fission::widgets::Positioned {
                    top: Some(10.0),
                    left: Some(10.0),
                    child: Some(
                        Container::new(
                            fission::core::ui::Text::new(card.video.category.clone())
                                .size(11.0)
                                .weight(tokens.typography.font_weight_bold)
                                .color(tokens.colors.text_primary),
                        )
                        .bg(black_alpha(92))
                        .border_radius(10.0)
                        .padding([8.0, 8.0, 4.0, 4.0])
                        .into(),
                    ),
                    ..Default::default()
                }
                .into(),
                fission::widgets::Positioned {
                    bottom: Some(12.0),
                    left: Some(10.0),
                    right: Some(10.0),
                    child: Some(
                        fission::core::ui::Column {
                            children: vec![
                                fission::core::ui::Text::new(format!(
                                    "@{}",
                                    card.video.user.handle
                                ))
                                .size(12.0)
                                .weight(tokens.typography.font_weight_bold)
                                .color(tokens.colors.text_primary)
                                .into(),
                                fission::core::ui::Text::new(card.video.caption.clone())
                                    .size(12.0)
                                    .color(tokens.colors.text_primary)
                                    .max_lines(2)
                                    .into(),
                                fission::core::ui::Row {
                                    children: vec![
                                        fission::core::ui::Text::new("▶")
                                            .size(12.0)
                                            .color(tokens.colors.text_primary)
                                            .into(),
                                        fission::core::ui::Text::new(format_count(
                                            card.video.views,
                                        ))
                                        .size(12.0)
                                        .color(tokens.colors.text_primary)
                                        .into(),
                                    ],
                                    gap: Some(4.0),
                                    align_items: AlignItems::Center,
                                    ..Default::default()
                                }
                                .into(),
                            ],
                            gap: Some(4.0),
                            ..Default::default()
                        }
                        .into(),
                    ),
                    ..Default::default()
                }
                .into(),
            ],
            ..Default::default()
        })
        .width(172.0)
        .height(230.0)
        .border_radius(10.0)
        .border(white_alpha(24), 1.0);

        fission::motion::appear(id, tracks, body)
    }
}

#[fission_component]
#[derive(Clone)]
pub struct SmallVideoTile {
    pub video: VideoData,
}

impl From<SmallVideoTile> for Widget {
    fn from(tile: SmallVideoTile) -> Self {
        let (_ctx, view) = fission::build::current::<crate::state::TikTokState>();
        let tokens = &view.env().theme.tokens;
        let views_label = view
            .env()
            .i18n
            .get(&view.env().locale, "feed.views")
            .unwrap_or("views");
        let accent = color_from_hex(&tile.video.user.avatar_color, tokens.colors.primary);

        Container::new(fission::core::ui::Row {
            children: vec![
                Container::new(
                    fission::core::ui::Text::new("▶")
                        .size(14.0)
                        .color(tokens.colors.text_primary),
                )
                .width(54.0)
                .height(76.0)
                .border_radius(8.0)
                .bg(accent)
                .into(),
                fission::core::ui::Column {
                    children: vec![
                        fission::core::ui::Text::new(tile.video.caption.clone())
                            .size(13.0)
                            .weight(tokens.typography.font_weight_bold)
                            .max_lines(2)
                            .color(tokens.colors.text_primary)
                            .into(),
                        fission::core::ui::Text::new(format!(
                            "{} {}",
                            format_count(tile.video.views),
                            views_label
                        ))
                        .size(12.0)
                        .color(tokens.colors.text_secondary)
                        .into(),
                    ],
                    gap: Some(4.0),
                    ..Default::default()
                }
                .into(),
            ],
            gap: Some(10.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Start,
            ..Default::default()
        })
        .bg(tokens.colors.surface)
        .border(tokens.colors.border, 1.0)
        .border_radius(10.0)
        .padding_all(8.0)
        .into()
    }
}
