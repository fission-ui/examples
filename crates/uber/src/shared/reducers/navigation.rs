use crate::shared::state::AppState;
use crate::shared::actions::navigation::*;

pub fn handle_navigate_to(state: &mut AppState, action: NavigateTo) {
    state.navigation_stack.push(state.current_page.clone());
    state.current_page = action.0;
}

pub fn handle_navigate_back(state: &mut AppState, _action: NavigateBack) {
    if let Some(prev_page) = state.navigation_stack.pop() {
        state.current_page = prev_page;
    }
}

pub fn handle_navigate_to_root(state: &mut AppState, action: NavigateToRoot) {
    state.navigation_stack.clear();
    state.current_page = action.0;
}
