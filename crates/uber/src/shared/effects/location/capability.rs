use crate::shared::models::location::LatLng;

pub struct LocationCapability;

impl LocationCapability {
    pub async fn request_permission() -> bool {
        true
    }
    
    pub async fn get_current_location() -> Option<LatLng> {
        None
    }
}
