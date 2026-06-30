use crate::style::{color_from_hex, rgba, white_alpha};
use fission::op::{AlignItems, JustifyContent};
use fission::prelude::*;

#[fission_component]
#[derive(Clone)]
pub struct CreatorAvatar {
    pub handle: String,
    pub display_name: String,
    pub avatar_color: String,
    pub is_following: bool,
    pub on_follow: ActionEnvelope,
}

impl From<CreatorAvatar> for Widget {
    fn from(avatar: CreatorAvatar) -> Self {
        let (_ctx, view) = fission::build::current::<crate::state::TikTokState>();
        let tokens = &view.env().theme.tokens;
        let motion_id = WidgetId::explicit(&format!("creator.avatar.{}.motion", avatar.handle));
        let hit_id = WidgetId::explicit(&format!("creator.avatar.{}.hit", avatar.handle));
        let bg = color_from_hex(&avatar.avatar_color, tokens.colors.primary);
        let initial = avatar
            .display_name
            .chars()
            .next()
            .unwrap_or('T')
            .to_uppercase()
            .to_string();

        let follow_badge = if avatar.is_following {
            Container::new(fission::core::ui::Text::new("✓").size(12.0))
                .width(22.0)
                .height(22.0)
                .border_radius(11.0)
                .bg(tokens.colors.success)
                .border(tokens.colors.text_primary, 2.0)
                .into()
        } else {
            Container::new(
                fission::core::ui::Text::new("+")
                    .size(18.0)
                    .weight(tokens.typography.font_weight_bold),
            )
            .width(24.0)
            .height(24.0)
            .border_radius(12.0)
            .bg(tokens.colors.primary)
            .border(tokens.colors.text_primary, 2.0)
            .into()
        };

        let stack = fission::widgets::ZStack {
            children: vec![
                Container::new(fission::core::ui::Row {
                    children: vec![fission::core::ui::Text::new(initial)
                        .size(24.0)
                        .weight(tokens.typography.font_weight_bold)
                        .color(tokens.colors.text_primary)
                        .into()],
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..Default::default()
                })
                .width(56.0)
                .height(56.0)
                .border_radius(28.0)
                .bg(bg)
                .border(white_alpha(210), 2.0)
                .into(),
                fission::widgets::Positioned {
                    bottom: Some(-4.0),
                    left: Some(16.0),
                    child: Some(follow_badge),
                    ..Default::default()
                }
                .into(),
            ],
            ..Default::default()
        };

        let tappable = fission::core::ui::widgets::GestureDetector {
            id: Some(hit_id),
            on_tap: Some(avatar.on_follow),
            child: stack.into(),
            ..Default::default()
        };

        fission::motion::interactive(motion_id, fission::motion::hover_press(hit_id), tappable)
    }
}

#[fission_component]
#[derive(Clone)]
pub struct CompactAvatar {
    pub id: String,
    pub label: String,
    pub avatar_color: String,
    pub size: f32,
}

impl From<CompactAvatar> for Widget {
    fn from(avatar: CompactAvatar) -> Self {
        let (_ctx, view) = fission::build::current::<crate::state::TikTokState>();
        let tokens = &view.env().theme.tokens;
        let initial = avatar
            .label
            .chars()
            .next()
            .unwrap_or('T')
            .to_uppercase()
            .to_string();

        Container::new(fission::core::ui::Text::new(initial).color(tokens.colors.text_primary))
            .width(avatar.size)
            .height(avatar.size)
            .border_radius(avatar.size / 2.0)
            .bg(color_from_hex(&avatar.avatar_color, rgba(80, 80, 80, 255)))
            .id(WidgetId::explicit(&format!("compact.avatar.{}", avatar.id)))
    }
}
