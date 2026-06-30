use crate::style::{black_alpha, rgba, white_alpha};
use fission::op::{AlignItems, JustifyContent};
use fission::prelude::*;

#[fission_component]
#[derive(Clone)]
pub struct RecordDisc {
    pub video_id: String,
    pub avatar_color: String,
}

impl From<RecordDisc> for Widget {
    fn from(disc: RecordDisc) -> Self {
        let (_ctx, view) = fission::build::current::<crate::state::TikTokState>();
        let tokens = &view.env().theme.tokens;
        let id = WidgetId::explicit(&format!("record.disc.{}", disc.video_id));
        let accent = crate::style::color_from_hex(&disc.avatar_color, tokens.colors.primary);
        let rotation = fission::motion::MotionTrack::composite(
            fission::motion::MotionPropertyId::Rotation,
            fission::motion::MotionStartValue::Explicit(fission::motion::deg(0.0)),
            fission::motion::deg(360.0),
        )
        .transition(
            fission::motion::MotionTransition::tween(3800, fission::motion::MotionEasing::Linear)
                .repeat(true)
                .frame_interval_ms(Some(16)),
        );

        let disc_body = Container::new(fission::core::ui::Row {
            children: vec![
                Container::default()
                    .width(30.0)
                    .height(30.0)
                    .border_radius(15.0)
                    .bg(black_alpha(255))
                    .border(white_alpha(180), 2.0)
                    .into(),
                Container::default()
                    .width(8.0)
                    .height(8.0)
                    .border_radius(4.0)
                    .bg(accent)
                    .into(),
            ],
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..Default::default()
        })
        .width(54.0)
        .height(54.0)
        .border_radius(27.0)
        .bg(rgba(20, 20, 24, 255))
        .border(white_alpha(24), 1.0);

        fission::motion::appear(id, vec![rotation], disc_body)
    }
}
