//! Places API response types for autocomplete and place details.

use serde::{Deserialize, Serialize};

use super::location::LatLng;

/// A single autocomplete prediction returned by the Places API.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlacePrediction {
    /// Unique identifier for this place (Google Place ID).
    pub place_id: String,
    /// Primary text describing the place (e.g. business name or street).
    pub main_text: String,
    /// Secondary text providing additional context (e.g. city, country).
    pub secondary_text: String,
}

/// Detailed information about a specific place.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaceDetails {
    /// Unique identifier for this place (Google Place ID).
    pub place_id: String,
    /// Human-readable name of the place.
    pub name: String,
    /// Fully formatted street address.
    pub formatted_address: String,
    /// Geographic coordinates of the place.
    pub position: LatLng,
}
