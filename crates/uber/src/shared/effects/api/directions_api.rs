use super::client::ApiClient;
use crate::shared::models::direction::DirectionDetails;
use crate::shared::models::location::LatLng;

pub struct DirectionsApi<'a> {
    pub client: &'a ApiClient,
}

impl<'a> DirectionsApi<'a> {
    pub async fn get_route(&self, _origin: LatLng, _destination: LatLng) -> anyhow::Result<DirectionDetails> {
        Err(anyhow::anyhow!("Not implemented"))
    }
}
