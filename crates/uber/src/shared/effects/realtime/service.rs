use crate::shared::models::location::LatLng;

pub struct RealtimeService;

impl RealtimeService {
    pub async fn subscribe_driver_location(_driver_id: &str) -> tokio::sync::mpsc::Receiver<LatLng> {
        let (tx, rx) = tokio::sync::mpsc::channel(100);
        let _ = tx;
        rx
    }
}
