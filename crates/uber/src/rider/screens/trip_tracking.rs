use fission::prelude::*;
use crate::shared::state::{AppState, Page};

#[fission_component]
#[derive(Clone)]
pub struct TripTrackingScreen {}

#[fission_reducer(CancelTrip)]
fn cancel_trip(state: &mut AppState) {
    state.current_page = Page::Home;
}

impl From<TripTrackingScreen> for Widget {
    fn from(_screen: TripTrackingScreen) -> Widget {
        let (ctx, _) = fission::build::current::<AppState>();
        
        let on_cancel = ctx.bind(CancelTrip, reduce!(cancel_trip));

        Container::new(Column {
            gap: Some(16.0),
            children: widgets![
                Text::new("Trip in Progress").size(24.0),
                
                // Map Placeholder (Live Tracking)
                Container::new(Text::new("Live Map Tracking Placeholder").size(20.0))
                    .padding_all(64.0),

                // Driver Details
                Container::new(Column {
                    gap: Some(8.0),
                    children: widgets![
                        Text::new("Driver: John Doe").size(18.0),
                        Text::new("Vehicle: Toyota Prius (ABC-1234)").size(16.0),
                        Text::new("ETA: 5 mins").size(16.0)
                    ],
                    ..Default::default()
                })
                .padding_all(16.0),

                // Cancel Button
                Button {
                    on_press: Some(on_cancel),
                    child: Some(Text::new("Cancel Ride").size(18.0).into()),
                    ..Default::default()
                }
            ],
            ..Default::default()
        })
        .padding_all(16.0)
        .into()
    }
}
