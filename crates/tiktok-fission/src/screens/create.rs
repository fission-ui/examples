use fission::prelude::*;
use fission::op::{AlignItems, JustifyContent};
use crate::state::TikTokState;

#[fission_component]
#[derive(Clone, Default)]
pub struct CreateScreen {}

impl From<CreateScreen> for Widget {
    fn from(_: CreateScreen) -> Self {
        let (ctx, view) = fission::build::current::<TikTokState>();
        let tokens = &view.env().theme.tokens;

        Container::new(
            fission::core::ui::Column {
                children: vec![
                    Container::new(fission::core::ui::Text::new("📹"))
                        .width(80.0)
                        .height(80.0)
                        .border_radius(40.0)
                        .bg(tokens.colors.primary)
                        .into(),
                    fission::core::ui::Text::new("Create").into(),
                    fission::core::ui::Text::new("Camera coming soon...").into(),
                    Container::new(fission::core::ui::Text::new("Tap to Record"))
                        .bg(tokens.colors.primary)
                        .border_radius(24.0)
                        .padding([32.0, 32.0, 12.0, 12.0])
                        .into(),
                ],
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..Default::default()
            }
        )
        .bg(tokens.colors.background)
        .flex_grow(1.0)
        .into()
    }
}
