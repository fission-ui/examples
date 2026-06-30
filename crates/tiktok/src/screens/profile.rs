use crate::state::{format_count, TikTokState};
use crate::style::{color_from_hex, screen_surface, tiktok_red, true_black, white_alpha};
use crate::widgets::{AppIcon, ProfileStat, ProfileVideoTile};
use fission::core::ui::TextContent;
use fission::op::{AlignItems, JustifyContent};
use fission::prelude::*;

#[fission_component]
#[derive(Clone, Default)]
pub struct ProfileScreen {}

impl From<ProfileScreen> for Widget {
    fn from(_: ProfileScreen) -> Self {
        let (_ctx, view) = fission::build::current::<TikTokState>();
        let state = view.state();
        let tokens = &view.env().theme.tokens;

        let creator = match state.videos.first() {
            Some(video) => video.user.clone(),
            None => {
                return Container::new(fission::core::ui::Text::new(TextContent::Key(
                    "feed.loading".into(),
                )))
                .bg(tokens.colors.background)
                .flex_grow(1.0)
                .into();
            }
        };

        let creator_videos = state.videos.clone();
        let mut grid_rows = Vec::new();
        for chunk in creator_videos.chunks(3) {
            let mut row_children = Vec::new();
            for video in chunk.iter().cloned() {
                row_children.push(ProfileVideoTile { video }.into());
            }
            grid_rows.push(
                fission::core::ui::Row {
                    children: row_children,
                    gap: Some(6.0),
                    align_items: AlignItems::Start,
                    justify_content: JustifyContent::Start,
                    ..Default::default()
                }
                .into(),
            );
        }

        let initial = creator
            .display_name
            .chars()
            .next()
            .unwrap_or('T')
            .to_uppercase()
            .to_string();

        let content = fission::core::ui::Column {
            children: vec![
                Container::default().height(48.0).into(),
                fission::core::ui::Row {
                    children: vec![
                        fission::core::ui::Text::new(format!("@{}", creator.handle))
                            .size(18.0)
                            .weight(tokens.typography.font_weight_bold)
                            .color(tokens.colors.text_primary)
                            .flex_grow(1.0)
                            .into(),
                        Container::new(AppIcon {
                            svg: fission::icons::material::navigation::menu::round(),
                            size: 24.0,
                            color: tokens.colors.text_primary,
                        })
                        .width(34.0)
                        .height(34.0)
                        .border_radius(17.0)
                        .into(),
                    ],
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::SpaceBetween,
                    ..Default::default()
                }
                .into(),
                Container::new(
                    fission::core::ui::Text::new(initial)
                        .size(34.0)
                        .weight(tokens.typography.font_weight_bold)
                        .color(tokens.colors.text_primary),
                )
                .width(92.0)
                .height(92.0)
                .border_radius(46.0)
                .bg(color_from_hex(&creator.avatar_color, tokens.colors.primary))
                .border(white_alpha(210), 3.0)
                .into(),
                fission::core::ui::Text::new(creator.display_name.clone())
                    .size(20.0)
                    .weight(tokens.typography.font_weight_bold)
                    .color(tokens.colors.text_primary)
                    .into(),
                fission::core::ui::Text::new(creator.bio.clone())
                    .size(13.0)
                    .color(tokens.colors.text_secondary)
                    .max_lines(2)
                    .into(),
                fission::core::ui::Row {
                    children: vec![
                        ProfileStat {
                            count: format_count(creator.following),
                            label_key: "profile.following",
                        }
                        .into(),
                        ProfileStat {
                            count: format_count(creator.followers),
                            label_key: "profile.followers",
                        }
                        .into(),
                        ProfileStat {
                            count: format_count(creator.total_likes),
                            label_key: "profile.likes",
                        }
                        .into(),
                    ],
                    gap: Some(28.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..Default::default()
                }
                .into(),
                fission::core::ui::Row {
                    children: vec![
                        ProfileButton {
                            label_key: "profile.edit",
                            primary: true,
                        }
                        .into(),
                        ProfileButton {
                            label_key: "profile.share",
                            primary: false,
                        }
                        .into(),
                    ],
                    gap: Some(8.0),
                    justify_content: JustifyContent::Center,
                    ..Default::default()
                }
                .into(),
                fission::core::ui::Row {
                    children: vec![
                        ProfileTab {
                            label_key: "profile.videos",
                            active: true,
                        }
                        .into(),
                        ProfileTab {
                            label_key: "profile.liked",
                            active: false,
                        }
                        .into(),
                        ProfileTab {
                            label_key: "profile.saved",
                            active: false,
                        }
                        .into(),
                    ],
                    gap: Some(24.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..Default::default()
                }
                .into(),
                fission::core::ui::Column {
                    children: grid_rows,
                    gap: Some(6.0),
                    ..Default::default()
                }
                .into(),
                Container::default().height(88.0).into(),
            ],
            gap: Some(14.0),
            align_items: AlignItems::Center,
            ..Default::default()
        };

        Container::new(Scroll {
            child: Some(
                Container::new(content)
                    .padding([16.0, 16.0, 0.0, 0.0])
                    .into(),
            ),
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
struct ProfileButton {
    label_key: &'static str,
    primary: bool,
}

impl From<ProfileButton> for Widget {
    fn from(button: ProfileButton) -> Self {
        let (_ctx, view) = fission::build::current::<TikTokState>();
        let tokens = &view.env().theme.tokens;

        Container::new(
            fission::core::ui::Text::new(TextContent::Key(button.label_key.into()))
                .size(13.0)
                .weight(tokens.typography.font_weight_bold)
                .color(if button.primary {
                    tokens.colors.text_primary
                } else {
                    tokens.colors.text_primary
                }),
        )
        .width(if button.primary { 180.0 } else { 116.0 })
        .bg(if button.primary {
            tiktok_red()
        } else {
            true_black()
        })
        .border(tokens.colors.border, 1.0)
        .border_radius(8.0)
        .padding([14.0, 14.0, 10.0, 10.0])
        .into()
    }
}

#[fission_component]
#[derive(Clone)]
struct ProfileTab {
    label_key: &'static str,
    active: bool,
}

impl From<ProfileTab> for Widget {
    fn from(tab: ProfileTab) -> Self {
        let (_ctx, view) = fission::build::current::<TikTokState>();
        let tokens = &view.env().theme.tokens;

        fission::core::ui::Column {
            children: vec![
                fission::core::ui::Text::new(TextContent::Key(tab.label_key.into()))
                    .size(13.0)
                    .weight(tokens.typography.font_weight_bold)
                    .color(if tab.active {
                        tokens.colors.text_primary
                    } else {
                        tokens.colors.text_secondary
                    })
                    .into(),
                if tab.active {
                    Container::default()
                        .height(2.0)
                        .width(42.0)
                        .border_radius(1.0)
                        .bg(tiktok_red())
                        .into()
                } else {
                    Container::default().height(2.0).into()
                },
            ],
            gap: Some(6.0),
            align_items: AlignItems::Center,
            ..Default::default()
        }
        .into()
    }
}
