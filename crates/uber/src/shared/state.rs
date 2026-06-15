//! Root application state shared across all app variants.

use crate::shared::models::{
    user::User,
    driver::Driver,
    trip::{Trip, NearbyDriver},
    location::{LatLng, GeoAddress},
    place::PlacePrediction,
    direction::DirectionDetails,
    vehicle::VehicleType,
    payment::FareEstimate,
};

/// Authentication state.
#[derive(Debug, Clone, Default)]
pub struct AuthState {
    pub is_authenticated: bool,
    pub is_loading: bool,
    pub current_user: Option<User>,
    pub auth_token: Option<String>,
    pub error: Option<String>,
}

/// Location and map state.
#[derive(Debug, Clone, Default)]
pub struct LocationState {
    pub current_location: Option<LatLng>,
    pub selected_pickup: Option<GeoAddress>,
    pub selected_dropoff: Option<GeoAddress>,
    pub has_location_permission: bool,
}

/// Ride booking state.
#[derive(Debug, Clone, Default)]
pub struct RideState {
    pub nearby_drivers: Vec<NearbyDriver>,
    pub selected_vehicle_type: Option<VehicleType>,
    pub bid_fare: Option<f64>,
    pub fare_estimate: Option<FareEstimate>,
    pub route: Option<DirectionDetails>,
    pub is_requesting: bool,
}

/// Active trip state.
#[derive(Debug, Clone, Default)]
pub struct TripState {
    pub active_trip: Option<Trip>,
    pub driver_location: Option<LatLng>,
    pub trip_history: Vec<Trip>,
    pub is_loading_history: bool,
}

/// Search state.
#[derive(Debug, Clone, Default)]
pub struct SearchState {
    pub query: String,
    pub predictions: Vec<PlacePrediction>,
    pub is_searching: bool,
}

use serde::{Serialize, Deserialize};

/// Navigation state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Page {
    Splash,
    Login,
    Signup,
    Home,
    Search,
    RideRequest,
    TripTracking,
    Profile,
    TripHistory,
    BecomeDriver,
    Payment,
    Earnings,
    AdminDashboard,
    AdminUsers,
    AdminDrivers,
    AdminTrips,
}

impl Default for Page {
    fn default() -> Self {
        Page::Splash
    }
}

/// Root application state.
#[derive(Debug, Clone, Default)]
pub struct AppState {
    pub auth: AuthState,
    pub location: LocationState,
    pub ride: RideState,
    pub trip: TripState,
    pub search: SearchState,
    pub current_page: Page,
    pub navigation_stack: Vec<Page>,
}

impl fission::prelude::GlobalState for AppState {}
