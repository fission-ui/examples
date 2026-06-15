use fission::prelude::*;
use crate::shared::state::{AppState, Page};

#[fission_component]
#[derive(Clone)]
pub struct LoginScreen {}

#[fission_reducer(GoToSignup)]
fn navigate_signup(state: &mut AppState) {
    state.current_page = Page::Signup;
}

#[fission_reducer(PerformLogin)]
fn handle_login(state: &mut AppState) {
    // In a real app, this would dispatch a login action or effect
    state.auth.is_authenticated = true;
    state.current_page = Page::Home;
}

impl From<LoginScreen> for Widget {
    fn from(_screen: LoginScreen) -> Widget {
        let (ctx, _) = fission::build::current::<AppState>();
        
        let on_signup = ctx.bind(GoToSignup, reduce!(navigate_signup));
        let on_login = ctx.bind(PerformLogin, reduce!(handle_login));

        Container::new(Column {
            gap: Some(24.0),
            children: widgets![
                Text::new("Uber Login").size(48.0),
                
                // Placeholder for Email Input
                Container::new(Text::new("Email Input Field").size(16.0))
                    .padding_all(16.0),
                    
                // Placeholder for Password Input
                Container::new(Text::new("Password Input Field").size(16.0))
                    .padding_all(16.0),
                    
                Button {
                    on_press: Some(on_login),
                    child: Some(Text::new("Log In").size(18.0).into()),
                    ..Default::default()
                },
                
                Button {
                    on_press: Some(on_signup),
                    child: Some(Text::new("Don't have an account? Sign up").size(14.0).into()),
                    ..Default::default()
                }
            ],
            ..Default::default()
        })
        .padding_all(32.0)
        .into()
    }
}
