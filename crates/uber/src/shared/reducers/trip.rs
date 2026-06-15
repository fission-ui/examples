use crate::shared::state::AppState;
use crate::shared::actions::trip::*;
use crate::shared::models::trip::TripStatus;

pub fn handle_trip_created(state: &mut AppState, action: TripCreated) {
    state.trip.active_trip = Some(action.0);
    state.ride.is_requesting = false;
}

pub fn handle_trip_accepted(state: &mut AppState, action: TripAccepted) {
    if let Some(trip) = &mut state.trip.active_trip {
        if trip.id == action.trip_id {
            trip.status = TripStatus::Accepted;
            trip.driver_id = Some(action.driver.id.clone());
        }
    }
}

pub fn handle_trip_status_changed(state: &mut AppState, action: TripStatusChanged) {
    if let Some(trip) = &mut state.trip.active_trip {
        if trip.id == action.trip_id {
            trip.status = action.status;
        }
    }
}

pub fn handle_driver_location_updated(state: &mut AppState, action: DriverLocationUpdated) {
    state.trip.driver_location = Some(action.0);
}

pub fn handle_trip_completed(state: &mut AppState, action: TripCompleted) {
    state.trip.active_trip = None;
    state.trip.driver_location = None;
    state.trip.trip_history.push(action.0);
}

pub fn handle_trip_cancelled(state: &mut AppState, _action: TripCancelled) {
    state.trip.active_trip = None;
    state.trip.driver_location = None;
}

pub fn handle_trip_history_loaded(state: &mut AppState, action: TripHistoryLoaded) {
    state.trip.trip_history = action.0;
    state.trip.is_loading_history = false;
}
