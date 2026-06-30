use fission::core::ui::TextContent;
use fission::op::{AlignItems, JustifyContent};
use fission::prelude::*;

#[fission_component]
#[derive(Clone, Default)]
pub struct FeedTopBar {}

impl From<FeedTopBar> for Widget {
    fn from(_: FeedTopBar) -> Self {
        let (_ctx, view) = fission::build::current::<crate::state::TikTokState>();
        let tokens = &view.env().theme.tokens;

        fission::core::ui::Column {
            children: vec![
                Container::default().height(48.0).into(),
                fission::core::ui::Row {
                    children: vec![
                        fission::core::ui::Text::new(TextContent::Key("feed.live".into()))
                            .size(13.0)
                            .weight(tokens.typography.font_weight_bold)
                            .color(crate::style::white())
                            .width(52.0)
                            .into(),
                        fission::core::ui::Row {
                            children: vec![
                                crate::widgets::FeedTabLabel {
                                    label_key: "feed.following",
                                    active: false,
                                }
                                .into(),
                                crate::widgets::FeedTabLabel {
                                    label_key: "feed.for_you",
                                    active: true,
                                }
                                .into(),
                            ],
                            gap: Some(20.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            flex_grow: 1.0,
                            ..Default::default()
                        }
                        .into(),
                        Container::new(crate::widgets::AppIcon {
                            svg: fission::icons::material::action::search::round(),
                            size: 24.0,
                            color: crate::style::white(),
                        })
                        .width(52.0)
                        .padding([10.0, 10.0, 6.0, 6.0])
                        .into(),
                    ],
                    justify_content: JustifyContent::SpaceBetween,
                    align_items: AlignItems::Center,
                    ..Default::default()
                }
                .into(),
            ],
            align_items: AlignItems::Center,
            ..Default::default()
        }
        .into()
    }
}
