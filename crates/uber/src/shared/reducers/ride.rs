use crate::shared::state::AppState;
use crate::shared::actions::ride::*;

pub fn handle_select_vehicle_type(state: &mut AppState, action: SelectVehicleType) {
    state.ride.selected_vehicle_type = Some(action.0);
}

pub fn handle_set_bid_fare(state: &mut AppState, action: SetBidFare) {
    state.ride.bid_fare = Some(action.0);
}

pub fn handle_request_ride(state: &mut AppState, action: RequestRide) {
    state.ride.is_requesting = true;
    state.location.selected_pickup = Some(action.pickup);
    state.location.selected_dropoff = Some(action.dropoff);
    state.ride.selected_vehicle_type = Some(action.vehicle_type);
    state.ride.bid_fare = Some(action.bid_fare);
}

pub fn handle_nearby_drivers_updated(state: &mut AppState, action: NearbyDriversUpdated) {
    state.ride.nearby_drivers = action.0;
}

pub fn handle_fare_estimate_calculated(state: &mut AppState, action: FareEstimateCalculated) {
    state.ride.fare_estimate = Some(action.0);
}

pub fn handle_ride_request_cancelled(state: &mut AppState, _action: RideRequestCancelled) {
    state.ride.is_requesting = false;
}
