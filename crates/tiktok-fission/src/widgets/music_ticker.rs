use fission::prelude::*;
use crate::ui_helpers::{rgb, text, empty_container};

pub fn build_music_ticker(music_name: &str) -> Widget {
    let text_white = rgb(255, 255, 255);
    let display_name = if music_name.len() > 28 {
        format!("{}...", &music_name[..25])
    } else {
        music_name.to_string()
    };

    Container {
        child: Some(text(&display_name, 13.0, text_white).into()),
        ..Default::default()
    }
    .width(180.0)
    .into()
}
