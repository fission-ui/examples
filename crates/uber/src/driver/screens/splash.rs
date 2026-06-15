use fission::prelude::*;
use crate::shared::state::{AppState, Page};

#[fission_component]
#[derive(Clone)]
pub struct SplashScreen {}

#[fission_reducer(GoToLogin)]
fn navigate_home(state: &mut AppState) {
    state.current_page = Page::Login;
}

impl From<SplashScreen> for Widget {
    fn from(_screen: SplashScreen) -> Widget {
        let (ctx, _) = fission::build::current::<AppState>();
        
        let on_enter = ctx.bind(GoToLogin, reduce!(navigate_home));

        Container::new(Column {
            gap: Some(24.0),
            children: widgets![
                Text::new("Uber Driver").size(64.0),
                Text::new("Loading...").size(18.0),
                Button {
                    on_press: Some(on_enter),
                    child: Some(Text::new("Enter").into()),
                    ..Default::default()
                }
            ],
            ..Default::default()
        })
        .padding_all(32.0)
        .into()
    }
}
