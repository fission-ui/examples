use fission::op::AlignItems;
use fission::prelude::*;

#[fission_component]
#[derive(Clone)]
pub struct FeedActionButton {
    pub id: String,
    pub icon_svg: &'static str,
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
                crate::widgets::AppIcon {
                    svg: button.icon_svg,
                    size: 34.0,
                    color: icon_color,
                }
                .into(),
                fission::core::ui::Text::new(button.count)
                    .size(11.0)
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
