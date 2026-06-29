use fission::prelude::*;
use fission::op::{AlignItems, JustifyContent};
use crate::state::TikTokState;

#[fission_component]
#[derive(Clone, Default)]
pub struct InboxScreen {}

impl From<InboxScreen> for Widget {
    fn from(_: InboxScreen) -> Self {
        let (ctx, view) = fission::build::current::<TikTokState>();
        let tokens = &view.env().theme.tokens;

        let notifications = vec![
            ("System", "Welcome to TikTok Fission! 🎉", "Just now"),
            ("bruno_dev", "liked your video ❤️", "2m ago"),
            ("ada_codes", "started following you", "15m ago"),
            ("mina_creates", "commented: Amazing! 🔥", "1h ago"),
            ("pixel_art_pro", "shared your video", "3h ago"),
            ("chef_maria", "liked your video ❤️", "5h ago"),
            ("travel_jake", "mentioned you in a comment", "1d ago"),
        ];

        let notification_rows: Vec<Widget> = notifications
            .into_iter()
            .map(|(user, message, time)| {
                Container::new(
                    fission::core::ui::Row {
                        children: vec![
                            Container::new(fission::core::ui::Text::new(user[..1].to_string()))
                                .width(44.0)
                                .height(44.0)
                                .border_radius(22.0)
                                .bg(tokens.colors.surface)
                                .into(),
                            fission::core::ui::Column {
                                children: vec![
                                    fission::core::ui::Row {
                                        children: vec![
                                            fission::core::ui::Text::new(user.to_string()).into(),
                                            fission::core::ui::Text::new(time.to_string()).into(),
                                        ],
                                        ..Default::default()
                                    }.into(),
                                    fission::core::ui::Text::new(message.to_string()).into(),
                                ],
                                ..Default::default()
                            }.into(),
                        ],
                        ..Default::default()
                    }
                )
                .padding_all(14.0)
                .into()
            })
            .collect();

        let mut children: Vec<Widget> = vec![
            Container::new(fission::core::ui::Text::new("Inbox"))
                .padding([16.0, 16.0, 52.0, 16.0])
                .into(),
            Container::default().height(0.5).bg(tokens.colors.border).into(),
        ];
        children.extend(notification_rows);

        Container::new(fission::core::ui::Column { children, ..Default::default() })
            .bg(tokens.colors.background)
            .flex_grow(1.0)
            .into()
    }
}
