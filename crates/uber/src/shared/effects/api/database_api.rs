use super::client::ApiClient;
use crate::shared::models::trip::Trip;

pub struct DatabaseApi<'a> {
    pub client: &'a ApiClient,
}

impl<'a> DatabaseApi<'a> {
    pub async fn get_trip(&self, _trip_id: &str) -> anyhow::Result<Trip> {
        // Firebase DB REST get implementation
        Err(anyhow::anyhow!("Not implemented"))
    }
}
