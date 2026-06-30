use crate::state::TikTokState;
use fission::core::ui::TextContent;
use fission::op::AlignItems;
use fission::prelude::*;

#[fission_component]
#[derive(Clone)]
pub struct NavItem {
    pub target_path: &'static str,
    pub icon_svg: &'static str,
    pub label_key: &'static str,
}

impl From<NavItem> for Widget {
    fn from(item: NavItem) -> Self {
        let (ctx, view) = fission::build::current::<TikTokState>();
        let tokens = &view.env().theme.tokens;
        let active = view.state().current_path == item.target_path;
        let color = if active {
            tokens.colors.text_primary
        } else {
            tokens.colors.text_secondary
        };
        let hit_id = WidgetId::explicit(&format!("nav.item.{}.hit", item.target_path));
        let action = ctx.bind(
            crate::state::SetCurrentPath(item.target_path.to_string()),
            reduce!(crate::state::reduce_set_current_path),
        );

        let body = Container::new(fission::core::ui::Column {
            children: vec![
                crate::widgets::AppIcon {
                    svg: item.icon_svg,
                    size: 24.0,
                    color,
                }
                .into(),
                fission::core::ui::Text::new(TextContent::Key(item.label_key.into()))
                    .size(11.0)
                    .weight(if active {
                        tokens.typography.font_weight_bold
                    } else {
                        tokens.typography.font_weight_medium
                    })
                    .color(color)
                    .into(),
            ],
            gap: Some(2.0),
            align_items: AlignItems::Center,
            ..Default::default()
        })
        .width(62.0)
        .padding([4.0, 4.0, 8.0, 8.0]);

        let tappable = fission::core::ui::widgets::GestureDetector {
            id: Some(hit_id),
            on_tap: Some(action),
            child: body.into(),
            ..Default::default()
        };

        tappable.into()
    }
}
