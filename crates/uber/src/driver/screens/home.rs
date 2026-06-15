use fission::prelude::*;
use crate::shared::state::{AppState, Page};

#[fission_component]
#[derive(Clone)]
pub struct HomeScreen {}

#[fission_reducer(GoToProfile)]
fn navigate_profile(state: &mut AppState) {
    state.current_page = Page::Profile;
}

#[fission_reducer(ToggleOnline)]
fn toggle_online(_state: &mut AppState) {
    // In a real app, this would dispatch an action or effect to update the driver's status
}

impl From<HomeScreen> for Widget {
    fn from(_screen: HomeScreen) -> Widget {
        let (ctx, _) = fission::build::current::<AppState>();
        
        let on_profile = ctx.bind(GoToProfile, reduce!(navigate_profile));
        let on_toggle = ctx.bind(ToggleOnline, reduce!(toggle_online));

        Container::new(Column {
            gap: Some(16.0),
            children: widgets![
                // Header row
                Row {
                    gap: Some(24.0),
                    children: widgets![
                        Text::new("Uber Driver").size(32.0),
                        Button {
                            on_press: Some(on_profile),
                            child: Some(Text::new("Profile").size(14.0).into()),
                            ..Default::default()
                        }
                    ],
                    ..Default::default()
                },
                
                // Map Placeholder
                Container::new(
                    Text::new("Driver Map View Placeholder").size(24.0)
                )
                .padding_all(64.0),

                // Online/Offline Toggle
                Button {
                    on_press: Some(on_toggle),
                    child: Some(Text::new("Go Online").size(24.0).into()),
                    ..Default::default()
                }
            ],
            ..Default::default()
        })
        .padding_all(16.0)
        .into()
    }
}
