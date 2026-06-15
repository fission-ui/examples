use crate::shared::models::trip::{Trip, TripStatus};
use crate::shared::models::driver::Driver;
use crate::shared::models::location::LatLng;

#[derive(Debug, Clone)]
pub struct TripCreated(pub Trip);

#[derive(Debug, Clone)]
pub struct TripAccepted {
    pub trip_id: String,
    pub driver: Driver,
}

#[derive(Debug, Clone)]
pub struct TripStatusChanged {
    pub trip_id: String,
    pub status: TripStatus,
}

#[derive(Debug, Clone)]
pub struct DriverLocationUpdated(pub LatLng);

#[derive(Debug, Clone)]
pub struct TripCompleted(pub Trip);

#[derive(Debug, Clone)]
pub struct TripCancelled {
    pub trip_id: String,
    pub reason: String,
}

#[derive(Debug, Clone)]
pub struct TripHistoryLoaded(pub Vec<Trip>);
