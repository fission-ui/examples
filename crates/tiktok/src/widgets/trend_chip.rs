use crate::data::TrendData;
use crate::style::{elevated_surface, tiktok_cyan, white_alpha};
use fission::op::{AlignItems, JustifyContent};
use fission::prelude::*;

#[fission_component]
#[derive(Clone)]
pub struct TrendChip {
    pub trend: TrendData,
    pub rank: usize,
}

impl From<TrendChip> for Widget {
    fn from(chip: TrendChip) -> Self {
        let (_ctx, view) = fission::build::current::<crate::state::TikTokState>();
        let tokens = &view.env().theme.tokens;
        let id = WidgetId::explicit(&format!("trend.chip.{}", chip.trend.tag));
        let mut tracks = fission::motion::fade();
        tracks.extend(fission::motion::slide_y(12.0));
        let delay = (chip.rank as u64).saturating_mul(45);
        for track in tracks.iter_mut() {
            track.transition = fission::motion::MotionTransition::tween(
                220,
                fission::motion::MotionEasing::EaseOut,
            )
            .delay_ms(delay);
        }

        let body = Container::new(fission::core::ui::Column {
            children: vec![
                fission::core::ui::Row {
                    children: vec![
                        fission::core::ui::Text::new(chip.trend.tag.clone())
                            .size(15.0)
                            .weight(tokens.typography.font_weight_bold)
                            .color(tokens.colors.text_primary)
                            .flex_grow(1.0)
                            .into(),
                        Container::new(
                            fission::core::ui::Text::new(chip.trend.lift.clone())
                                .size(12.0)
                                .color(tiktok_cyan()),
                        )
                        .border_radius(12.0)
                        .bg(white_alpha(24))
                        .padding([8.0, 8.0, 4.0, 4.0])
                        .into(),
                    ],
                    justify_content: JustifyContent::SpaceBetween,
                    align_items: AlignItems::Center,
                    ..Default::default()
                }
                .into(),
                fission::core::ui::Text::new(chip.trend.title.clone())
                    .size(13.0)
                    .color(tokens.colors.text_primary)
                    .into(),
                fission::core::ui::Text::new(chip.trend.subtitle.clone())
                    .size(12.0)
                    .color(tokens.colors.text_secondary)
                    .max_lines(2)
                    .into(),
            ],
            gap: Some(6.0),
            ..Default::default()
        })
        .bg(elevated_surface())
        .border(tokens.colors.border, 1.0)
        .border_radius(12.0)
        .padding([14.0, 14.0, 12.0, 12.0]);

        fission::motion::appear(id, tracks, body)
    }
}
