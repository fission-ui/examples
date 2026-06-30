use crate::data::SoundData;
use crate::style::{black_alpha, white_alpha};
use fission::op::{AlignItems, JustifyContent};
use fission::prelude::*;

#[fission_component]
#[derive(Clone)]
pub struct SoundPill {
    pub video_id: String,
    pub sound: SoundData,
}

impl From<SoundPill> for Widget {
    fn from(pill: SoundPill) -> Self {
        let (_ctx, view) = fission::build::current::<crate::state::TikTokState>();
        let tokens = &view.env().theme.tokens;
        let id = WidgetId::explicit(&format!("sound.pill.{}", pill.video_id));
        let mut tracks = fission::motion::slide_x(14.0);
        tracks.extend(fission::motion::fade());

        let label = format!("♪ {} - {}", pill.sound.title, pill.sound.artist);
        let child = Container::new(fission::core::ui::Row {
            children: vec![fission::core::ui::Text::new(label)
                .size(13.0)
                .color(tokens.colors.text_primary)
                .max_width(250.0)
                .into()],
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Start,
            ..Default::default()
        })
        .bg(black_alpha(82))
        .border(white_alpha(42), 1.0)
        .border_radius(14.0)
        .padding([10.0, 10.0, 5.0, 5.0]);

        fission::motion::appear(id, tracks, child)
    }
}
