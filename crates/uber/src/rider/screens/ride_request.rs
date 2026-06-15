use fission::prelude::*;
use crate::shared::state::{AppState, Page};

#[fission_component]
#[derive(Clone)]
pub struct RideRequestScreen {}

#[fission_reducer(GoBackToHome)]
fn navigate_home(state: &mut AppState) {
    state.current_page = Page::Home;
}

#[fission_reducer(ConfirmRide)]
fn confirm_ride(state: &mut AppState) {
    state.current_page = Page::TripTracking;
}

impl From<RideRequestScreen> for Widget {
    fn from(_screen: RideRequestScreen) -> Widget {
        let (ctx, _) = fission::build::current::<AppState>();
        
        let on_back = ctx.bind(GoBackToHome, reduce!(navigate_home));
        let on_confirm = ctx.bind(ConfirmRide, reduce!(confirm_ride));

        Container::new(Column {
            gap: Some(16.0),
            children: widgets![
                // Header row
                Row {
                    gap: Some(16.0),
                    children: widgets![
                        Button {
                            on_press: Some(on_back),
                            child: Some(Text::new("< Back").size(16.0).into()),
                            ..Default::default()
                        },
                        Text::new("Request a Ride").size(24.0)
                    ],
                    ..Default::default()
                },
                
                // Map Placeholder (Trip Route)
                Container::new(Text::new("Route Map Placeholder").size(20.0))
                    .padding_all(48.0),

                // Vehicle Selector
                Text::new("Choose a Ride").size(18.0),
                Container::new(Column {
                    gap: Some(8.0),
                    children: widgets![
                        Button {
                            on_press: None,
                            child: Some(Text::new("UberX - $12.50").size(16.0).into()),
                            ..Default::default()
                        },
                        Button {
                            on_press: None,
                            child: Some(Text::new("UberXL - $18.20").size(16.0).into()),
                            ..Default::default()
                        },
                        Button {
                            on_press: None,
                            child: Some(Text::new("UberBlack - $25.00").size(16.0).into()),
                            ..Default::default()
                        }
                    ],
                    ..Default::default()
                })
                .padding_all(8.0),

                // Confirm Button
                Button {
                    on_press: Some(on_confirm),
                    child: Some(Text::new("Confirm UberX").size(20.0).into()),
                    ..Default::default()
                }
            ],
            ..Default::default()
        })
        .padding_all(16.0)
        .into()
    }
}
