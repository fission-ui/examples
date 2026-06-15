use crate::shared::models::{
    location::LatLng as AppLatLng,
    place::PlacePrediction,
    direction::DirectionDetails,
};
use anyhow::{Result, Context};

const GOOGLE_MAPS_API_KEY: &str = "YOUR_MAPS_API_KEY";

/// Fetch autocomplete predictions using Google Places API (New).
pub async fn get_place_predictions(input: &str) -> Result<Vec<PlacePrediction>> {
    let client = google_maps::Client::try_new(GOOGLE_MAPS_API_KEY)
        .context("Failed to create Google Maps client")?;

    let response = client
        .autocomplete(input)
        .execute()
        .await
        .context("Failed to fetch autocomplete predictions")?;

    let mut predictions = Vec::new();
    for suggestion in response {
        predictions.push(PlacePrediction {
            place_id: suggestion.place_id().unwrap_or("").to_string(),
            main_text: suggestion.text().to_string(),
            secondary_text: String::new(),
        });
    }

    Ok(predictions)
}

/// Fetch directions and polyline using Google Directions API.
pub async fn get_directions(origin: &AppLatLng, destination: &AppLatLng) -> Result<DirectionDetails> {
    let client = google_maps::Client::try_new(GOOGLE_MAPS_API_KEY)
        .context("Failed to create Google Maps client")?;

    let origin_ll = google_maps::LatLng::try_from_f64(origin.lat, origin.lng)?;
    let dest_ll = google_maps::LatLng::try_from_f64(destination.lat, destination.lng)?;

    let response = client
        .directions(origin_ll, dest_ll)
        .execute()
        .await
        .context("Failed to fetch directions")?;

    // Get the first route and its first leg
    if let Some(route) = response.routes.first() {
        if let Some(leg) = route.legs.first() {
            // Overview polyline is not an Option, so we directly access points
            let polyline = route.overview_polyline.clone().points;
            return Ok(DirectionDetails {
                distance_text: leg.distance.text.clone(),
                distance_value_meters: leg.distance.value as u64,
                duration_text: leg.duration.text.clone(),
                duration_value_seconds: leg.duration.value.num_seconds() as u64,
                encoded_polyline: polyline,
            });
        }
    }

    anyhow::bail!("No routes found")
}
