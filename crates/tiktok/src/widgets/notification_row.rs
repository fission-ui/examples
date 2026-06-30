use crate::data::NotificationData;
use crate::style::{elevated_surface, tiktok_red, white_alpha};
use fission::op::{AlignItems, JustifyContent};
use fission::prelude::*;

#[fission_component]
#[derive(Clone)]
pub struct NotificationRow {
    pub notification: NotificationData,
}

impl From<NotificationRow> for Widget {
    fn from(row: NotificationRow) -> Self {
        let (_ctx, view) = fission::build::current::<crate::state::TikTokState>();
        let tokens = &view.env().theme.tokens;
        let bg = if row.notification.unread {
            white_alpha(24)
        } else {
            elevated_surface()
        };

        Container::new(fission::core::ui::Row {
            children: vec![
                crate::widgets::CompactAvatar {
                    id: row.notification.id.clone(),
                    label: row.notification.actor.clone(),
                    avatar_color: row.notification.avatar_color.clone(),
                    size: 44.0,
                }
                .into(),
                fission::core::ui::Column {
                    children: vec![
                        fission::core::ui::Row {
                            children: vec![
                                fission::core::ui::Text::new(row.notification.actor.clone())
                                    .size(14.0)
                                    .weight(tokens.typography.font_weight_bold)
                                    .color(tokens.colors.text_primary)
                                    .into(),
                                fission::core::ui::Text::new(row.notification.age.clone())
                                    .size(12.0)
                                    .color(tokens.colors.text_muted)
                                    .into(),
                            ],
                            gap: Some(8.0),
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::Start,
                            ..Default::default()
                        }
                        .into(),
                        fission::core::ui::Text::new(row.notification.action.clone())
                            .size(13.0)
                            .color(tokens.colors.text_primary)
                            .into(),
                        fission::core::ui::Text::new(row.notification.detail.clone())
                            .size(12.0)
                            .max_lines(2)
                            .color(tokens.colors.text_secondary)
                            .into(),
                    ],
                    gap: Some(3.0),
                    flex_grow: 1.0,
                    ..Default::default()
                }
                .into(),
                if row.notification.unread {
                    Container::default()
                        .width(8.0)
                        .height(8.0)
                        .border_radius(4.0)
                        .bg(tiktok_red())
                        .into()
                } else {
                    Container::default().width(8.0).into()
                },
            ],
            gap: Some(12.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Start,
            ..Default::default()
        })
        .bg(bg)
        .border_radius(12.0)
        .padding([12.0, 12.0, 10.0, 10.0])
        .into()
    }
}
