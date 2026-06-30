use crate::state::TikTokState;
use crate::style::{elevated_surface, screen_surface, tiktok_red};
use crate::widgets::{NotificationRow, SmallVideoTile};
use fission::core::ui::TextContent;
use fission::op::{AlignItems, JustifyContent};
use fission::prelude::*;

#[fission_component]
#[derive(Clone, Default)]
pub struct InboxScreen {}

impl From<InboxScreen> for Widget {
    fn from(_: InboxScreen) -> Self {
        let (_ctx, view) = fission::build::current::<TikTokState>();
        let state = view.state();
        let tokens = &view.env().theme.tokens;

        let mut notification_widgets = Vec::new();
        for notification in state.notifications.iter().cloned() {
            notification_widgets.push(NotificationRow { notification }.into());
        }

        let mut video_widgets = Vec::new();
        for video in state.videos.iter().take(3).cloned() {
            video_widgets.push(SmallVideoTile { video }.into());
        }

        let unread = state.unread_notifications();

        let content = fission::core::ui::Column {
            children: vec![
                Container::default().height(48.0).into(),
                fission::core::ui::Row {
                    children: vec![
                        fission::core::ui::Text::new(TextContent::Key("inbox.title".into()))
                            .size(28.0)
                            .weight(tokens.typography.font_weight_bold)
                            .color(tokens.colors.text_primary)
                            .flex_grow(1.0)
                            .into(),
                        Container::new(fission::core::ui::Text::new(format!("{}", unread)))
                            .width(34.0)
                            .height(34.0)
                            .border_radius(17.0)
                            .bg(tokens.colors.primary)
                            .into(),
                    ],
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::SpaceBetween,
                    ..Default::default()
                }
                .into(),
                fission::core::ui::Row {
                    children: vec![
                        InboxTab {
                            label_key: "inbox.activity",
                            active: true,
                        }
                        .into(),
                        InboxTab {
                            label_key: "inbox.messages",
                            active: false,
                        }
                        .into(),
                        InboxTab {
                            label_key: "inbox.creator_updates",
                            active: false,
                        }
                        .into(),
                    ],
                    gap: Some(8.0),
                    ..Default::default()
                }
                .into(),
                fission::core::ui::Column {
                    children: notification_widgets,
                    gap: Some(10.0),
                    ..Default::default()
                }
                .into(),
                fission::core::ui::Text::new(TextContent::Key("inbox.suggested".into()))
                    .size(18.0)
                    .weight(tokens.typography.font_weight_bold)
                    .color(tokens.colors.text_primary)
                    .into(),
                fission::core::ui::Column {
                    children: video_widgets,
                    gap: Some(10.0),
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

#[fission_component]
#[derive(Clone)]
struct InboxTab {
    label_key: &'static str,
    active: bool,
}

impl From<InboxTab> for Widget {
    fn from(tab: InboxTab) -> Self {
        let (_ctx, view) = fission::build::current::<TikTokState>();
        let tokens = &view.env().theme.tokens;

        Container::new(
            fission::core::ui::Text::new(TextContent::Key(tab.label_key.into()))
                .size(12.0)
                .weight(tokens.typography.font_weight_bold)
                .color(if tab.active {
                    tokens.colors.text_primary
                } else {
                    tokens.colors.text_secondary
                }),
        )
        .bg(if tab.active {
            tiktok_red()
        } else {
            elevated_surface()
        })
        .border(tokens.colors.border, 1.0)
        .border_radius(16.0)
        .padding([12.0, 12.0, 8.0, 8.0])
        .into()
    }
}
