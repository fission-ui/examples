use fission::prelude::*;
use crate::ui_helpers::{rgb, empty_container, pos_tl, text};

pub fn build_rotating_disc() -> Widget {
    let disc_bg = rgb(38, 38, 38);
    let disc_ring = rgb(64, 64, 64);
    let center_dot = rgb(255, 255, 255);

    ZStack {
        children: vec![
            empty_container().width(48.0).height(48.0).border_radius(24.0).bg(disc_bg).into(),
            pos_tl(4.0, 4.0, empty_container().width(40.0).height(40.0).border_radius(20.0).bg(disc_ring)).into(),
            pos_tl(10.0, 10.0, empty_container().width(28.0).height(28.0).border_radius(14.0).bg(disc_bg)).into(),
            pos_tl(14.0, 14.0, Container {
                child: Some(text("🎵", 10.0, center_dot).into()),
                ..Default::default()
            }.width(20.0).height(20.0).border_radius(10.0).bg(rgb(89, 89, 89))).into(),
            pos_tl(20.0, 20.0, empty_container().width(8.0).height(8.0).border_radius(4.0).bg(center_dot)).into(),
        ],
        ..Default::default()
    }.into()
}
