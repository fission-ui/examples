use fission::prelude::*;
use crate::shared::state::{AppState, Page};

#[fission_component]
#[derive(Clone)]
pub struct LoginScreen {}

#[fission_reducer(PerformAdminLogin)]
fn handle_login(state: &mut AppState) {
    state.auth.is_authenticated = true;
    state.current_page = Page::AdminDashboard;
}

impl From<LoginScreen> for Widget {
    fn from(_screen: LoginScreen) -> Widget {
        let (ctx, _) = fission::build::current::<AppState>();
        
        let on_login = ctx.bind(PerformAdminLogin, reduce!(handle_login));

        Container::new(Column {
            gap: Some(24.0),
            children: widgets![
                Text::new("Uber Admin Portal").size(48.0),
                
                Container::new(Text::new("Admin Email").size(16.0))
                    .padding_all(16.0),
                    
                Container::new(Text::new("Password").size(16.0))
                    .padding_all(16.0),
                    
                Button {
                    on_press: Some(on_login),
                    child: Some(Text::new("Secure Login").size(18.0).into()),
                    ..Default::default()
                }
            ],
            ..Default::default()
        })
        .padding_all(32.0)
        .into()
    }
}
