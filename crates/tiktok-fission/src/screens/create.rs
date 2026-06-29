use fission::prelude::*;
use crate::ui_helpers::{rgb, text, text_bold};

pub fn build_create() -> Widget {
    let bg_black = rgb(0, 0, 0);
    let text_white = rgb(255, 255, 255);
    let text_grey = rgb(170, 170, 170);
    let tiktok_red = rgb(237, 28, 82);

    Container {
        child: Some(
            Column {
                children: vec![
                    Container {
                        child: Some(text("📹", 36.0, text_white).into()),
                        ..Default::default()
                    }
                    .width(80.0)
                    .height(80.0)
                    .border_radius(40.0)
                    .bg(tiktok_red)
                    .into(),
                    text_bold("Create", 22.0, text_white).into(),
                    text("Camera coming soon...", 14.0, text_grey).into(),
                    Container {
                        child: Some(text_bold("Tap to Record", 15.0, text_white).into()),
                        ..Default::default()
                    }
                    .bg(tiktok_red)
                    .border_radius(24.0)
                    .padding([32.0, 32.0, 12.0, 12.0])
                    .into(),
                ],
                ..Default::default()
            }.into()
        ),
        ..Default::default()
    }
    .bg(bg_black)
    .width(390.0)
    .height(844.0)
    .into()
}
