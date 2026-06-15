use crate::shared::state::AppState;
use crate::shared::actions::auth::*;

pub fn handle_login_requested(state: &mut AppState, _action: LoginRequested) {
    state.auth.is_loading = true;
    state.auth.error = None;
    // Side effect to make API call would be dispatched here
}

pub fn handle_signup_requested(state: &mut AppState, _action: SignupRequested) {
    state.auth.is_loading = true;
    state.auth.error = None;
}

pub fn handle_logout_requested(state: &mut AppState, _action: LogoutRequested) {
    state.auth.is_authenticated = false;
    state.auth.current_user = None;
    state.auth.auth_token = None;
}

pub fn handle_auth_state_changed(state: &mut AppState, action: AuthStateChanged) {
    state.auth.is_loading = false;
    if let Some(user) = action.0 {
        state.auth.is_authenticated = true;
        state.auth.current_user = Some(user);
    } else {
        state.auth.is_authenticated = false;
        state.auth.current_user = None;
    }
}

pub fn handle_auth_error(state: &mut AppState, action: AuthError) {
    state.auth.is_loading = false;
    state.auth.error = Some(action.0);
}
