use crate::state::TikTokState;
use crate::style::{elevated_surface, screen_surface};
use crate::widgets::{AppIcon, TrendChip, VideoPreviewCard};
use fission::core::ui::TextContent;
use fission::op::{AlignItems, JustifyContent};
use fission::prelude::*;

#[fission_component]
#[derive(Clone, Default)]
pub struct DiscoverScreen {}

impl From<DiscoverScreen> for Widget {
    fn from(_: DiscoverScreen) -> Self {
        let (_ctx, view) = fission::build::current::<TikTokState>();
        let state = view.state();
        let tokens = &view.env().theme.tokens;

        let mut trend_widgets = Vec::new();
        for (index, trend) in state.trends.iter().cloned().enumerate() {
            trend_widgets.push(TrendChip { trend, rank: index }.into());
        }

        let mut card_rows = Vec::new();
        for (row_index, chunk) in state.videos.chunks(2).enumerate() {
            let mut row_children = Vec::new();
            for (col_index, video) in chunk.iter().cloned().enumerate() {
                row_children.push(
                    VideoPreviewCard {
                        video,
                        rank: row_index * 2 + col_index,
                    }
                    .into(),
                );
            }
            card_rows.push(
                fission::core::ui::Row {
                    children: row_children,
                    gap: Some(12.0),
                    align_items: AlignItems::Start,
                    justify_content: JustifyContent::Start,
                    ..Default::default()
                }
                .into(),
            );
        }

        let content = fission::core::ui::Column {
            children: vec![
                Container::default().height(48.0).into(),
                fission::core::ui::Row {
                    children: vec![
                        fission::core::ui::Text::new(TextContent::Key("discover.title".into()))
                            .size(28.0)
                            .weight(tokens.typography.font_weight_bold)
                            .color(tokens.colors.text_primary)
                            .flex_grow(1.0)
                            .into(),
                        Container::new(AppIcon {
                            svg: fission::icons::material::action::search::round(),
                            size: 20.0,
                            color: tokens.colors.text_primary,
                        })
                        .width(38.0)
                        .height(38.0)
                        .border_radius(19.0)
                        .bg(elevated_surface())
                        .into(),
                    ],
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::SpaceBetween,
                    ..Default::default()
                }
                .into(),
                Container::new(
                    fission::core::ui::Text::new(TextContent::Key("discover.search".into()))
                        .color(tokens.colors.text_secondary),
                )
                .bg(elevated_surface())
                .border(tokens.colors.border, 1.0)
                .border_radius(12.0)
                .padding([14.0, 14.0, 12.0, 12.0])
                .into(),
                fission::core::ui::Text::new(TextContent::Key("discover.trending_hashtags".into()))
                    .size(18.0)
                    .weight(tokens.typography.font_weight_bold)
                    .color(tokens.colors.text_primary)
                    .into(),
                fission::core::ui::Column {
                    children: trend_widgets,
                    gap: Some(10.0),
                    ..Default::default()
                }
                .into(),
                fission::core::ui::Text::new(TextContent::Key("discover.hot_now".into()))
                    .size(18.0)
                    .weight(tokens.typography.font_weight_bold)
                    .color(tokens.colors.text_primary)
                    .into(),
                fission::core::ui::Column {
                    children: card_rows,
                    gap: Some(12.0),
                    ..Default::default()
                }
                .into(),
                Container::default().height(88.0).into(),
            ],
            gap: Some(16.0),
            ..Default::default()
        };

        Container::new(Scroll {
            child: Some(Container::new(content).padding_all(16.0).into()),
            direction: FlexDirection::Column,
            show_scrollbar: false,
            flex_grow: 1.0,
            ..Default::default()
        })
        .bg(screen_surface())
        .flex_grow(1.0)
        .into()
    }
}
