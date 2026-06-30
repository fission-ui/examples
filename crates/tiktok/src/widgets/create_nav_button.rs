use crate::state::TikTokState;
use crate::style::white_alpha;
use fission::op::{AlignItems, JustifyContent};
use fission::prelude::*;

#[fission_component]
#[derive(Clone, Default)]
pub struct CreateNavButton {}

impl From<CreateNavButton> for Widget {
    fn from(_: CreateNavButton) -> Self {
        let (ctx, view) = fission::build::current::<TikTokState>();
        let tokens = &view.env().theme.tokens;
        let motion_id = WidgetId::explicit("nav.create.button.motion");
        let hit_id = WidgetId::explicit("nav.create.button.hit");
        let action = ctx.bind(
            crate::state::SetCurrentPath("/create".to_string()),
            reduce!(crate::state::reduce_set_current_path),
        );

        let body = Container::new(fission::widgets::ZStack {
            children: vec![
                fission::widgets::Positioned {
                    left: Some(0.0),
                    top: Some(0.0),
                    child: Some(
                        Container::default()
                            .width(42.0)
                            .height(31.0)
                            .border_radius(9.0)
                            .bg(tokens.colors.primary)
                            .into(),
                    ),
                    ..Default::default()
                }
                .into(),
                fission::widgets::Positioned {
                    right: Some(0.0),
                    top: Some(0.0),
                    child: Some(
                        Container::default()
                            .width(42.0)
                            .height(31.0)
                            .border_radius(9.0)
                            .bg(tokens.colors.info)
                            .into(),
                    ),
                    ..Default::default()
                }
                .into(),
                fission::widgets::Positioned {
                    left: Some(5.0),
                    top: Some(0.0),
                    child: Some(
                        Container::new(fission::core::ui::Row {
                            children: vec![fission::core::ui::Text::new("+")
                                .size(22.0)
                                .weight(tokens.typography.font_weight_bold)
                                .color(tokens.colors.background)
                                .into()],
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::Center,
                            ..Default::default()
                        })
                        .width(43.0)
                        .height(31.0)
                        .border_radius(8.0)
                        .bg(tokens.colors.text_primary)
                        .border(white_alpha(160), 1.0)
                        .into(),
                    ),
                    ..Default::default()
                }
                .into(),
            ],
            ..Default::default()
        })
        .width(54.0)
        .height(31.0);

        let tappable = fission::core::ui::widgets::GestureDetector {
            id: Some(hit_id),
            on_tap: Some(action),
            child: body.into(),
            ..Default::default()
        };

        fission::motion::interactive(motion_id, fission::motion::hover_press(hit_id), tappable)
    }
}
