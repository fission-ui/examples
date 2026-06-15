use crate::shared::models::location::GeoAddress;
use crate::shared::models::vehicle::VehicleType;
use crate::shared::models::trip::NearbyDriver;
use crate::shared::models::payment::FareEstimate;

#[derive(Debug, Clone)]
pub struct SelectVehicleType(pub VehicleType);

#[derive(Debug, Clone)]
pub struct SetBidFare(pub f64);

#[derive(Debug, Clone)]
pub struct RequestRide {
    pub pickup: GeoAddress,
    pub dropoff: GeoAddress,
    pub vehicle_type: VehicleType,
    pub bid_fare: f64,
}

#[derive(Debug, Clone)]
pub struct NearbyDriversUpdated(pub Vec<NearbyDriver>);

#[derive(Debug, Clone)]
pub struct FareEstimateCalculated(pub FareEstimate);

#[derive(Debug, Clone)]
pub struct RideRequestCancelled;
