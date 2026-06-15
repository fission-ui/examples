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
    // In a real app, this would dispatch a signup action or effect
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
                Text::new("Create an Account").size(48.0),
                
                // Placeholder for Name Input
                Container::new(Text::new("Full Name Input").size(16.0))
                    .padding_all(16.0),
                    
                // Placeholder for Email Input
                Container::new(Text::new("Email Input").size(16.0))
                    .padding_all(16.0),
                    
                // Placeholder for Password Input
                Container::new(Text::new("Password Input").size(16.0))
                    .padding_all(16.0),
                    
                Button {
                    on_press: Some(on_signup),
                    child: Some(Text::new("Sign Up").size(18.0).into()),
                    ..Default::default()
                },
                
                Button {
                    on_press: Some(on_login),
                    child: Some(Text::new("Already have an account? Log in").size(14.0).into()),
                    ..Default::default()
                }
            ],
            ..Default::default()
        })
        .padding_all(32.0)
        .into()
    }
}
