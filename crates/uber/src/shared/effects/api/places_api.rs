use super::client::ApiClient;
use crate::shared::models::place::{PlacePrediction};

pub struct PlacesApi<'a> {
    pub client: &'a ApiClient,
}

impl<'a> PlacesApi<'a> {
    pub async fn autocomplete(&self, _query: &str) -> anyhow::Result<Vec<PlacePrediction>> {
        Ok(vec![])
    }
}
