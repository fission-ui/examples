use fission::prelude::*;
use fission::widgets::Positioned;

#[fission_component]
#[derive(Clone)]
pub struct MapView {
    pub lat: f64,
    pub lng: f64,
}

impl From<MapView> for Widget {
    fn from(_view: MapView) -> Widget {
        let map_image = Image::asset("assets/map.png")
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
