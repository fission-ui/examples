use crate::shared::state::AppState;
use crate::shared::actions::location::*;

pub fn handle_location_permission_granted(state: &mut AppState, _action: LocationPermissionGranted) {
    state.location.has_location_permission = true;
}

pub fn handle_location_permission_denied(state: &mut AppState, _action: LocationPermissionDenied) {
    state.location.has_location_permission = false;
}

pub fn handle_current_location_updated(state: &mut AppState, action: CurrentLocationUpdated) {
    state.location.current_location = Some(action.0);
}

pub fn handle_pickup_location_set(state: &mut AppState, action: PickupLocationSet) {
    state.location.selected_pickup = Some(action.0);
}

pub fn handle_dropoff_location_set(state: &mut AppState, action: DropoffLocationSet) {
    state.location.selected_dropoff = Some(action.0);
}
