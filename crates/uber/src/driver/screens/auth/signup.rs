use fission::prelude::*;
use crate::shared::state::{AppState, Page};

#[fission_component]
#[derive(Clone)]
pub struct SignupScreen {}

#[fission_reducer(GoToLogin)]
fn navigate_login(state: &mut AppState) {
    state.current_page = Page::Login;
}

#[fission_reducer(PerformSignup)]
fn handle_signup(state: &mut AppState) {
    state.auth.is_authenticated = true;
    state.current_page = Page::Home;
}

impl From<SignupScreen> for Widget {
    fn from(_screen: SignupScreen) -> Widget {
        let (ctx, _) = fission::build::current::<AppState>();
        
        let on_login = ctx.bind(GoToLogin, reduce!(navigate_login));
        let on_signup = ctx.bind(PerformSignup, reduce!(handle_signup));

        Container::new(Column {
            gap: Some(24.0),
            children: widgets![
                Text::new("Apply to Drive").size(48.0),
                
                Container::new(Text::new("Full Name Input").size(16.0))
                    .padding_all(16.0),
                    
                Container::new(Text::new("Email Input").size(16.0))
                    .padding_all(16.0),
                    
                Container::new(Text::new("Vehicle Details Input").size(16.0))
                    .padding_all(16.0),
                    
                Button {
                    on_press: Some(on_signup),
                    child: Some(Text::new("Sign Up").size(18.0).into()),
                    ..Default::default()
                },
                
                Button {
                    on_press: Some(on_login),
                    child: Some(Text::new("Already a driver? Log in").size(14.0).into()),
                    ..Default::default()
                }
            ],
            ..Default::default()
        })
        .padding_all(32.0)
        .into()
    }
}
