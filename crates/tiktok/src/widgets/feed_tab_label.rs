use fission::core::ui::TextContent;
use fission::op::AlignItems;
use fission::prelude::*;

#[fission_component]
#[derive(Clone)]
pub struct FeedTabLabel {
    pub label_key: &'static str,
    pub active: bool,
}

impl From<FeedTabLabel> for Widget {
    fn from(label: FeedTabLabel) -> Self {
        let (_ctx, view) = fission::build::current::<crate::state::TikTokState>();
        let tokens = &view.env().theme.tokens;
        let color = if label.active {
            crate::style::white()
        } else {
            crate::style::white_alpha(190)
        };
        let id = WidgetId::explicit(&format!("feed.tab.{}", label.label_key));

        let body = fission::core::ui::Column {
            children: vec![
                fission::core::ui::Text::new(TextContent::Key(label.label_key.into()))
                    .size(if label.active { 16.0 } else { 15.0 })
                    .color(color)
                    .weight(if label.active {
                        tokens.typography.font_weight_bold
                    } else {
                        tokens.typography.font_weight_medium
                    })
                    .into(),
                if label.active {
                    Container::default()
                        .bg(crate::style::white())
                        .width(24.0)
                        .height(2.0)
                        .border_radius(1.0)
                        .into()
                } else {
                    Container::default().height(2.0).into()
                },
            ],
            align_items: AlignItems::Center,
            gap: Some(4.0),
            ..Default::default()
        };

        fission::motion::appear(id, fission::motion::fade(), body)
    }
}
