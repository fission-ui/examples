use crate::shared::state::AppState;
use crate::shared::actions::search::*;

pub fn handle_search_query_changed(state: &mut AppState, action: SearchQueryChanged) {
    state.search.query = action.0;
    state.search.is_searching = true;
}

pub fn handle_place_predictions_received(state: &mut AppState, action: PlacePredictionsReceived) {
    state.search.predictions = action.0;
    state.search.is_searching = false;
}

pub fn handle_clear_search(state: &mut AppState, _action: ClearSearch) {
    state.search.query.clear();
    state.search.predictions.clear();
    state.search.is_searching = false;
}
