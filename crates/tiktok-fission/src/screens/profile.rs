use fission::prelude::*;

use crate::state::TikTokState;
use crate::ui_helpers::{rgb, text, text_bold, empty_container};

pub fn build_profile(state: &TikTokState) -> Widget {
    let (username, avatar_initial) = if let Some(video) = state.videos.first() {
        (video.user.name.clone(), video.user.name.chars().next().unwrap_or('U'))
    } else {
        ("user".to_string(), 'U')
    };

    let bg_black = rgb(0, 0, 0);
    let text_white = rgb(255, 255, 255);
    let text_grey = rgb(170, 170, 170);
    let card_bg = rgb(25, 25, 25);
    let divider_color = rgb(51, 51, 51);
    let tiktok_red = rgb(237, 28, 82);

    let stats = Row {
        children: vec![
            build_stat("42", "Following", text_white, text_grey),
            build_stat("1.2M", "Followers", text_white, text_grey),
            build_stat("8.5M", "Likes", text_white, text_grey),
        ],
        ..Default::default()
    };

    let buttons = Row {
        children: vec![
            Container {
                child: Some(text_bold("Edit Profile", 14.0, text_white).into()),
                ..Default::default()
            }
            .bg(rgb(51, 51, 51))
            .border_radius(4.0)
            .padding([32.0, 32.0, 10.0, 10.0])
            .into(),
            Container {
                child: Some(text("📤", 16.0, text_white).into()),
                ..Default::default()
            }
            .bg(rgb(51, 51, 51))
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
                    Container {
                        child: Some(
                            Column {
                                children: vec![
                                    text("▶", 20.0, text_white).into(),
                                    text(&format!("{}", crate::state::format_count(state.videos[idx].likes)), 11.0, text_grey).into(),
                                ],
                                ..Default::default()
                            }.into()
                        ),
                        ..Default::default()
                    }
                    .width(130.0)
                    .height(160.0)
                    .bg(card_bg)
                    .into()
                );
            } else {
                row_children.push(
                    empty_container()
                        .width(130.0)
                        .height(160.0)
                        .bg(rgb(13, 13, 13))
                        .into()
                );
            }
        }
        grid_rows.push(Row { children: row_children, ..Default::default() }.into());
    }

    let mut main_children: Vec<Widget> = vec![
        empty_container().height(52.0).into(),
        text_bold(&format!("@{}", username), 18.0, text_white).into(),
        Container {
            child: Some(text_bold(&avatar_initial.to_uppercase().to_string(), 36.0, text_white).into()),
            ..Default::default()
        }
        .width(88.0)
        .height(88.0)
        .border_radius(44.0)
        .bg(tiktok_red)
        .into(),
        stats.into(),
        text("Building cool things with Rust 🦀", 13.0, text_grey).into(),
        buttons.into(),
        empty_container().height(0.5).width(390.0).bg(divider_color).into(),
        Row {
            children: vec![
                text_bold("📹 Videos", 14.0, text_white).into(),
                text("❤️ Liked", 14.0, text_grey).into(),
            ],
            ..Default::default()
        }.into(),
    ];
    main_children.extend(grid_rows);

    Container {
        child: Some(Column { children: main_children, ..Default::default() }.into()),
        ..Default::default()
    }
    .bg(bg_black)
    .width(390.0)
    .height(844.0)
    .into()
}

fn build_stat(count: &str, label: &str, white: fission::op::Color, grey: fission::op::Color) -> Widget {
    Column {
        children: vec![
            text_bold(count, 18.0, white).into(),
            text(label, 12.0, grey).into(),
        ],
        ..Default::default()
    }.into()
}
