use crate::shared::models::place::PlacePrediction;

#[derive(Debug, Clone)]
pub struct SearchQueryChanged(pub String);

#[derive(Debug, Clone)]
pub struct PlacePredictionsReceived(pub Vec<PlacePrediction>);

#[derive(Debug, Clone)]
pub struct PlaceSelected(pub PlacePrediction);

#[derive(Debug, Clone)]
pub struct ClearSearch;
