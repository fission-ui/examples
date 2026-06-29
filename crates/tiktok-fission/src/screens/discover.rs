use fission::prelude::*;
use fission::op::{AlignItems, JustifyContent};
use crate::state::TikTokState;

#[fission_component]
#[derive(Clone, Default)]
pub struct DiscoverScreen {}

impl From<DiscoverScreen> for Widget {
    fn from(_: DiscoverScreen) -> Self {
        let (ctx, view) = fission::build::current::<TikTokState>();
        let tokens = &view.env().theme.tokens;

        let search_bar = Container::new(fission::core::ui::Text::new("🔍 Search"))
            .bg(tokens.colors.surface)
            .border_radius(8.0)
            .padding_all(12.0)
            .width(358.0);

        let topics = vec![
            "🔥 Trending", "#rustlang", "#coding", "#music",
            "#dance", "#food", "#travel", "#comedy",
        ];

        let topic_chips: Vec<Widget> = topics
            .iter()
            .map(|topic| {
                Container::new(fission::core::ui::Text::new(topic.to_string()))
                    .bg(tokens.colors.surface)
                    .border_radius(6.0)
                    .padding_all(10.0)
                    .into()
            })
            .collect();

        let mut grid_rows: Vec<Widget> = Vec::new();
        for chunk in topic_chips.chunks(2) {
            let mut row_children = Vec::new();
            for chip in chunk {
                row_children.push(
                    Container::new(
                        fission::core::ui::Column {
                            children: vec![
                                fission::core::ui::Text::new("📹").into(),
                                fission::core::ui::Text::new("Video Preview").into(),
                            ],
                            ..Default::default()
                        }
                    )
                    .width(175.0)
                    .height(120.0)
                    .bg(tokens.colors.surface)
                    .border_radius(8.0)
                    .into()
                );
            }
            grid_rows.push(fission::core::ui::Row { children: row_children, ..Default::default() }.into());
        }

        let mut children: Vec<Widget> = vec![
            Container::new(fission::core::ui::Text::new("Discover"))
                .padding([0.0, 0.0, 48.0, 0.0])
                .into(),
            search_bar.into(),
            fission::core::ui::Text::new("Trending Hashtags").into(),
        ];
        children.extend(grid_rows);

        Container::new(fission::core::ui::Column { children, ..Default::default() })
            .bg(tokens.colors.background)
            .flex_grow(1.0)
            .padding_all(16.0)
            .into()
    }
}
