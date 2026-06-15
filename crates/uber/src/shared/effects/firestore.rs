use crate::shared::models::trip::Trip;
use crate::shared::models::trip::NearbyDriver;
use anyhow::{Result, Context};
use reqwest::Client;
use serde::{Deserialize, Serialize};

const PROJECT_ID: &str = "YOUR_FIREBASE_PROJECT_ID";
const FIRESTORE_URL: &str = "https://firestore.googleapis.com/v1/projects";

/// Request a new ride by creating a document in the `trips` collection.
pub async fn create_trip_request(client: &Client, id_token: &str, trip: &Trip) -> Result<String> {
    let url = format!("{}/{}/databases/(default)/documents/trips?documentId={}", FIRESTORE_URL, PROJECT_ID, trip.id);
    
    // In a real implementation, we would map the `Trip` model to a Firestore REST Document layout here.
    // e.g. {"fields": {"status": {"stringValue": "Requested"}, ...}}
    let payload = serde_json::json!({
        "fields": {
            "status": { "stringValue": "Requested" },
            "riderId": { "stringValue": trip.rider_id },
            // Mapped pickup and dropoff coordinates
        }
    });

    let res = client.post(&url)
        .bearer_auth(id_token)
        .json(&payload)
        .send()
        .await?;

    if !res.status().is_success() {
        let err_text = res.text().await?;
        anyhow::bail!("Failed to create trip request: {}", err_text);
    }

    Ok(trip.id.clone())
}

/// Fetch a list of nearby drivers (Mocked for now).
pub async fn fetch_nearby_drivers(_client: &Client, _lat: f64, _lng: f64) -> Result<Vec<NearbyDriver>> {
    // In reality, this might query a geohash index in Firestore or a separate backend service.
    // Returning a mock driver for UI testing.
    Ok(vec![])
}
