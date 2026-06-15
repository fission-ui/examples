//! Driver model for the Uber Fission platform.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::location::LatLng;
use super::vehicle::VehicleType;

/// A vehicle registered to a driver on the platform.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Vehicle {
    /// Vehicle make and model (e.g. "Toyota Corolla").
    pub model: String,
    /// Exterior colour of the vehicle.
    pub color: String,
    /// License plate number.
    pub plate_number: String,
    /// Category of the vehicle.
    pub vehicle_type: VehicleType,
}

/// Represents a driver registered on the platform.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Driver {
    /// Unique identifier for the driver.
    pub id: String,
    /// Associated user account ID.
    pub user_id: String,
    /// Full name of the driver.
    pub name: String,
    /// Email address.
    pub email: String,
    /// Phone number.
    pub phone: String,
    /// The vehicle the driver is currently using.
    pub vehicle: Vehicle,
    /// Average rating (1.0–5.0).
    pub rating: f64,
    /// Total number of completed trips.
    pub total_trips: u32,
    /// Whether the driver has passed all verification checks.
    pub is_verified: bool,
    /// Whether the driver is currently online and accepting rides.
    pub is_online: bool,
    /// The driver's real-time location, if available.
    pub current_location: Option<LatLng>,
    /// Timestamp when the driver account was created.
    pub created_at: DateTime<Utc>,
}
