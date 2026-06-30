use fission::core::ui::TextContent;
use fission::op::{AlignItems, JustifyContent};
use fission::prelude::*;

#[fission_component]
#[derive(Clone, Default)]
pub struct FeedTopBar {}

impl From<FeedTopBar> for Widget {
    fn from(_: FeedTopBar) -> Self {
        fission::core::ui::Column {
            children: vec![
                Container::default().height(48.0).into(),
                fission::core::ui::Row {
                    children: vec![
                        Container::new(
                            fission::core::ui::Text::new(TextContent::Key("feed.live".into()))
                                .size(13.0),
                        )
                        .padding([10.0, 10.0, 6.0, 6.0])
                        .into(),
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
                        Container::new(
                            fission::core::ui::Text::new(TextContent::Key("feed.search".into()))
                                .size(13.0),
                        )
                        .padding([10.0, 10.0, 6.0, 6.0])
                        .into(),
                    ],
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    gap: Some(18.0),
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
