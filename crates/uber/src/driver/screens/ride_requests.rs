use fission::prelude::*;
use crate::shared::state::{AppState, Page};

#[fission_component]
#[derive(Clone)]
pub struct RideRequestsScreen {}

#[fission_reducer(AcceptRide)]
fn accept_ride(state: &mut AppState) {
    state.current_page = Page::TripTracking;
}

#[fission_reducer(RejectRide)]
fn reject_ride(state: &mut AppState) {
    state.current_page = Page::Home;
}

impl From<RideRequestsScreen> for Widget {
    fn from(_screen: RideRequestsScreen) -> Widget {
        let (ctx, _) = fission::build::current::<AppState>();
        
        let on_accept = ctx.bind(AcceptRide, reduce!(accept_ride));
        let on_reject = ctx.bind(RejectRide, reduce!(reject_ride));

        Container::new(Column {
            gap: Some(16.0),
            children: widgets![
                Text::new("New Ride Request!").size(32.0),
                
                Container::new(Column {
                    gap: Some(8.0),
                    children: widgets![
                        Text::new("Pickup: 123 Main St").size(18.0),
                        Text::new("Dropoff: Airport Terminal 2").size(18.0),
                        Text::new("Est. Earnings: $14.50").size(20.0),
                        Text::new("Distance: 5.2 mi").size(16.0)
                    ],
                    ..Default::default()
                })
                .padding_all(24.0),

                Row {
                    gap: Some(16.0),
                    children: widgets![
                        Button {
                            on_press: Some(on_reject),
                            child: Some(Text::new("Decline").size(20.0).into()),
                            ..Default::default()
                        },
                        Button {
                            on_press: Some(on_accept),
                            child: Some(Text::new("Accept").size(20.0).into()),
                            ..Default::default()
                        }
                    ],
                    ..Default::default()
                }
            ],
            ..Default::default()
        })
        .padding_all(32.0)
        .into()
    }
}
