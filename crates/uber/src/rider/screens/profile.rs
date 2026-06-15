use fission::prelude::*;
use crate::shared::state::{AppState, Page};

#[fission_component]
#[derive(Clone)]
pub struct ProfileScreen {}

#[fission_reducer(GoBackToHomeFromProfile)]
fn navigate_home(state: &mut AppState) {
    state.current_page = Page::Home;
}

#[fission_reducer(GoToTripHistory)]
fn navigate_trip_history(state: &mut AppState) {
    state.current_page = Page::TripHistory;
}

#[fission_reducer(GoToPayment)]
fn navigate_payment(state: &mut AppState) {
    state.current_page = Page::Payment;
}

impl From<ProfileScreen> for Widget {
    fn from(_screen: ProfileScreen) -> Widget {
        let (ctx, _) = fission::build::current::<AppState>();
        
        let on_back = ctx.bind(GoBackToHomeFromProfile, reduce!(navigate_home));
        let on_history = ctx.bind(GoToTripHistory, reduce!(navigate_trip_history));
        let on_payment = ctx.bind(GoToPayment, reduce!(navigate_payment));

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
                        Text::new("My Profile").size(24.0)
                    ],
                    ..Default::default()
                },
                
                // User Details
                Container::new(Column {
                    gap: Some(8.0),
                    children: widgets![
                        Text::new("Jane Doe").size(20.0),
                        Text::new("jane.doe@example.com").size(16.0),
                        Text::new("+1 234 567 8900").size(16.0),
                        Text::new("Rating: 4.9").size(16.0)
                    ],
                    ..Default::default()
                })
                .padding_all(24.0),

                // Menu Options
                Button {
                    on_press: Some(on_payment),
                    child: Some(Text::new("Payment Methods").size(18.0).into()),
                    ..Default::default()
                },
                Button {
                    on_press: Some(on_history),
                    child: Some(Text::new("Trip History").size(18.0).into()),
                    ..Default::default()
                }
            ],
            ..Default::default()
        })
        .padding_all(16.0)
        .into()
    }
}
