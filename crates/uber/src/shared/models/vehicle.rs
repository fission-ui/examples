use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum VehicleType {
    Car,
    Bike,
    Rickshaw,
}

impl std::fmt::Display for VehicleType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VehicleType::Car => write!(f, "Car"),
            VehicleType::Bike => write!(f, "Bike"),
            VehicleType::Rickshaw => write!(f, "Rickshaw"),
        }
    }
}

impl Default for VehicleType {
    fn default() -> Self {
        VehicleType::Car
    }
}

impl VehicleType {
    pub fn base_fare(&self) -> f64 {
        match self {
            VehicleType::Car => 2.0,
            VehicleType::Bike => 1.0,
            VehicleType::Rickshaw => 1.5,
        }
    }

    pub fn per_km_rate(&self) -> f64 {
        match self {
            VehicleType::Car => 1.2,
            VehicleType::Bike => 0.5,
            VehicleType::Rickshaw => 0.8,
        }
    }

    pub fn per_minute_rate(&self) -> f64 {
        match self {
            VehicleType::Car => 0.2,
            VehicleType::Bike => 0.1,
            VehicleType::Rickshaw => 0.15,
        }
    }

    pub fn icon_name(&self) -> &'static str {
        match self {
            VehicleType::Car => "car-sport",
            VehicleType::Bike => "bicycle",
            VehicleType::Rickshaw => "rickshaw",
        }
    }
}
