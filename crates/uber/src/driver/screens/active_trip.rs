use fission::prelude::*;
use crate::shared::state::{AppState, Page};

#[fission_component]
#[derive(Clone)]
pub struct ActiveTripScreen {}

#[fission_reducer(CompleteTrip)]
fn complete_trip(state: &mut AppState) {
    state.current_page = Page::Home;
}

impl From<ActiveTripScreen> for Widget {
    fn from(_screen: ActiveTripScreen) -> Widget {
        let (ctx, _) = fission::build::current::<AppState>();
        
        let on_complete = ctx.bind(CompleteTrip, reduce!(complete_trip));

        Container::new(Column {
            gap: Some(16.0),
            children: widgets![
                Text::new("Active Trip").size(24.0),
                
                // Map Placeholder (Navigation)
                Container::new(Text::new("Turn-by-turn Navigation Placeholder").size(20.0))
                    .padding_all(64.0),

                // Rider Details
                Container::new(Column {
                    gap: Some(8.0),
                    children: widgets![
                        Text::new("Rider: Jane Doe").size(18.0),
                        Text::new("Dropoff: Airport Terminal 2").size(16.0)
                    ],
                    ..Default::default()
                })
                .padding_all(16.0),

                // Complete Button
                Button {
                    on_press: Some(on_complete),
                    child: Some(Text::new("Complete Trip").size(20.0).into()),
                    ..Default::default()
                }
            ],
            ..Default::default()
        })
        .padding_all(16.0)
        .into()
    }
}
