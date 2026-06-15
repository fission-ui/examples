use serde::{Deserialize, Serialize};
use super::vehicle::VehicleType;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum PaymentMethod {
    Cash,
    Card { last_four: String },
}

impl Default for PaymentMethod {
    fn default() -> Self {
        PaymentMethod::Cash
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FareEstimate {
    pub base_fare: f64,
    pub distance_fare: f64,
    pub time_fare: f64,
    pub total: f64,
    pub currency: String,
}

impl FareEstimate {
    pub fn calculate(vehicle: &VehicleType, distance_km: f64, duration_min: f64) -> Self {
        let base = vehicle.base_fare();
        let dist = distance_km * vehicle.per_km_rate();
        let time = duration_min * vehicle.per_minute_rate();
        Self {
            base_fare: base,
            distance_fare: dist,
            time_fare: time,
            total: base + dist + time,
            currency: "USD".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PaymentInfo {
    pub method: PaymentMethod,
    pub amount: f64,
    pub currency: String,
    pub status: PaymentStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum PaymentStatus {
    Pending,
    Processing,
    Completed,
    Failed,
}
