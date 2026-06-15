use fission::prelude::*;
use crate::shared::state::{AppState, Page};

#[fission_component]
#[derive(Clone)]
pub struct TripsScreen {}

#[fission_reducer(GoBackToDashboardFromTrips)]
fn navigate_dashboard(state: &mut AppState) {
    state.current_page = Page::AdminDashboard;
}

impl From<TripsScreen> for Widget {
    fn from(_screen: TripsScreen) -> Widget {
        let (ctx, _) = fission::build::current::<AppState>();
        
        let on_back = ctx.bind(GoBackToDashboardFromTrips, reduce!(navigate_dashboard));

        Container::new(Column {
            gap: Some(24.0),
            children: widgets![
                Row {
                    gap: Some(16.0),
                    children: widgets![
                        Button {
                            on_press: Some(on_back),
                            child: Some(Text::new("< Back").size(16.0).into()),
                            ..Default::default()
                        },
                        Text::new("Monitor Live Trips").size(32.0)
                    ],
                    ..Default::default()
                },
                
                // Trips Map/List Placeholder
                Container::new(Column {
                    gap: Some(8.0),
                    children: widgets![
                        Text::new("Trip ID | Rider | Driver | Status").size(16.0),
                        Text::new("-------------------------------------------------").size(16.0),
                        Text::new("TRP_551 | Jane D. | Mark L. | In Progress").size(14.0),
                        Text::new("TRP_552 | Bob K.  | Sarah M. | Driver Arriving").size(14.0)
                    ],
                    ..Default::default()
                })
                .padding_all(24.0)
            ],
            ..Default::default()
        })
        .padding_all(32.0)
        .into()
    }
}
