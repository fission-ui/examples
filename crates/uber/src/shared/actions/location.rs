use crate::shared::models::location::{LatLng, GeoAddress};

#[derive(Debug, Clone)]
pub struct LocationPermissionGranted;

#[derive(Debug, Clone)]
pub struct LocationPermissionDenied;

#[derive(Debug, Clone)]
pub struct CurrentLocationUpdated(pub LatLng);

#[derive(Debug, Clone)]
pub struct PickupLocationSet(pub GeoAddress);

#[derive(Debug, Clone)]
pub struct DropoffLocationSet(pub GeoAddress);
