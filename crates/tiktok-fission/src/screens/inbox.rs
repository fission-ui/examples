use fission::prelude::*;
use crate::ui_helpers::{rgb, text, text_bold, empty_container};

pub fn build_inbox() -> Widget {
    let bg_black = rgb(0, 0, 0);
    let text_white = rgb(255, 255, 255);
    let text_grey = rgb(170, 170, 170);
    let divider = rgb(51, 51, 51);

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
            Container {
                child: Some(
                    Row {
                        children: vec![
                            Container {
                                child: Some(text_bold(&user[..1], 18.0, text_white).into()),
                                ..Default::default()
                            }
                            .width(44.0)
                            .height(44.0)
                            .border_radius(22.0)
                            .bg(rgb(64, 64, 64))
                            .into(),
                            Column {
                                children: vec![
                                    Row {
                                        children: vec![
                                            text_bold(user, 14.0, text_white).into(),
                                            text(time, 12.0, text_grey).into(),
                                        ],
                                        ..Default::default()
                                    }.into(),
                                    text(message, 13.0, text_grey).into(),
                                ],
                                ..Default::default()
                            }.into(),
                        ],
                        ..Default::default()
                    }.into()
                ),
                ..Default::default()
            }
            .padding_all(14.0)
            .width(390.0)
            .into()
        })
        .collect();

    let mut children: Vec<Widget> = vec![
        Container {
            child: Some(text_bold("Inbox", 20.0, text_white).into()),
            ..Default::default()
        }
        .padding([16.0, 16.0, 52.0, 16.0])
        .width(390.0)
        .into(),
        empty_container().height(0.5).width(390.0).bg(divider).into(),
    ];
    children.extend(notification_rows);

    Container {
        child: Some(Column { children, ..Default::default() }.into()),
        ..Default::default()
    }
    .bg(bg_black)
    .width(390.0)
    .height(844.0)
    .into()
}
