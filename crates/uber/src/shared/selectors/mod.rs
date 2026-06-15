//! Selectors — derived view-state computations.

use crate::shared::state::AppState;
use crate::shared::models::trip::NearbyDriver;
use crate::shared::models::trip::TripStatus;

/// Returns whether the user is logged in.
pub fn is_authenticated(state: &AppState) -> bool {
    state.auth.is_authenticated
}

/// Returns the display name of the current user.
pub fn current_user_name(state: &AppState) -> String {
    state.auth.current_user
        .as_ref()
        .map(|u| u.name.clone())
        .unwrap_or_else(|| "Guest".to_string())
}

/// Returns nearby drivers filtered by selected vehicle type.
pub fn filtered_nearby_drivers(state: &AppState) -> Vec<&NearbyDriver> {
    match &state.ride.selected_vehicle_type {
        Some(vt) => state.ride.nearby_drivers.iter()
            .filter(|d| &d.vehicle_type == vt)
            .collect(),
        None => state.ride.nearby_drivers.iter().collect(),
    }
}

/// Returns true if a ride can be requested (all required data present).
pub fn can_request_ride(state: &AppState) -> bool {
    state.location.selected_pickup.is_some()
        && state.location.selected_dropoff.is_some()
        && state.ride.selected_vehicle_type.is_some()
        && !state.ride.is_requesting
}

/// Returns true if there's an active trip in progress.
pub fn has_active_trip(state: &AppState) -> bool {
    state.trip.active_trip.is_some()
}

/// Returns the current trip status display text.
pub fn trip_status_text(state: &AppState) -> &'static str {
    match state.trip.active_trip.as_ref().map(|t| &t.status) {
        Some(TripStatus::Requested) => "Finding your driver...",
        Some(TripStatus::Accepted) => "Driver assigned!",
        Some(TripStatus::DriverArriving) => "Driver is on the way",
        Some(TripStatus::DriverArrived) => "Driver has arrived",
        Some(TripStatus::InProgress) => "Trip in progress",
        Some(TripStatus::Completed) => "Trip completed",
        Some(TripStatus::Cancelled) => "Trip cancelled",
        None => "No active trip",
    }
}
