//! Trip model and related types for the Uber Fission platform.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::direction::DirectionDetails;
use super::location::{GeoAddress, LatLng};
use super::vehicle::VehicleType;

/// The lifecycle status of a trip.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TripStatus {
    /// The rider has requested a trip and is waiting for a driver.
    Requested,
    /// A driver has accepted the trip request.
    Accepted,
    /// The driver is en route to the pickup location.
    DriverArriving,
    /// The driver has arrived at the pickup location.
    DriverArrived,
    /// The trip is currently in progress.
    InProgress,
    /// The trip has been completed successfully.
    Completed,
    /// The trip was cancelled by either party.
    Cancelled,
}

/// Represents a single trip/ride on the platform.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Trip {
    /// Unique identifier for the trip.
    pub id: String,
    /// ID of the rider who requested the trip.
    pub rider_id: String,
    /// ID of the assigned driver, if one has accepted.
    pub driver_id: Option<String>,
    /// Pickup location and address.
    pub pickup: GeoAddress,
    /// Drop-off location and address.
    pub dropoff: GeoAddress,
    /// The type of vehicle requested for this trip.
    pub vehicle_type: VehicleType,
    /// Calculated or final fare amount.
    pub fare_amount: f64,
    /// Optional bid fare submitted by the rider.
    pub bid_fare: Option<f64>,
    /// Current status of the trip.
    pub status: TripStatus,
    /// Route/direction details, if available.
    pub route: Option<DirectionDetails>,
    /// Timestamp when the trip was created/requested.
    pub created_at: DateTime<Utc>,
    /// Timestamp when the trip actually started (rider picked up).
    pub started_at: Option<DateTime<Utc>>,
    /// Timestamp when the trip was completed.
    pub completed_at: Option<DateTime<Utc>>,
}

/// A driver who is nearby and available to accept a trip request.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NearbyDriver {
    /// Unique identifier of the driver.
    pub driver_id: String,
    /// Display name of the driver.
    pub name: String,
    /// Current geographic location.
    pub location: LatLng,
    /// Type of vehicle the driver is operating.
    pub vehicle_type: VehicleType,
    /// Driver's average rating.
    pub rating: f64,
}
