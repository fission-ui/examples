use crate::style::white_alpha;
use fission::op::AlignItems;
use fission::prelude::*;

#[fission_component]
#[derive(Clone)]
pub struct FeedActionButton {
    pub id: String,
    pub glyph: &'static str,
    pub count: String,
    pub active: bool,
    pub accent: fission::op::Color,
    pub on_tap: Option<ActionEnvelope>,
}

impl From<FeedActionButton> for Widget {
    fn from(button: FeedActionButton) -> Self {
        let (_ctx, view) = fission::build::current::<crate::state::TikTokState>();
        let tokens = &view.env().theme.tokens;
        let motion_id = WidgetId::explicit(&format!("feed.action.{}.motion", button.id));
        let hit_id = WidgetId::explicit(&format!("feed.action.{}.hit", button.id));
        let icon_color = if button.active {
            button.accent
        } else {
            tokens.colors.text_primary
        };

        let body = fission::core::ui::Column {
            align_items: AlignItems::Center,
            children: vec![
                Container::new(
                    fission::core::ui::Text::new(button.glyph)
                        .size(26.0)
                        .color(icon_color),
                )
                .width(48.0)
                .height(48.0)
                .border_radius(24.0)
                .bg(white_alpha(32))
                .into(),
                fission::core::ui::Text::new(button.count)
                    .size(12.0)
                    .weight(tokens.typography.font_weight_bold)
                    .color(tokens.colors.text_primary)
                    .into(),
            ],
            gap: Some(4.0),
            ..Default::default()
        };

        let child: Widget = match button.on_tap {
            Some(action) => fission::core::ui::widgets::GestureDetector {
                id: Some(hit_id),
                on_tap: Some(action),
                child: body.into(),
                ..Default::default()
            }
            .into(),
            None => body.into(),
        };

        fission::motion::interactive(motion_id, fission::motion::hover_press(hit_id), child)
    }
}
