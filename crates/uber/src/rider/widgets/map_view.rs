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
        let map_image = Image::memory(include_bytes!("../../../assets/map.png"))
            .fit(fission::op::ImageFit::Cover);
    
        // Return the image directly, but inside a container that fills its parent.
        // The parent home.rs ZStack will position it.
        Widget::from(
            fission::widgets::Container::new(map_image)
                .width(core::f32::INFINITY)
                .height(core::f32::INFINITY)
        )
    }
}
