use fission::prelude::*;

#[fission_component]
#[derive(Clone)]
pub struct AppIcon {
    pub svg: &'static str,
    pub size: f32,
    pub color: fission::op::Color,
}

impl From<AppIcon> for Widget {
    fn from(icon: AppIcon) -> Self {
        fission::widgets::Icon::svg(icon.svg)
            .size(icon.size)
            .color(icon.color)
            .into()
    }
}
