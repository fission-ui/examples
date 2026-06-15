use fission::prelude::*;
use fission::widgets::Positioned;

#[fission_component]
#[derive(Clone)]
pub struct MapView {
    pub lat: f64,
    pub lng: f64,
}

impl From<MapView> for Widget {
    fn from(view: MapView) -> Widget {
        let map_image = Image::file("/Users/ola/.gemini/antigravity/brain/3392c4e2-5ab0-4512-968f-ab3a96f5afbc/uber_map_background_1780932349119.png")
            .fit(fission::op::ImageFit::Cover);
    
        // Wrap in a Positioned to stretch it to fill the screen
        Widget::from(Positioned {
            top: Some(0.0),
            bottom: Some(0.0),
            left: Some(0.0),
            right: Some(0.0),
            child: Some(Widget::from(map_image)),
            ..Default::default()
        })
    }
}
