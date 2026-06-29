use fission::prelude::*;
use fission::op::Color;

// ─── Colour constants ─────────────────────────────────────────────────────────

pub const BLACK:         Color = Color { r: 0,   g: 0,   b: 0,   a: 255 };
pub const WHITE:         Color = Color { r: 255, g: 255, b: 255, a: 255 };
pub const TRANSPARENT:   Color = Color { r: 0,   g: 0,   b: 0,   a: 0   };
pub const LIKE_RED:      Color = Color { r: 254, g: 44,  b: 85,  a: 255 };
pub const TIKTOK_CYAN:   Color = Color { r: 105, g: 201, b: 208, a: 255 };
pub const TIKTOK_RED:    Color = Color { r: 238, g: 29,  b: 82,  a: 255 };
pub const INACTIVE_GREY: Color = Color { r: 100, g: 100, b: 100, a: 255 };

pub fn rgb(r: u8, g: u8, b: u8) -> Color {
    Color { r, g, b, a: 255 }
}

pub fn rgba(r: u8, g: u8, b: u8, a: u8) -> Color {
    Color { r, g, b, a }
}

// ─── Text helpers ─────────────────────────────────────────────────────────────

pub fn text(content: impl Into<String>, size: f32, color: Color) -> Text {
    let mut t = Text::new(content.into());
    t.font_size = Some(size);
    t.color = Some(color);
    t
}

pub fn text_bold(content: impl Into<String>, size: f32, color: Color) -> Text {
    let mut t = Text::new(content.into());
    t.font_size = Some(size);
    t.color = Some(color);
    t.font_weight = Some(700);
    t
}

pub fn text_weight(content: impl Into<String>, size: f32, color: Color, weight: u16) -> Text {
    let mut t = Text::new(content.into());
    t.font_size = Some(size);
    t.color = Some(color);
    t.font_weight = Some(weight);
    t
}

// ─── Positioned helpers ───────────────────────────────────────────────────────

pub fn pos_tl(left: f32, top: f32, child: impl Into<Widget>) -> Positioned {
    Positioned {
        left: Some(left),
        top: Some(top),
        child: Some(child.into()),
        ..Default::default()
    }
}

pub fn pos_tr(right: f32, top: f32, child: impl Into<Widget>) -> Positioned {
    Positioned {
        right: Some(right),
        top: Some(top),
        child: Some(child.into()),
        ..Default::default()
    }
}

pub fn pos_bl(left: f32, bottom: f32, child: impl Into<Widget>) -> Positioned {
    Positioned {
        left: Some(left),
        bottom: Some(bottom),
        child: Some(child.into()),
        ..Default::default()
    }
}

pub fn pos_br(right: f32, bottom: f32, child: impl Into<Widget>) -> Positioned {
    Positioned {
        right: Some(right),
        bottom: Some(bottom),
        child: Some(child.into()),
        ..Default::default()
    }
}

pub fn pos_fill(child: impl Into<Widget>) -> Positioned {
    Positioned {
        left: Some(0.0),
        top: Some(0.0),
        right: Some(0.0),
        bottom: Some(0.0),
        child: Some(child.into()),
        ..Default::default()
    }
}

pub fn pos_bottom_bar(bottom: f32, child: impl Into<Widget>) -> Positioned {
    Positioned {
        left: Some(0.0),
        right: Some(0.0),
        bottom: Some(bottom),
        child: Some(child.into()),
        ..Default::default()
    }
}

pub fn pos_top_bar(top: f32, child: impl Into<Widget>) -> Positioned {
    Positioned {
        left: Some(0.0),
        right: Some(0.0),
        top: Some(top),
        child: Some(child.into()),
        ..Default::default()
    }
}

// ─── Container helpers ────────────────────────────────────────────────────────

pub fn container_bg(color: Color) -> Container {
    Container::default().bg(color)
}

pub fn empty_container() -> Container {
    Container::default()
}

// ─── Gesture wrappers ─────────────────────────────────────────────────────────

pub fn on_tap(child: impl Into<Widget>, action: ActionEnvelope) -> Widget {
    GestureDetector {
        child: child.into(),
        on_tap: Some(action),
        ..Default::default()
    }.into()
}

pub fn swipeable(child: impl Into<Widget>, drag_update: ActionEnvelope, drag_end: ActionEnvelope) -> Widget {
    GestureDetector {
        child: child.into(),
        on_drag_update: Some(drag_update),
        on_drag_end: Some(drag_end),
        ..Default::default()
    }.into()
}
