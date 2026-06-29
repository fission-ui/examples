use fission::prelude::*;
use crate::ui_helpers::{rgb, text, text_bold, empty_container};

pub fn build_discover() -> Widget {
    let bg_black = rgb(0, 0, 0);
    let text_white = rgb(255, 255, 255);
    let text_grey = rgb(170, 170, 170);
    let card_bg = rgb(30, 30, 30);

    let search_bar = Container {
        child: Some(text("🔍 Search", 15.0, text_grey).into()),
        ..Default::default()
    }
    .bg(rgb(38, 38, 38))
    .border_radius(8.0)
    .padding_all(12.0)
    .width(358.0); // 390 - 32 padding

    let topics = vec![
        "🔥 Trending", "#rustlang", "#coding", "#music",
        "#dance", "#food", "#travel", "#comedy",
    ];

    let topic_chips: Vec<Widget> = topics
        .iter()
        .map(|topic| {
            Container {
                child: Some(text(*topic, 13.0, text_white).into()),
                ..Default::default()
            }
            .bg(card_bg)
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
                Container {
                    child: Some(
                        Column {
                            children: vec![
                                text("📹", 32.0, text_white).into(),
                                text("Video Preview", 11.0, text_grey).into(),
                            ],
                            ..Default::default()
                        }.into()
                    ),
                    ..Default::default()
                }
                .width(175.0)
                .height(120.0)
                .bg(card_bg)
                .border_radius(8.0)
                .into()
            );
        }
        grid_rows.push(Row { children: row_children, ..Default::default() }.into());
    }

    let mut children: Vec<Widget> = vec![
        Container {
            child: Some(text_bold("Discover", 20.0, text_white).into()),
            ..Default::default()
        }
        .padding([0.0, 0.0, 48.0, 0.0])
        .into(),
        search_bar.into(),
        text_bold("Trending Hashtags", 16.0, text_white).into(),
    ];
    children.extend(grid_rows);

    Container {
        child: Some(Column { children, ..Default::default() }.into()),
        ..Default::default()
    }
    .bg(bg_black)
    .width(390.0)
    .height(844.0)
    .padding_all(16.0)
    .into()
}
