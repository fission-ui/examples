use fission::prelude::*;
use fission::op::{AlignItems, JustifyContent};
use crate::state::TikTokState;

#[fission_component]
#[derive(Clone, Default)]
pub struct ProfileScreen {}

impl From<ProfileScreen> for Widget {
    fn from(_: ProfileScreen) -> Self {
        let (ctx, view) = fission::build::current::<TikTokState>();
        let state = view.state();
        let tokens = &view.env().theme.tokens;

        let (username, avatar_initial) = if let Some(video) = state.videos.first() {
            (video.user.name.clone(), video.user.name.chars().next().unwrap_or('U'))
        } else {
            ("user".to_string(), 'U')
        };

        let stats = fission::core::ui::Row {
            children: vec![
                build_stat("42", "Following", tokens),
                build_stat("1.2M", "Followers", tokens),
                build_stat("8.5M", "Likes", tokens),
            ],
            ..Default::default()
        };

        let buttons = fission::core::ui::Row {
            children: vec![
                Container::new(fission::core::ui::Text::new("Edit Profile"))
                    .bg(tokens.colors.surface)
                    .border_radius(4.0)
                    .padding([32.0, 32.0, 10.0, 10.0])
                    .into(),
                Container::new(fission::core::ui::Text::new("📤"))
                    .bg(tokens.colors.surface)
                    .border_radius(4.0)
                    .padding([16.0, 16.0, 10.0, 10.0])
                    .into(),
            ],
            ..Default::default()
        };

        let num_videos = state.videos.len().max(9);
        let mut grid_rows: Vec<Widget> = Vec::new();
        for row_start in (0..num_videos).step_by(3) {
            let mut row_children = Vec::new();
            for col in 0..3 {
                let idx = row_start + col;
                if idx < state.videos.len() {
                    row_children.push(
                        Container::new(
                            fission::core::ui::Column {
                                children: vec![
                                    fission::core::ui::Text::new("▶").into(),
                                    fission::core::ui::Text::new(crate::state::format_count(state.videos[idx].likes)).into(),
                                ],
                                ..Default::default()
                            }
                        )
                        .width(130.0)
                        .height(160.0)
                        .bg(tokens.colors.surface)
                        .into()
                    );
                } else {
                    row_children.push(
                        Container::default()
                            .width(130.0)
                            .height(160.0)
                            .bg(tokens.colors.surface)
                            .into()
                    );
                }
            }
            grid_rows.push(fission::core::ui::Row { children: row_children, ..Default::default() }.into());
        }

        let mut main_children: Vec<Widget> = vec![
            Container::default().height(52.0).into(),
            fission::core::ui::Text::new(format!("@{}", username)).into(),
            Container::new(fission::core::ui::Text::new(avatar_initial.to_uppercase().to_string()))
                .width(88.0)
                .height(88.0)
                .border_radius(44.0)
                .bg(tokens.colors.primary)
                .into(),
            stats.into(),
            fission::core::ui::Text::new("Building cool things with Rust 🦀").into(),
            buttons.into(),
            Container::default().height(0.5).bg(tokens.colors.border).into(),
            fission::core::ui::Row {
                children: vec![
                    fission::core::ui::Text::new("📹 Videos").into(),
                    fission::core::ui::Text::new("❤️ Liked").into(),
                ],
                ..Default::default()
            }.into(),
        ];
        main_children.extend(grid_rows);

        Container::new(fission::core::ui::Column { children: main_children, ..Default::default() })
            .bg(tokens.colors.background)
            .flex_grow(1.0)
            .into()
    }
}

fn build_stat(count: &str, label: &str, tokens: &fission::theme::Tokens) -> Widget {
    fission::core::ui::Column {
        children: vec![
            fission::core::ui::Text::new(count.to_string()).into(),
            fission::core::ui::Text::new(label.to_string()).into(),
        ],
        ..Default::default()
    }.into()
}
