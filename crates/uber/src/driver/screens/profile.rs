use fission::prelude::*;
use crate::shared::state::{AppState, Page};

#[fission_component]
#[derive(Clone)]
pub struct ProfileScreen {}

#[fission_reducer(GoBackToHomeFromProfile)]
fn navigate_home(state: &mut AppState) {
    state.current_page = Page::Home;
}

#[fission_reducer(GoToEarnings)]
fn navigate_earnings(state: &mut AppState) {
    state.current_page = Page::Earnings; // Note: Ensure Earnings is in Page enum!
}

impl From<ProfileScreen> for Widget {
    fn from(_screen: ProfileScreen) -> Widget {
        let (ctx, _) = fission::build::current::<AppState>();
        
        let on_back = ctx.bind(GoBackToHomeFromProfile, reduce!(navigate_home));
        let on_earnings = ctx.bind(GoToEarnings, reduce!(navigate_earnings));

        Container::new(Column {
            gap: Some(16.0),
            children: widgets![
                Row {
                    gap: Some(16.0),
                    children: widgets![
                        Button {
                            on_press: Some(on_back),
                            child: Some(Text::new("< Back").size(16.0).into()),
                            ..Default::default()
                        },
                        Text::new("Driver Profile").size(24.0)
                    ],
                    ..Default::default()
                },
                
                // User Details
                Container::new(Column {
                    gap: Some(8.0),
                    children: widgets![
                        Text::new("John Doe").size(20.0),
                        Text::new("Rating: 4.95").size(16.0),
                        Text::new("Total Trips: 1,204").size(16.0)
                    ],
                    ..Default::default()
                })
                .padding_all(24.0),

                // Menu Options
                Button {
                    on_press: Some(on_earnings),
                    child: Some(Text::new("Earnings").size(18.0).into()),
                    ..Default::default()
                }
            ],
            ..Default::default()
        })
        .padding_all(16.0)
        .into()
    }
}
